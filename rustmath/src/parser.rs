use nom::{
    bytes::complete::take_while1,
    character::complete,
    multi::many0,
    sequence::{delimited, preceded},
    IResult, Parser,
};

use crate::{
    common,
    mathlist::{self, AtomType, MathList},
};

#[derive(Clone)]
pub enum Node<'a> {
    Char(char),
    Command(&'a str),
    Group(Vec<Node<'a>>),
    Whitespace,
}

fn parse_node<'a>(src: &'a str) -> IResult<&'a str, Node<'a>> {
    let parse_char = complete::none_of("}").map(Node::Char);
    let parse_command = preceded(complete::char('\\'), complete::alpha1).map(Node::Command);
    let parse_whitespace = take_while1(|c: char| c.is_whitespace()).map(|_| Node::Whitespace);

    let parse_group =
        delimited(complete::char('{'), many0(parse_node), complete::char('}')).map(Node::Group);
    let mut parser = nom::branch::alt((parse_whitespace, parse_group, parse_command, parse_char));

    parser(src)
}

impl<'a> Node<'a> {
    pub fn parse(src: &'a str) -> Option<Self> {
        let mut parser = many0(parse_node);
        let (_, mut nodes) = parser.parse(src).ok()?;

        if nodes.len() == 1 {
            nodes.drain(..).next()
        } else {
            Some(Node::Group(nodes))
        }
    }

    pub fn to_mathlist<G: common::Glyph>(&self) -> Option<MathList<G>> {
        let nodes = [self];
        Converter::convert(nodes.into_iter())
    }
}

struct Converter<'a, Glyph: common::Glyph, Input: Iterator<Item = &'a Node<'a>>> {
    input: Input,
    output: mathlist::Builder<Glyph>,
}

impl<'a, Glyph: common::Glyph, Input: Iterator<Item = &'a Node<'a>>> Converter<'a, Glyph, Input> {
    fn new(input: Input) -> Self {
        Self {
            input,
            output: Default::default(),
        }
    }

    fn next_non_white(&mut self) -> Option<&'a Node<'a>> {
        loop {
            let node = self.input.next()?;
            if !matches!(node, Node::Whitespace) {
                return Some(node);
            }
        }
    }

    pub fn convert(input: Input) -> Option<MathList<Glyph>> {
        let mut converter = Self::new(input);
        loop {
            if !converter.iterate()? {
                break;
            }
        }
        Some(converter.output.finish())
    }

    fn iterate(&mut self) -> Option<bool> {
        let node = match self.next_non_white() {
            Some(node) => node,
            None => return Some(false),
        };

        match node {
            Node::Char(ch) => self.add_char(*ch),
            Node::Command(cmd) => self.process_cmd(*cmd)?,
            Node::Group(nodes) => self.add_group(nodes)?,
            Node::Whitespace => unreachable!(),
        }

        Some(true)
    }

    fn add_char(&mut self, ch: char) {
        let (ch, atom_type) = match ch {
            '∑' => ('∑', AtomType::Op),
            '∫' => ('∫', AtomType::Op),
            '+' | '-' | '*' | '∧' | '∨' | '∩' | '∪' => (ch, AtomType::Bin),
            '=' | '<' | '>' | '∈' | '∉' | '≠' | '≤' | '≥' => (ch, AtomType::Rel),
            'h' => ('ℏ', AtomType::Ord),
            'A'..='Z' => (
                char::from_u32(ch as u32 + '𝐴' as u32 - 'A' as u32).unwrap(),
                AtomType::Ord,
            ),
            'a'..='z' => (
                char::from_u32(ch as u32 + '𝑎' as u32 - 'a' as u32).unwrap(),
                AtomType::Ord,
            ),
            _ => (ch, AtomType::Ord),
        };
        // todo: char classification. For now, just assume ord
        self.output.add_symbol(atom_type, ch);
    }

    fn add_group(&mut self, nodes: &Vec<Node>) -> Option<()> {
        let list = Converter::convert(nodes.iter())?;
        self.output.add_list(AtomType::Ord, list);
        Some(())
    }

    fn process_cmd(&mut self, cmd: &str) -> Option<()> {
        match cmd {
            "int" => self.output.add_op('∫'),
            "sum" => self.output.add_op('∑'),
            "mathop" => match self.next_non_white()? {
                Node::Char(ch) => self.output.add_op(*ch),
                Node::Command(_) => return None,
                Node::Group(nodes) => {
                    let list = Converter::convert(nodes.iter())?;
                    self.output.add_list(AtomType::Op, list);
                }
                Node::Whitespace => unreachable!(),
            },
            _ => return None,
        }
        Some(())
    }
}
