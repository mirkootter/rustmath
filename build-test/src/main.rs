use nom::IResult;

#[derive(Clone, Copy, Debug)]
pub enum Selector {
    CodePoint(char),
    Range(char, char),
}

impl Selector {
    pub fn start(&self) -> char {
        match self {
            Selector::CodePoint(ch) => *ch,
            Selector::Range(ch, _) => *ch,
        }
    }

    pub fn stop(&self) -> char {
        match self {
            Selector::CodePoint(ch) => *ch,
            Selector::Range(_, ch) => *ch,
        }
    }
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
                crate::Selector::CodePoint(ch) => self.add_char_range(ch..=ch, mathclass),
                crate::Selector::Range(a, b) => self.add_char_range(a..=b, mathclass),
            }
        }

        pub fn print_classification(&self) {
            println!(
                "pub const CHAR_CLASSIFICATION: [(u32, CharClassification); {}] = [",
                self.m.len()
            );

            for (range, &mathclass) in self.m.iter() {
                let start = *range.start();

                if start == 0 {
                    println!("    (0, CharClassification::{}),", mathclass.classify());
                } else if mathclass == super::MathClass::Ignore {
                    println!(
                        "    ('{}' as u32 + 1, CharClassification::{}),",
                        char::from_u32(start - 1).unwrap().escape_debug(),
                        mathclass.classify()
                    );
                } else {
                    println!(
                        "    ('{}' as u32, CharClassification::{}),",
                        char::from_u32(start).unwrap().escape_debug(),
                        mathclass.classify()
                    );
                }
            }

            println!("];");
        }
    }
}

fn parse_math_class_txt(d: &mut data::Data) {
    let entries = include_str!("../../rustmath/data/MathClass-15.txt")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .map(MathListEntry::parse);

    for entry in entries {
        d.add_range(entry.selector, entry.math_class);
    }
}

fn main() {
    let mut d = data::Data::new();
    parse_math_class_txt(&mut d);

    println!("use super::CharClassification;");
    println!();

    d.print_classification();
}
