use nom::IResult;

#[derive(Clone, Copy, Debug)]
pub enum Selector {
    CodePoint(char),
    Range(char, char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathClass {
    Normal,
    Alphabetic,
    Binary,
    Closing,
    Diacritic,
    Fence,
    GlyphPart,
    Large,
    Opening,
    Punct,
    Relation,
    Space,
    Unary,
    Vary,
    Special,
    Ignore,
}

impl MathClass {
    pub fn parse(ch: char) -> Self {
        match ch {
            'N' => Self::Normal,
            'A' => Self::Alphabetic,
            'B' => Self::Binary,
            'C' => Self::Closing,
            'D' => Self::Diacritic,
            'F' => Self::Fence,
            'G' => Self::GlyphPart,
            'L' => Self::Large,
            'O' => Self::Opening,
            'P' => Self::Punct,
            'R' => Self::Relation,
            'S' => Self::Space,
            'U' => Self::Unary,
            'V' => Self::Vary,
            'X' => Self::Special,
            _ => unreachable!(),
        }
    }

    pub fn from_command(cmd: &str) -> Self {
        match cmd {
            "mathclose" => Self::Closing,
            "mathord" => Self::Normal,
            "mathpunct" => Self::Punct,
            "mathrel" => Self::Relation,
            "mathopen" => Self::Opening,
            "mathfence" => Self::Fence,
            "mathalpha" => Self::Alphabetic,
            "mathaccent" => Self::Diacritic,
            "mathaccentwide" => Self::Diacritic,
            "mathbotaccent" => Self::Diacritic,
            "mathbotaccentwide" => Self::Diacritic,
            "mathaccentoverlay" => Self::Diacritic,
            "mathbin" => Self::Binary,
            "mathop" => Self::Large,
            "mathover" | "mathunder" => Self::Diacritic,
            _ => {
                eprintln!("oje {:?}", cmd);
                unreachable!()
            }
        }
    }

    pub fn classify(self) -> &'static str {
        match self {
            MathClass::Normal => "Normal",
            MathClass::Alphabetic => "Alphabetic",
            MathClass::Binary => "Binary",
            MathClass::Closing => "Closing",
            MathClass::Diacritic => "Diacritic",
            MathClass::Fence => "Fence",
            MathClass::GlyphPart => "GlyphPart",
            MathClass::Large => "Large",
            MathClass::Opening => "Opening",
            MathClass::Punct => "Punct",
            MathClass::Relation => "Relation",
            MathClass::Space => "Space",
            MathClass::Unary => "Unary",
            MathClass::Vary => "Vary",
            MathClass::Special => "Special",
            MathClass::Ignore => "Ignore",
        }
    }
}

#[derive(Debug)]
struct MathListEntry {
    selector: Selector,
    math_class: MathClass,
}

impl MathListEntry {
    pub fn parse(src: &str) -> Self {
        let parse_code_point = || {
            nom::combinator::map(nom::character::complete::hex_digit1, |s| {
                u32::from_str_radix(s, 16)
                    .map(char::from_u32)
                    .unwrap()
                    .unwrap()
            })
        };
        let parse_range = nom::combinator::map(
            nom::sequence::tuple((
                parse_code_point(),
                nom::bytes::complete::tag(".."),
                parse_code_point(),
            )),
            |(start, _, stop)| Selector::Range(start, stop),
        );

        let parse_selector = nom::branch::alt((
            parse_range,
            nom::combinator::map(parse_code_point(), Selector::CodePoint),
        ));

        let parse_math_class =
            nom::combinator::map(nom::character::complete::anychar, MathClass::parse);

        let mut parser = nom::sequence::tuple((
            parse_selector,
            nom::bytes::complete::tag(";"),
            parse_math_class,
        ));

        let parse_result: IResult<_, _> = parser(src);
        let (selector, _, math_class) = parse_result.unwrap().1;

        Self {
            selector,
            math_class,
        }
    }
}

struct MathTableRow<'a> {
    ch: char,
    command: &'a str,
    math_class: MathClass,
    #[allow(dead_code)]
    descr: &'a str,
}

