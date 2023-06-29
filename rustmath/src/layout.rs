use crate::common::{self, Color, Construction, Font, FontStyle};

pub enum Node<Glyph: common::Glyph> {
    Glue(f32),
    Glyph {
        glyph: Glyph,
        color: Color,
    },
    HBox {
        children: Vec<(f32, Self)>,
        height: f32,
        depth: f32,
        advance: f32,
    },
    VBox {
        children: Vec<(f32, Self)>,
        height: f32,
        depth: f32,
        advance: f32,
    },
    Rule {
        height: f32,
        depth: f32,
        advance: f32,
    },
}

impl<Glyph: common::Glyph> Node<Glyph> {
    pub fn height(&self, vertical_mode: bool) -> f32 {
        match self {
            Node::Glue(h) if vertical_mode => *h,
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph, .. } => glyph.height(),
            Node::HBox { height, .. } | Node::VBox { height, .. } | Node::Rule { height, .. } => {
                *height
            }
        }
    }

    pub fn depth(&self) -> f32 {
        match self {
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph, .. } => glyph.depth(),
            Node::HBox { depth, .. } | Node::VBox { depth, .. } | Node::Rule { depth, .. } => {
                *depth
            }
        }
    }

    pub fn advance(&self, vertical_mode: bool) -> f32 {
        match self {
            Node::Glue(_) if vertical_mode => 0.0,
            Node::Glue(w) => *w,
            Node::Glyph { glyph, .. } => glyph.advance(),
            Node::HBox { advance, .. }
            | Node::VBox { advance, .. }
            | Node::Rule { advance, .. } => *advance,
        }
    }

    pub fn new_hbox(children: Vec<(f32, Self)>) -> Self {
        let mut height = 0f32;
        let mut depth = 0f32;
        let mut advance = 0f32;

        for (vshift, node) in &children {
            height = height.max(node.height(false) + vshift);
            depth = depth.max(node.depth() - vshift);
            advance += node.advance(false)
        }

        Node::HBox {
            children,
            height,
            depth,
            advance,
        }
    }

    pub fn new_vbox(children: Vec<(f32, Self)>) -> Self {
        let mut height = 0f32;
        let mut depth = 0f32;
        let mut advance = 0f32;
        let mut vshift = 0f32;

        let mut first = true;
        for (hshift, node) in &children {
            if first {
                if let Self::Glue(glue) = node {
                    vshift += glue;
                    continue;
                } else {
                    first = false;
                    depth = node.depth();
                }
            } else {
                height += node.depth();
            }

            advance = advance.max(node.advance(true) + hshift);

            height += node.height(true);
        }

        let height = (height + vshift).max(0.0);
        let depth = (depth - vshift).max(0.0);

        Node::VBox {
            children,
            height,
            depth,
            advance,
        }
    }

    pub fn new_rule(width: f32, height: f32) -> Self {
        Self::Rule {
            height,
            depth: 0.0,
            advance: width,
        }
    }

    fn children_from_construction(
        construction: Construction<Glyph>,
        min_size: f32,
        color: Color,
        center: Option<f32>,
    ) -> Vec<(f32, Self)> {
        let (size, iter) = construction.construct(min_size);
        let mut result = Vec::new();
        result.reserve_exact(result.len() * 2 + 1);

        if let Some(center) = center {
            let glue = center - size / 2.0;
            result.push((0.0, Self::Glue(glue)));
        }

        for (overlap, part) in iter {
            if overlap != 0.0 {
                result.push((0.0, Self::Glue(-overlap)));
            }

            let glyph = part.glyph.clone();
            result.push((0.0, Self::Glyph { glyph, color }));
        }

        result
    }

    pub fn new_extended_glyph_hor(
        font: &dyn Font<Glyph>,
        ch: char,
        min_width: f32,
        size: f32,
        style: FontStyle,
        color: Color,
    ) -> Option<Self> {
        let glyph = font.get_glyph_minwidth(ch, size, style, min_width);
        if let Some(glyph) = glyph {
            return Some(Self::Glyph { glyph, color });
        }

        let construction = font.get_glyph_hor_construction(ch, size, style)?;
        let children = Self::children_from_construction(construction, min_width, color, None);
        Some(Self::new_hbox(children))
    }

    pub fn new_extended_glyph_vert(
        font: &dyn Font<Glyph>,
        ch: char,
        min_height: f32,
        size: f32,
        style: FontStyle,
        color: Color,
    ) -> Option<Self> {
        // TODO: Correct 'cramped' param
        let params = font.calculate_general_params(size, style.into(), false);

        let glyph = font.get_glyph_minheight(ch, size, style, min_height);
        if let Some(glyph) = glyph {
            return Some(Self::Glyph { glyph, color });
        }

        let construction = font.get_glyph_vert_construction(ch, size, style)?;
        let children = Self::children_from_construction(
            construction,
            min_height,
            color,
            Some(params.axis_height),
        );
        Some(Self::new_vbox(children))
    }
}

impl<Glyph: common::Glyph> Node<Glyph> {
    pub fn render<B: common::FontBackend<Glyph = Glyph>, R: common::Renderer<FontBackend = B>>(
        &self,
        renderer: &mut R,
        x0: f32,
        y0: f32,
    ) {
        match self {
            Node::Glue(_) => {}
            Node::Glyph { glyph, color } => renderer.render_glyph(glyph, x0, y0, *color),
            Node::HBox { children, .. } => {
                let mut x = x0;

                for (vshift, child) in children {
                    child.render(renderer, x, y0 + vshift);
                    x += child.advance(false);
                }
            }
            Node::VBox { children, .. } => {
                let mut y = y0;
                let mut first = true;

                for (hshift, child) in children {
                    if first {
                        first = false;
                    } else {
                        y += child.depth();
                    }
                    child.render(renderer, x0 + hshift, y);
                    y += child.height(true);
                }
            }
            Node::Rule {
                height,
                depth,
                advance,
            } => {
                renderer.render_box(x0, y0 - depth, *advance, depth + height);
            }
        }
    }
}
