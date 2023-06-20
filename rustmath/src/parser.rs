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

        let (ch, atom_type) = match tables::CharClassification::classify(ch).to_atom_type() {
            Some(atom_type) => (ch, atom_type),
            None => ('?', AtomType::Ord), // TODO: Error handling
        };

        (atom_type, Field::Symbol(Color::Normal, ch))
    }

    fn make_error_field(_text: &str) -> Field<Glyph> {
        todo!()
    }

    fn handle_command<'a>(
        cmd: &'_ str,
        remaining: &'a str,
    ) -> IResult<&'a str, (AtomType, Field<Glyph>)> {
        if let Some(ch) = tables::command_to_char(cmd) {
            return Ok((remaining, Self::handle_char(ch)));
        }

        Ok(match cmd {
            "mathop" => {
                let (remaining, (_, field)) = Self::field(remaining, false)?;
                (remaining, (AtomType::Op, field))
            }
            _ => {
                let field = Self::make_error_field(cmd);
                (remaining, (AtomType::Ord, field))
            }
        })
    }

    fn parse_command(src: &str, with_args: bool) -> IResult<&str, (AtomType, Field<Glyph>)> {
        let (src, cmd) = preceded(complete::char('\\'), complete::alpha1)(src)?;
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

        let parse_group =
            delimited(complete::char('{'), Self::parse, complete::char('}')).map(|ml| {
                let field = Field::MathList(ml);
                (AtomType::Ord, field)
            });

        let mut parser = nom::branch::alt((parse_group, parse_command, parse_char));

        parser(src)
    }

    pub fn atom(src: &str) -> IResult<&str, Atom<Glyph>> {
        let (src, (atom_type, nucleus)) = Self::field(src, true)?;

        let atom = Atom {
            atom_type,
            nucleus,
            subscript: Field::Empty,
            superscript: Field::Empty,
        };

        Ok((src, atom))
    }

    pub fn parse(src: &str) -> IResult<&str, MathList<Glyph>> {
        let mut builder = crate::mathlist::Builder::default();
        let mut src = src;
        while !src.is_empty() && !src.starts_with("}") {
            let (remaining, atom) = Self::atom(src)?;
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
