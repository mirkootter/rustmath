use crate::common;

pub enum Node<Glyph: common::Glyph> {
    Glue(f32),
    Glyph {
        glyph: Glyph,
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
}

impl<Glyph: common::Glyph> Node<Glyph> {
    pub fn height(&self) -> f32 {
        match self {
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph } => glyph.height(),
            Node::HBox { height, .. } | Node::VBox { height, .. } => *height,
        }
    }

    pub fn depth(&self) -> f32 {
        match self {
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph } => glyph.depth(),
            Node::HBox { depth, .. } | Node::VBox { depth, .. } => *depth,
        }
    }

    pub fn advance(&self) -> f32 {
        match self {
            Node::Glue(w) => *w,
            Node::Glyph { glyph } => glyph.advance(),
            Node::HBox { advance, .. } | Node::VBox { advance, .. } => *advance,
        }
    }

    pub fn new_hbox(children: Vec<(f32, Self)>) -> Self {
        let mut height = 0f32;
        let mut depth = 0f32;
        let mut advance = 0f32;

        for (vshift, node) in &children {
            height = height.max(node.height() + vshift);
            depth = depth.max(node.depth() - vshift);
            advance += node.advance()
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

            advance = advance.max(node.advance() + hshift);

            height += node.height();
        }

        Node::VBox {
            children,
            height,
            depth,
            advance,
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
            Node::Glyph { glyph } => renderer.render_glyph(glyph, x0, y0),
            Node::HBox { children, .. } => {
                let mut x = x0;

                for (vshift, child) in children {
                    child.render(renderer, x, y0 - vshift);
                    x += child.advance();
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
                    y -= child.height();
                }
            }
        }
    }
}
