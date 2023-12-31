use crate::common::{self, Color};
use common::Family;

mod layout_helper;

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

    pub fn to_subscript(self) -> Self {
        if self < Self::TextCramped {
            // SS', SS, S', S -> SS'
            Self::SuperScriptCramped
        } else {
            // T', T, D', D
            Self::ScriptCramped
        }
    }

    pub fn to_superscript(self) -> Self {
        match self {
            Self::Display | Self::Text => Self::Script,
            Self::DisplayCramped | Self::TextCramped => Self::ScriptCramped,
            Self::Script | Self::SuperScript => Self::SuperScript,
            Self::ScriptCramped | Self::SuperScriptCramped => Self::SuperScriptCramped,
        }
    }

    pub fn is_cramped(self) -> bool {
        match self {
            Self::DisplayCramped
            | Self::TextCramped
            | Self::ScriptCramped
            | Self::SuperScriptCramped => true,
            _ => false,
        }
    }

    pub fn to_numerator(self) -> Self {
        match self {
            Self::Display => Self::Text,
            Self::DisplayCramped => Self::TextCramped,
            _ => self.to_superscript(),
        }
    }

    pub fn to_denominator(self) -> Self {
        match self {
            Self::Display | Self::DisplayCramped => Self::TextCramped,
            _ => self.to_subscript(),
        }
    }
}

impl Into<common::FontStyle> for Style {
    fn into(self) -> common::FontStyle {
        if self <= Self::SuperScript {
            common::FontStyle::SuperScript
        } else if self <= Self::Script {
            common::FontStyle::Script
        } else if self <= Self::Text {
            common::FontStyle::Text
        } else {
            common::FontStyle::Display
        }
    }
}

pub enum Node<Glyph: common::Glyph> {
    Atom(Atom<Glyph>),
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
        (Punct, _)
        | (Inner, Ord)
        | (Inner, Open)
        | (Inner, Punct)
        | (Inner, Inner)
        | (Ord, Inner)
        | (Op, Inner)
        | (Close, Inner) => nonscript(thin),
        (Ord, Bin) | (Bin, _) | (Close, Bin) | (Inner, Bin) => nonscript(med),
        (Ord, Rel) | (Op, Rel) | (Rel, _) | (Close, Rel) | (Inner, Rel) => nonscript(thick),
        _ => 0.0,
    }
}

impl<Glyph: common::Glyph> Node<Glyph> {
    pub fn atom_type(&self) -> Option<&AtomType> {
        let Node::Atom(atom) = self;
        Some(&atom.atom_type)
    }
}

pub struct MathList<Glyph: common::Glyph>(Vec<Node<Glyph>>);

pub struct Atom<Glyph: common::Glyph> {
    pub atom_type: AtomType,
    pub nucleus: Field<Glyph>,
    pub subscript: Field<Glyph>,
    pub superscript: Field<Glyph>,
}

pub struct Delimiter {
    pub ch: char,
    pub color: Color,
}

impl Delimiter {
    pub fn translate<G: common::Glyph>(
        &self,
        font: &dyn crate::common::Font<G>,
        content: &crate::layout::Node<G>,
        size: f32,
        style: Style,
    ) -> crate::layout::Node<G> {
        let params = font.calculate_general_params(size, style.into(), style.is_cramped());
        let half_height = content.height(false) - params.axis_height;
        let half_height = half_height.max(content.depth() + params.axis_height);

        let min_height = 2.0 * half_height.max(0.0);
        let result = crate::layout::Node::new_extended_glyph_vert(
            font,
            self.ch,
            min_height,
            size,
            style.into(),
            self.color,
        );
        result.unwrap_or_else(|| {
            let glyph = font.get_fallback_glyph(size, style.into());
            crate::layout::Node::Glyph {
                glyph,
                color: Color::Error,
            }
        })
    }
}

pub enum Field<Glyph: common::Glyph> {
    Empty,
    Symbol(Color, char),
    Fallback(Color),
    MathList(Option<Delimiter>, MathList<Glyph>, Option<Delimiter>),
    Fraction(Box<Self>, Box<Self>),
    Layout {
        translation: crate::layout::Node<Glyph>,
        italic_correction: f32,
    }, // already translated
}

