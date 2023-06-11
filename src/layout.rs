use crate::common;

pub enum Node<Glyph: common::Glyph> {
    Glue(f32),
    Glyph {
        glyph: Glyph,
        dx: f32,
        dy: f32,
    },
    HBox {
        children: Vec<Self>,
        height: f32,
        depth: f32,
        advance: f32,
    },
}

impl<Glyph: common::Glyph> Node<Glyph> {
    pub fn height(&self) -> f32 {
        match self {
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph, dy, .. } => (glyph.height() + dy).max(0.0),
            Node::HBox { height, .. } => *height,
        }
    }

    pub fn depth(&self) -> f32 {
        match self {
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph, dy, .. } => (glyph.depth() - dy).max(0.0),
            Node::HBox { depth, .. } => *depth,
        }
    }

    pub fn advance(&self) -> f32 {
        match self {
            Node::Glue(w) => *w,
            Node::Glyph { glyph, dx, .. } => glyph.advance() + dx,
            Node::HBox { advance, .. } => *advance,
        }
    }

    pub fn new_hbox(children: Vec<Self>) -> Self {
        let mut height = 0f32;
        let mut depth = 0f32;
        let mut advance = 0f32;

        for node in &children {
            height = height.max(node.height());
            depth = depth.max(node.depth());
            advance += node.advance()
        }

        Node::HBox {
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
            Node::Glyph { glyph, dx, dy } => renderer.render_glyph(glyph, x0 + dx, y0 - dy),
            Node::HBox { children, .. } => {
                let mut x = x0;

                for child in children {
                    child.render(renderer, x, y0);
                    x += child.advance();
                }
            }
        }
    }
}
