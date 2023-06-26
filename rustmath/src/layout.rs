use crate::common::{self, Color};

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

        let mut first = true;
        for (hshift, node) in &children {
            if first {
                first = false;
                depth = node.depth();
            } else {
                height += node.depth();
            }

            advance = advance.max(node.advance(true) + hshift);

            height += node.height(true);
        }

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
                    child.render(renderer, x, y0 - vshift);
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
                        y -= child.depth();
                    }
                    child.render(renderer, x0 + hshift, y);
                    y -= child.height(true);
                }
            }
            Node::Rule {
                height,
                depth,
                advance,
            } => {
                renderer.render_box(x0, y0 + depth, *advance, depth + height);
            }
        }
    }
}