impl<G: common::Glyph> Field<G> {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
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

pub struct Builder<Glyph: common::Glyph> {
    list: Vec<Node<Glyph>>,
}

impl<Glyph: common::Glyph> Default for Builder<Glyph> {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

impl<Glyph: common::Glyph> Builder<Glyph> {
    pub fn finish(self) -> MathList<Glyph> {
        MathList(self.list)
    }

    pub fn add_atom(&mut self, atom: Atom<Glyph>) {
        self.list.push(Node::Atom(atom));
    }

    pub fn add_symbol(&mut self, ch: char, color: Color) {
        let nucleus = Field::Symbol(color, ch);
        let atom = Atom {
            atom_type: AtomType::Ord,
            nucleus,
            subscript: Field::Empty,
            superscript: Field::Empty,
        };
        self.add_atom(atom);
    }
}

impl<Glyph: common::Glyph> MathList<Glyph> {
    /// Translate into boxes
    pub fn translate<B: common::FontBackend<Glyph = Glyph>>(
        self,
        backend: &B,
        size: f32,
        style: Style,
    ) -> crate::layout::Node<Glyph> {
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
            let Node::Atom(atom) = node;
            let big = match &atom.atom_type {
                AtomType::Op if style > Style::Text => true,
                _ => false,
            };

            // TODO: Implement Rule 14 from the TeX-book
            // This means dealing with kerning, ligatures

            // TODO: Determine if we need italic correction (See Rule 17)
            let italic_correction = false;

            atom.nucleus.translate(
                backend,
                size,
                style,
                big,
                !atom.subscript.is_empty(),
                italic_correction,
            );

            atom.subscript
                .translate(backend, size, style.to_subscript(), false, false, true);
            atom.superscript
                .translate(backend, size, style.to_superscript(), false, false, true);
        }

        let mut nodes = Vec::new();
        nodes.reserve_exact(list.len() * 2);
        {
            let mut previous_atom_type = None::<AtomType>;
            for node in &mut list {
                let Node::Atom(atom) = node;
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
                        nodes.push((0.0, crate::layout::Node::Glue(spacing)));
                    }
                }

                previous_atom_type = Some(atom_type.clone());

                let nucleus = atom.nucleus.take_translation();
                let subscript = atom.subscript.take_translation();
                let superscript = atom.superscript.take_translation();

                let italic_correction = nucleus.as_ref().map(|n| n.1).unwrap_or(0.0);
                let nucleus = nucleus.map(|n| n.0);
                let subscript = subscript.map(|n| n.0);
                let superscript = superscript.map(|n| n.0);

                let font = backend.get_font(Family::Italic); // Default math font
                let params = font.calculate_script_params(size, style.into(), style.is_cramped());

                let (subscript_vshift, mut superscript_vshift) =
                    layout_helper::calculate_script_shifts(
                        &params,
                        &nucleus,
                        &subscript,
                        &superscript,
                    );

                if let Some(atom) = nucleus {
                    nodes.push((0.0, atom));
                }

                let mut script_nodes = Vec::new();
                let mut vshift = superscript_vshift;

                if let Some(script) = subscript {
                    superscript_vshift -= subscript_vshift + script.height(false);
                    script_nodes.push((0.0, script));
                    vshift = subscript_vshift;
                }

                if let Some(script) = superscript {
                    if !script_nodes.is_empty() && superscript_vshift > 0.0 {
                        script_nodes.push((0.0, crate::layout::Node::Glue(superscript_vshift)));
                    }
                    script_nodes.push((italic_correction, script));
                }

                if !script_nodes.is_empty() {
                    let vbox = crate::layout::Node::new_vbox(script_nodes);
                    nodes.push((vshift, vbox));
                }
            }
        }

        crate::layout::Node::new_hbox(nodes)
    }
}

