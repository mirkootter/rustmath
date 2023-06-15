use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Selector {
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

#[derive(Debug, Clone, Copy)]
enum MathClass {
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

#[derive(Default)]
struct CharacterClassfication(Vec<String>);

impl CharacterClassfication {
    pub fn add_ignore(&mut self, last: char) {
        self.0.push(format!(
            "    ('{}' as u32 + 1, CharClassification::Ignore),",
            last.escape_debug()
        ));
    }

    pub fn add_char(&mut self, ch: char, classfication: &str) {
        self.0.push(format!(
            "    ('{}' as u32, CharClassification::{}),",
            ch.escape_debug(),
            classfication
        ));
    }

    pub fn print(&self) {
        println!("use super::CharClassification;");
        println!();

        println!(
            "pub const CHAR_CLASSIFICATION: [(u32, CharClassification); {}] = [",
            self.0.len() + 1
        );
        println!("    (0, CharClassification::Ignore),");
        for line in &self.0 {
            println!("{}", line);
        }
        println!("];");
    }
}

fn main() {
    let mut entries = include_str!("../../rustmath/data/MathClass-15.txt")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .map(MathListEntry::parse);

    let mut last = None::<char>;
    let mut cc = CharacterClassfication::default();
    while let Some(entry) = entries.next() {
        let start = entry.selector.start();
        if let Some(last) = last {
            assert!(start >= last);
            if start as u32 > last as u32 + 1 {
                cc.add_ignore(last);
            }
        }

        cc.add_char(start, entry.math_class.classify());
        last = Some(entry.selector.stop());
    }
    cc.print();
}
