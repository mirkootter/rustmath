#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Style {
    SuperScriptCramped,
    SuperScript,
    ScriptCramped,
    Script,
    TextCramped,
    Text,
    DisplayCramped,
    Display,
}

impl Style {
    pub fn to_cramped(self) -> Self {
        match self {
            Self::SuperScript => Self::SuperScriptCramped,
            Self::Script => Self::ScriptCramped,
            Self::Text => Self::TextCramped,
            Self::Display => Self::DisplayCramped,
            _ => self,
        }
    }
}

pub enum Node {
    Atom(Atom),
}

fn spacing(
    left: &AtomType,
    right: &AtomType,
    is_script: bool,
    thin: f32,
    med: f32,
    thick: f32,
) -> f32 {
    use AtomType::*;

    let nonscript = |w| {
        if is_script {
            0.0f32
        } else {
            w
        }
    };

    match (left, right) {
        (Ord, Op) | (Op, Ord) | (Op, Op) | (Close, Op) | (Inner, Op) => thin,
        (Punct, _) | (Inner, Ord) | (Inner, Open) | (Inner, Punct) | (Inner, Inner) => {
            nonscript(thin)
        }
        (Ord, Bin) | (Bin, _) | (Close, Bin) | (Inner, Bin) => nonscript(med),
        (Ord, Rel) | (Op, Rel) | (Rel, _) | (Close, Rel) | (Inner, Rel) => nonscript(thick),
        _ => 0.0,
    }
}

impl Node {
    pub fn atom_type(&self) -> Option<&AtomType> {
        if let Node::Atom(atom) = self {
            Some(&atom.atom_type)
        } else {
            None
        }
    }
}

pub struct MathList(Vec<Node>);

pub struct Atom {
    pub atom_type: AtomType,
    pub nucleus: Field,
    pub subscript: Field,
    pub superscript: Field,
}

pub enum Field {
    Empty,
    Symbol(char),
    MathList(MathList),
    Layout(crate::layout::Node), // already translated
}

#[derive(Clone)]
pub enum AtomType {
    Acc,
    Bin,
    Close,
    Inner,
    Op,
    Open,
    Ord,
    Punct,
    Rel,
}

impl AtomType {
    /// Some atoms might precede a binary operator, some don't
    pub fn might_precede_bin(&self) -> bool {
        match &self {
            AtomType::Bin | AtomType::Rel | AtomType::Open | AtomType::Punct => false,
            _ => true,
        }
    }
}

#[derive(Default)]
pub struct Builder {
    list: Vec<Node>,
}

impl Builder {
    pub fn finish(self) -> MathList {
        MathList(self.list)
    }

    fn add_atom(&mut self, atom_type: AtomType, nucleus: Field) {
        let atom = Atom {
            atom_type,
            nucleus,
            subscript: Field::Empty,
            superscript: Field::Empty,
        };

        self.list.push(Node::Atom(atom));
    }

    pub fn add_symbol(&mut self, atom_type: AtomType, ch: char) {
        let nucleus = Field::Symbol(ch);
        self.add_atom(atom_type, nucleus);
    }

    pub fn add_list(&mut self, atom_type: AtomType, list: MathList) {
        let nucleus = Field::MathList(list);
        self.add_atom(atom_type, nucleus);
    }

    pub fn add_bin(&mut self, ch: char) {
        self.add_symbol(AtomType::Bin, ch);
    }

    pub fn add_op(&mut self, ch: char) {
        self.add_symbol(AtomType::Op, ch);
    }

    pub fn add_ord(&mut self, ch: char) {
        self.add_symbol(AtomType::Ord, ch);
    }

    pub fn add_punct(&mut self, ch: char) {
        self.add_symbol(AtomType::Punct, ch);
    }

    pub fn add_rel(&mut self, ch: char) {
        self.add_symbol(AtomType::Rel, ch);
    }
}

