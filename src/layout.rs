use ttf_parser::{Face, GlyphId};

#[derive(Default)]
struct OutlineBuilder {
    path_builder: tiny_skia::PathBuilder,
}

impl OutlineBuilder {
    pub fn finish(self, scale: f32) -> tiny_skia::Path {
        let ts = tiny_skia::Transform::from_scale(scale, -scale);
        self.path_builder.finish().unwrap().transform(ts).unwrap()
    }
}

impl ttf_parser::OutlineBuilder for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path_builder.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path_builder.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path_builder.quad_to(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path_builder.cubic_to(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        self.path_builder.close();
    }
}

pub struct Glyph {
    pub id: GlyphId,
    pub height: f32,
    pub depth: f32,
    pub advance: f32,
    pub italic_correction: f32,
    pub path: tiny_skia::Path,
}

impl Glyph {
    fn get_italic_correction(f: &Face, id: ttf_parser::GlyphId, scale: f32) -> Option<f32> {
        let value = f.tables().math?.glyph_info?.italic_corrections?.get(id)?.value;
        Some(value as f32 * scale)
    }

    pub fn new(f: &Face, ch: char, size: f32) -> Option<Self> {
        let scale = size / 1000.0;

        let id = f.glyph_index(ch)?;
        let advance = f.glyph_hor_advance(id)? as f32 * scale;

        let mut outline_builder = OutlineBuilder::default();
        let bounds = f.outline_glyph(id, &mut outline_builder)?;
        let path = outline_builder.finish(scale);

        let italic_correction = Self::get_italic_correction(f, id, scale).unwrap_or(0.0);

        Some(Glyph {
            id,
            height: bounds.y_max.max(0) as f32 * scale,
            depth: (-bounds.y_min).max(0) as f32 * scale,
            advance,
            italic_correction,
            path,
        })
    }

    pub fn render(&self, pixmap: &mut tiny_skia::Pixmap, x0: f32, y0: f32) {
        const DPI: f32 = 96.0;
        let scale = DPI / 72.0;

        let ts = tiny_skia::Transform::from_translate(x0, y0).post_scale(scale, scale);
        pixmap.fill_path(
            &self.path,
            &Default::default(),
            tiny_skia::FillRule::EvenOdd,
            ts,
            None,
        )
    }
}

pub enum Node {
    Glue(f32),
    Glyph {
        glyph: Glyph,
        dx: f32,
        dy: f32,
    },
    HBox {
        children: Vec<Node>,
        height: f32,
        depth: f32,
        advance: f32,
    },
}

impl Node {
    pub fn new_glyph(face: &Face, ch: char, size: f32) -> Self {
        let glyph = Glyph::new(face, ch, size).unwrap();
        Node::Glyph {
            glyph,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph, dy, .. } => (glyph.height + dy).max(0.0),
            Node::HBox { height, .. } => *height,
        }
    }

    pub fn depth(&self) -> f32 {
        match self {
            Node::Glue(_) => 0.0,
            Node::Glyph { glyph, dy, .. } => (glyph.depth - dy).max(0.0),
            Node::HBox { depth, .. } => *depth,
        }
    }

    pub fn advance(&self) -> f32 {
        match self {
            Node::Glue(w) => *w,
            Node::Glyph { glyph, dx, .. } => glyph.advance + dx,
            Node::HBox { advance, .. } => *advance,
        }
    }

    pub fn new_hbox(children: Vec<Node>) -> Node {
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

    pub fn render(&self, pixmap: &mut tiny_skia::Pixmap, x0: f32, y0: f32) {
        match self {
            Node::Glue(_) => {}
            Node::Glyph { glyph, dx, dy } => glyph.render(pixmap, x0 + dx, y0 - dy),
            Node::HBox { children, .. } => {
                let mut x = x0;

                for child in children {
                    child.render(pixmap, x, y0);
                    x += child.advance();
                }
            }
        }
    }
}
