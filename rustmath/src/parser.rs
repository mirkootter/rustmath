use crate::{
    common::Color,
    mathlist::{Atom, AtomType, Field, MathList},
};
use nom::{
    character::complete,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use std::marker::PhantomData;
mod tables;

enum Command<'a> {
    Named(&'a str),
    SingleChar(char),
}

struct ParserImp<Glyph: crate::common::Glyph> {
    m: PhantomData<Glyph>,
}

impl<Glyph: crate::common::Glyph> ParserImp<Glyph> {
    fn whitespace(src: &str) -> IResult<&str, ()> {
        use nom::InputTakeAtPosition;
        let (src, _) = src.split_at_position_complete(|ch| !ch.is_whitespace())?;
        Ok((src, ()))
    }

    fn handle_char(ch: char) -> (AtomType, Field<Glyph>) {
        // HACK: Map chars to their math equivalent
        let ch = match ch {
            '-' => '−',
            '*' => '∗',
            'h' => 'ℎ',
            'A'..='Z' => char::from_u32(ch as u32 + '𝐴' as u32 - 'A' as u32).unwrap(),
            'a'..='z' => char::from_u32(ch as u32 + '𝑎' as u32 - 'a' as u32).unwrap(),
            _ => ch,
        };

        let (symbol, atom_type) = match tables::CharClassification::classify(ch).to_atom_type() {
            Some(atom_type) => (Field::Symbol(Color::Normal, ch), atom_type),
            None => (Field::Symbol(Color::Error, '?'), AtomType::Ord), // TODO: Default-Char for the font
        };

        (atom_type, symbol)
    }

    fn make_error_field(text: &[&str]) -> Field<Glyph> {
        let len = text.into_iter().map(|x| x.len()).sum();
        match len {
            0 => Field::Empty,
            1 => Field::Symbol(Color::Error, text[0].chars().next().unwrap()),
            _ => {
                let mut builder = crate::mathlist::Builder::default();
                for text in text {
                    for ch in text.chars() {
                        builder.add_symbol(ch, Color::Error);
                    }
                }
                Field::MathList(builder.finish())
            }
        }
    }

    fn handle_command<'a>(
        cmd: &'_ str,
        remaining: &'a str,
    ) -> IResult<&'a str, (AtomType, Field<Glyph>)> {
        if let Some(ch) = tables::command_to_char(cmd) {
            return Ok((remaining, Self::handle_char(ch)));
        }

        Ok(match cmd {
            "\\" | " " | "%" | "$" | "_" | "^" | "{" | "}" => {
                // TODO: ' ' does not work
                let ch = cmd.chars().next().unwrap();
                (remaining, Self::handle_char(ch))
            }
            "mathop" => {
                let (remaining, (_, field)) = Self::field(remaining, false)?;
                (remaining, (AtomType::Op, field))
            }
            _ => {
                let field = Self::make_error_field(&["\\", cmd]);
                (remaining, (AtomType::Ord, field))
            }
        })
    }

    fn parse_command(src: &str, with_args: bool) -> IResult<&str, (AtomType, Field<Glyph>)> {
        let (src, cmd) = preceded(
            complete::char('\\'),
            nom::branch::alt((
                complete::alpha1.map(Command::Named),
                complete::anychar.map(Command::SingleChar),
            )),
        )(src)?;

        let mut buf = [0; 4];
        let cmd = match cmd {
            Command::Named(cmd) => cmd,
            Command::SingleChar(ch) => ch.encode_utf8(&mut buf),
        };

        if with_args {
            Self::handle_command(cmd, src)
        } else {
            let (_, result) = Self::handle_command(cmd, "")?;
            Ok((src, result))
        }
    }

    fn field(src: &str, with_args: bool) -> IResult<&str, (AtomType, Field<Glyph>)> {
        let (src, _) = Self::whitespace(src)?;

        let parse_command = |src| Self::parse_command(src, with_args);
        let parse_char = complete::none_of("}").map(Self::handle_char);
        let parse_broken_char =
            nom::bytes::complete::tag("{").map(|_| (AtomType::Ord, Self::make_error_field(&["{"])));
        let parse_group =
            delimited(complete::char('{'), Self::parse, complete::char('}')).map(|ml| {
                let field = Field::MathList(ml);
                (AtomType::Ord, field)
            });

        let mut parser =
            nom::branch::alt((parse_group, parse_command, parse_broken_char, parse_char));

        parser(src)
    }

    pub fn atom(src: &str) -> IResult<&str, Atom<Glyph>> {
        let (src, (atom_type, nucleus)) = Self::field(src, true)?;
        let (mut src, _) = Self::whitespace(src)?;

        let mut subscript = Field::Empty;
        let mut superscript = Field::Empty;

        let mut parse_subscript = preceded(nom::bytes::complete::tag("_"), |src| {
            Self::field(src, false)
        });

        let mut parse_superscript = preceded(nom::bytes::complete::tag("^"), |src| {
            Self::field(src, false)
        });

        loop {
            let mut done_something = false;
            if let Ok((remaining, script)) = parse_subscript(src) {
                // TODO: Error in case of double subscript
                subscript = script.1;
                src = remaining;
                done_something = true;
            }
            if let Ok((remaining, script)) = parse_superscript(src) {
                // TODO: Error in case of double superscript
                superscript = script.1;
                src = remaining;
                done_something = true;
            }

            if !done_something {
                break;
            }
        }

        let atom = Atom {
            atom_type,
            nucleus,
            subscript,
            superscript,
        };

        Ok((src, atom))
    }

    pub fn parse(src: &str) -> IResult<&str, MathList<Glyph>> {
        let mut builder = crate::mathlist::Builder::default();
        let mut src = src;

        loop {
            let (remaining, _) = Self::whitespace(src)?;
            if remaining.is_empty() || remaining.starts_with("}") {
                break;
            }

            let (remaining, atom) = Self::atom(remaining)?;
            builder.add_atom(atom);

            src = remaining;
        }

        let ml = builder.finish();
        Ok((src, ml))
    }
}

pub fn parse<G: crate::common::Glyph>(src: &str) -> Option<MathList<G>> {
    ParserImp::parse(src).ok().map(|r| r.1)
}