impl MathList {
    /// Translate into boxes
    pub fn translate(
        self,
        face: &ttf_parser::Face,
        size: f32,
        style: Style,
    ) -> crate::layout::Node {
        let mut list = self.0;

        let quad_size = size; // TODO
        let mu = quad_size / 18.0;
        let thin_space = 3.0 * mu;
        let med_space = 4.0 * mu;
        let thick_space = 5.0 * mu;

        // TODO: The first pass should change atom_types to Ord, if necessary
        // For example, two `Bin` atoms must not be adjacient; a bin atom
        // must not be the first or the last, and many more
        {
            // ...
        }

        // TODO: Other passes?

        // Translate the nucleus for all atoms which have not been translated yet
        for node in &mut list {
            if let Node::Atom(atom) = node {
                match &atom.atom_type {
                    AtomType::Op => {
                        if style > Style::Text {
                            atom.nucleus.grow_symbol_if_possible(face, size);
                        }
                    }
                    _ => {}
                }

                // TODO: Implement Rule 14 from the TeX-book
                // This means dealing with kerning, ligatures

                // TODO: Determine if we need italic correction (See Rule 17)
                let italic_correction = false;

                atom.nucleus.translate(face, size, style, italic_correction);
            }
        }

        let mut nodes = Vec::new();
        nodes.reserve_exact(list.len() * 2);
        {
            let mut previous_atom_type = None::<AtomType>;
            for node in &mut list {
                if let Node::Atom(atom) = node {
                    let atom_type = &atom.atom_type;

                    if let Some(previous_atom_type) = &previous_atom_type {
                        let is_script = style <= Style::Script;
                        let spacing = spacing(
                            previous_atom_type,
                            &atom_type,
                            is_script,
                            thin_space,
                            med_space,
                            thick_space,
                        );

                        if spacing > 0.0 {
                            nodes.push(crate::layout::Node::Glue(spacing));
                        }
                    }

                    previous_atom_type = Some(atom_type.clone());
                    if let Some(atom) = atom.nucleus.take_translation() {
                        nodes.push(atom);
                    }
                }
            }
        }

        crate::layout::Node::new_hbox(nodes)
    }
}

impl Field {
    fn translate(
        &mut self,
        face: &ttf_parser::Face,
        size: f32,
        style: Style,
        _want_italic_correction: bool,
    ) {
        match self {
            Field::Symbol(ch) => {
                let node = crate::layout::Node::new_glyph(face, *ch, size);
                *self = Field::Layout(node);
            }
            Field::MathList(list) => {
                let mut taken_list = MathList(Default::default());
                std::mem::swap(&mut taken_list, list);
                let node = taken_list.translate(face, size, style);
                *self = Field::Layout(node);
            }
            _ => {} // Nothing to do
        }
    }

    /// If the field contains a single symbol, try to grow it once
    /// This is used in display mode to enlarge operations like \sum
    fn grow_symbol_if_possible(&mut self, face: &ttf_parser::Face, size: f32) {
        let ch = match self {
            Field::Symbol(ch) => *ch,
            _ => return,
        };

        let mut try_grow = || -> Option<()> {
            let glyph_id = face.glyph_index(ch)?;
            let construction = face
                .tables()
                .math?
                .variants?
                .vertical_constructions
                .get(glyph_id)?;
            
            let glyph_id = construction.variants.get(1)?.variant_glyph;
            let glyph = crate::layout::Glyph::new_from_id(face, glyph_id, size)?;
            let node = crate::layout::Node::Glyph { glyph, dx: 0.0, dy: 0.0 };
            *self = Field::Layout(node);
            None
        };

        try_grow();
    }

    fn take_translation(&mut self) -> Option<crate::layout::Node> {
        let mut f = Field::Empty;
        std::mem::swap(&mut f, self);
        match f {
            Field::Layout(node) => Some(node),
            _ => None,
        }
    }
}
