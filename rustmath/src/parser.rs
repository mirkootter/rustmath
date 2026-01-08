use crate::{
    common::Color,
    mathlist::{Atom, AtomType, Delimiter, Field, MathList},
};
use nom::{
    character::complete,
    sequence::{delimited, preceded},
    Parser,
};
use std::marker::PhantomData;

use self::error::{make_recoverable_error, ErrorKind};

mod error;
mod tables;

type ParseResult<'a, T> = nom::IResult<&'a str, T, error::Error<&'a str>>;

enum Command<'a> {
    Named(&'a str),
    SingleChar(char),
}

struct ParserImp<Glyph: crate::common::Glyph> {
    m: PhantomData<Glyph>,
}

impl<Glyph: crate::common::Glyph> ParserImp<Glyph> {
    fn whitespace(src: &str) -> ParseResult<'_, ()> {
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
            None => (Field::Fallback(Color::Error), AtomType::Ord),
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
                Field::MathList(None, builder.finish(), None)
            }
        }
    }

    fn delimiter(src: &str) -> ParseResult<'_, Option<Delimiter>> {
        let (src, _) = Self::whitespace(src)?;
        if src.starts_with(".") {
            return Ok((&src[1..], None));
        }

        let (remaining, (_, field)) = Self::field(src, false)?;
        let delim = match field {
            Field::Empty => None,
            Field::Symbol(color, ch) => Some(Delimiter { ch, color }),
            _ => return make_recoverable_error(src, ErrorKind::InvalidDelimiter),
        };

        Ok((remaining, delim))
    }

    fn handle_command<'a>(
        cmd: &'_ str,
        remaining: &'a str,
    ) -> ParseResult<'a, (AtomType, Field<Glyph>)> {
        if let Some(ch) = tables::command_to_char(cmd) {
            return Ok((remaining, Self::handle_char(ch)));
        }

        Ok(match cmd {
            "\\" | " " | "%" | "$" | "_" | "^" | "{" | "}" => {
                // TODO: ' ' does not work
                let ch = cmd.chars().next().unwrap();
                (remaining, Self::handle_char(ch))
            }
            "left" => {
                let (remaining, left) = Self::delimiter(remaining)?;
                let (remaining, content) = Self::parse(remaining, Some("\\right"))?;
                let (remaining, _) = Self::whitespace(remaining)?;
                let (remaining, _) = nom::bytes::complete::tag("\\right")(remaining)?;
                let (remaining, right) = Self::delimiter(remaining)?;

                let field = Field::MathList(left, content, right);
                (remaining, (AtomType::Inner, field))
            }
            "frac" => {
                let (remaining, (_, numerator)) = Self::field(remaining, false)?;
                let (remaining, (_, denominator)) = Self::field(remaining, false)?;

                let field = Field::Fraction(numerator.into(), denominator.into());
                (remaining, (AtomType::Inner, field))
            }
            "mathop" => {
                let (remaining, (_, field)) = Self::field(remaining, false)?;
                (remaining, (AtomType::Op, field))
            }
            _ => return make_recoverable_error(remaining, ErrorKind::UnsupportedCommand),
        })
    }

    fn parse_command(src: &str, with_args: bool) -> ParseResult<'_, (AtomType, Field<Glyph>)> {
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

        let result = if with_args {
            Self::handle_command(cmd, src)
        } else {
            Self::handle_command(cmd, "").map(|(_, result)| (src, result))
        };

        result.or_else(|_| {
            let field = Self::make_error_field(&["\\", cmd]);
            Ok((src, (AtomType::Ord, field)))
        })
    }

    fn field(src: &str, with_args: bool) -> ParseResult<'_, (AtomType, Field<Glyph>)> {
        let (src, _) = Self::whitespace(src)?;

        let parse_command = |src| Self::parse_command(src, with_args);
        let parse_char = complete::anychar.map(Self::handle_char);
        let parse_broken_char = complete::one_of("{}").map(|ch| {
            let mut buf = [0; 4];
            (
                AtomType::Ord,
                Self::make_error_field(&[ch.encode_utf8(&mut buf)]),
            )
        });
        let parse_group = delimited(
            complete::char('{'),
            |src| Self::parse(src, Some("}")),
            complete::char('}'),
        )
        .map(|ml| {
            let field = Field::MathList(None, ml, None);
            (AtomType::Ord, field)
        });

        let mut parser =
            nom::branch::alt((parse_group, parse_command, parse_broken_char, parse_char));

        parser(src)
    }

    pub fn atom(src: &str) -> ParseResult<'_, Atom<Glyph>> {
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

    pub fn parse<'a>(
        src: &'a str,
        expected_stop: Option<&'_ str>,
    ) -> ParseResult<'a, MathList<Glyph>> {
        let mut builder = crate::mathlist::Builder::default();
        let mut src = src;

        loop {
            let (remaining, _) = Self::whitespace(src)?;
            if remaining.is_empty() {
                break;
            }
            if let Some(expected_stop) = expected_stop {
                if remaining.starts_with(expected_stop) {
                    break;
                }
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
    ParserImp::parse(src, None).ok().map(|r| r.1)
}
