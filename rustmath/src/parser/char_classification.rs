use crate::mathlist::AtomType;

mod generated;

#[derive(Clone, Copy)]
pub enum CharClassification {
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

impl CharClassification {
    pub fn classify(ch: char) -> Self {
        let idx =
            generated::CHAR_CLASSIFICATION.partition_point(|(boundary, _)| *boundary <= ch as u32);
        generated::CHAR_CLASSIFICATION[idx - 1].1
    }

    pub fn to_atom_type(self) -> Option<AtomType> {
        match self {
            CharClassification::Normal
            | CharClassification::Alphabetic
            | CharClassification::Diacritic
            | CharClassification::Unary
            | CharClassification::Special => Some(AtomType::Ord),
            CharClassification::Binary => Some(AtomType::Bin),
            CharClassification::Closing => Some(AtomType::Close),
            CharClassification::Fence => Some(AtomType::Ord),
            CharClassification::GlyphPart => None,
            CharClassification::Large => Some(AtomType::Op),
            CharClassification::Opening => Some(AtomType::Open),
            CharClassification::Punct => Some(AtomType::Punct),
            CharClassification::Relation => Some(AtomType::Rel),
            CharClassification::Space => None,
            CharClassification::Vary => Some(AtomType::Bin),
            CharClassification::Ignore => None,
        }
    }
}