impl<Glyph: common::Glyph> Field<Glyph> {
    fn translate<B: common::FontBackend<Glyph = Glyph>>(
        &mut self,
        backend: &B,
        size: f32,
        style: Style,
        big: bool,
        has_subscript: bool,
        _want_italic_correction: bool,
    ) {
        match self {
            Field::Symbol(color, ch) => {
                let font = backend.get_font(Family::Italic);

                let glyph = big
                    .then_some(())
                    .and_then(|_| font.get_larger_glyph(*ch, size, style.into(), !has_subscript))
                    .or_else(|| font.get_glyph(*ch, size, style.into()));

                let (color, glyph) = if let Some(glyph) = glyph {
                    (*color, glyph)
                } else {
                    let glyph = font.get_fallback_glyph(size, style.into());
                    (Color::Error, glyph)
                };

                let italic_correction = glyph.italic_correction();
                let translation = crate::layout::Node::Glyph { glyph, color };
                *self = Field::Layout {
                    translation,
                    italic_correction,
                };
            }
            Field::Fallback(color) => {
                let font = backend.get_font(Family::Italic);
                let glyph = font.get_fallback_glyph(size, style.into());
                let italic_correction = glyph.italic_correction();
                let translation = crate::layout::Node::Glyph {
                    glyph,
                    color: *color,
                };
                *self = Field::Layout {
                    translation,
                    italic_correction,
                };
            }
            Field::MathList(left, list, right) => {
                let mut taken_list = MathList(Default::default());
                std::mem::swap(&mut taken_list, list);
                let mut translation = taken_list.translate(backend, size, style);

                if left.is_some() || right.is_some() {
                    let mut children = Vec::new();
                    let font = backend.get_font(Family::Italic);

                    let left = left
                        .as_ref()
                        .map(|del| del.translate(font, &translation, size, style));
                    let right = right
                        .as_ref()
                        .map(|del| del.translate(font, &translation, size, style));

                    if let Some(left) = left {
                        children.push((0.0, left));
                    }

                    children.push((0.0, translation));

                    if let Some(right) = right {
                        children.push((0.0, right));
                    }

                    translation = crate::layout::Node::new_hbox(children);
                }

                *self = Field::Layout {
                    translation,
                    italic_correction: 0.0,
                };
            }
            Field::Fraction(num, denom) => {
                let mut children = Vec::new();
                children.reserve_exact(3);

                num.translate(backend, size, style.to_numerator(), false, false, true);
                denom.translate(backend, size, style.to_denominator(), false, false, true);

                let num = num.take_translation();
                let denom = denom.take_translation();

                let font = backend.get_font(Family::Italic);
                let general_params =
                    font.calculate_general_params(size, style.into(), style.is_cramped());
                let frac_params =
                    font.calculate_fraction_params(size, style.into(), style.is_cramped());

                let width = {
                    let mut width = 0.0f32;
                    if let Some((num, italic_correction)) = &num {
                        width = width.max(num.advance(true) + italic_correction);
                    };
                    if let Some((denom, italic_correction)) = &denom {
                        width = width.max(denom.advance(false) + italic_correction);
                    }

                    width
                };

                let mut vshift = general_params.axis_height;
                if let Some((denom, italic_correction)) = denom {
                    let gap = layout_helper::calculate_denominator_gap(
                        general_params.axis_height,
                        &frac_params,
                        &denom,
                    );
                    vshift -= denom.height(false) + gap + frac_params.rule_thickness / 2.0;

                    let hshift = (width - denom.advance(true) - italic_correction).max(0.0) / 2.0;
                    children.push((hshift, denom));
                    children.push((0.0, crate::layout::Node::Glue(gap)));
                }

                children.push((
                    0.0,
                    crate::layout::Node::new_rule(width, frac_params.rule_thickness),
                ));

                if let Some((num, italic_correction)) = num {
                    let gap = layout_helper::calculate_numerator_gap(
                        general_params.axis_height,
                        &frac_params,
                        &num,
                    );
                    let hshift = (width - num.advance(true) - italic_correction).max(0.0) / 2.0;
                    children.push((0.0, crate::layout::Node::Glue(gap)));
                    children.push((hshift, num));
                }

                let vbox = crate::layout::Node::new_vbox(children);
                let translation = crate::layout::Node::new_hbox(vec![(vshift, vbox)]);
                *self = Field::Layout {
                    translation,
                    italic_correction: 0.0,
                };
            }
            Field::Empty | Field::Layout { .. } => {}
        }
    }

    fn take_translation(&mut self) -> Option<(crate::layout::Node<Glyph>, f32)> {
        let mut f = Field::Empty;
        std::mem::swap(&mut f, self);
        match f {
            Field::Layout {
                translation,
                italic_correction,
            } => Some((translation, italic_correction)),
            _ => None,
        }
    }
}