impl<'a> MathTableRow<'a> {
    pub fn parse(src: &'a str) -> Self {
        let parse_code_point = nom::combinator::map(nom::character::complete::hex_digit1, |s| {
            u32::from_str_radix(s, 16)
                .map(char::from_u32)
                .unwrap()
                .unwrap()
        });

        let parse_code_point = nom::sequence::delimited(
            nom::bytes::complete::tag("{\""),
            parse_code_point,
            nom::bytes::complete::tag("}"),
        );

        let parse_command = || {
            nom::sequence::delimited(
                nom::bytes::complete::tag("\\"),
                nom::character::complete::alpha1,
                nom::character::complete::space0,
            )
        };

        let parse_classify =
            nom::combinator::map(parse_command(), |cmd| MathClass::from_command(cmd));

        let parse_main_command = nom::sequence::delimited(
            nom::bytes::complete::tag("{"),
            parse_command(),
            nom::bytes::complete::tag("}"),
        );

        let parse_classify = nom::sequence::delimited(
            nom::bytes::complete::tag("{"),
            parse_classify,
            nom::bytes::complete::tag("}"),
        );

        let parse_text = nom::sequence::delimited(
            nom::bytes::complete::tag("{"),
            nom::bytes::complete::take_while1(|ch| ch != '}'),
            nom::bytes::complete::tag("}"),
        );

        let mut parser = nom::sequence::tuple((
            nom::bytes::complete::tag("\\UnicodeMathSymbol"),
            parse_code_point,
            parse_main_command,
            parse_classify,
            parse_text,
        ));

        let parse_result: IResult<_, _> = parser(src);
        let (_, ch, command, math_class, descr) = parse_result.unwrap().1;

        Self {
            ch,
            command,
            math_class,
            descr,
        }
    }
}

mod data {
    use std::collections::BTreeMap;

    pub struct Data {
        m: rangemap::RangeInclusiveMap<u32, super::MathClass>,
        commands: BTreeMap<String, char>,
    }

    impl Data {
        pub fn new() -> Self {
            let mut result = Data {
                m: Default::default(),
                commands: Default::default(),
            };
            result.m.insert(0..=u32::MAX, super::MathClass::Ignore);
            result
        }

        fn add_char_range(
            &mut self,
            range: std::ops::RangeInclusive<char>,
            mathclass: super::MathClass,
        ) {
            let start = *range.start() as u32;
            let end = *range.end() as u32;
            self.m.insert(start..=end, mathclass);
        }

        pub fn add_range(&mut self, sel: super::Selector, mathclass: super::MathClass) {
            match sel {
                super::Selector::CodePoint(ch) => self.add_char_range(ch..=ch, mathclass),
                super::Selector::Range(a, b) => self.add_char_range(a..=b, mathclass),
            }
        }

        pub fn add_command(&mut self, cmd: String, ch: char, math_class: super::MathClass) {
            self.commands.insert(cmd, ch);

            let ch = ch as u32;
            self.m.insert(ch..=ch, math_class);
        }

        pub fn print_classification(&self, out: &mut impl core::fmt::Write) -> core::fmt::Result {
            writeln!(
                out,
                "pub static CHAR_CLASSIFICATION: [(u32, CharClassification); {}] = [",
                self.m.len()
            )?;

            for (range, &mathclass) in self.m.iter() {
                let start = *range.start();

                if start == 0 {
                    writeln!(
                        out,
                        "    (0, CharClassification::{}),",
                        mathclass.classify()
                    )?;
                } else if mathclass == super::MathClass::Ignore {
                    writeln!(
                        out,
                        "    ('{}' as u32 + 1, CharClassification::{}),",
                        char::from_u32(start - 1).unwrap().escape_debug(),
                        mathclass.classify()
                    )?;
                } else {
                    writeln!(
                        out,
                        "    ('{}' as u32, CharClassification::{}),",
                        char::from_u32(start).unwrap().escape_debug(),
                        mathclass.classify()
                    )?;
                }
            }

            writeln!(out, "];")?;

            Ok(())
        }

        pub fn print_commands(&self, out: &mut impl core::fmt::Write) -> core::fmt::Result {
            writeln!(
                out,
                "pub static CHAR_COMMANDS: [(&'static str, char); {}] = [",
                self.commands.len()
            )?;
            for (cmd, ch) in &self.commands {
                writeln!(
                    out,
                    "    (\"{}\", '{}'),",
                    cmd.escape_debug(),
                    ch.escape_debug()
                )?;
            }
            writeln!(out, "];")
        }
    }
}

fn parse_math_class_txt(d: &mut data::Data) {
    let entries = include_str!("../../../rustmath/data/MathClass-15.txt")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .map(MathListEntry::parse);

    for entry in entries {
        d.add_range(entry.selector, entry.math_class);
    }
}

fn parse_math_table_tex(d: &mut data::Data) {
    let entries = include_str!("../../data/unicode-math-table.tex")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("%"))
        .map(MathTableRow::parse);

    for row in entries {
        d.add_command(row.command.to_owned(), row.ch, row.math_class);
    }
}

fn generate() -> Result<String, core::fmt::Error> {
    use core::fmt::Write;

    let mut d = data::Data::new();
    parse_math_class_txt(&mut d);
    parse_math_table_tex(&mut d);

    let mut generated = String::new();

    writeln!(&mut generated, "use super::CharClassification;")?;
    writeln!(&mut generated)?;

    d.print_classification(&mut generated)?;
    writeln!(&mut generated)?;
    d.print_commands(&mut generated)?;

    Ok(generated)
}

#[test]
fn test_generated_sources() {
    let generated = generate().unwrap();
    let found = include_str!("../parser/tables/generated.rs");
    assert_eq!(generated, found);
}
