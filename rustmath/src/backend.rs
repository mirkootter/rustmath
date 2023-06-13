use crate::common;
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

impl common::Glyph for Glyph {
    fn height(&self) -> f32 {
        self.height
    }

    fn depth(&self) -> f32 {
        self.depth
    }

    fn advance(&self) -> f32 {
        self.advance
    }

    fn italic_correction(&self) -> f32 {
        self.italic_correction
    }
}

impl Glyph {
    fn get_italic_correction(f: &Face, id: ttf_parser::GlyphId, scale: f32) -> Option<f32> {
        let value = f
            .tables()
            .math?
            .glyph_info?
            .italic_corrections?
            .get(id)?
            .value;
        Some(value as f32 * scale)
    }

    fn new_from_id(f: &Face, id: ttf_parser::GlyphId, size: f32) -> Option<Self> {
        let scale = size / 1000.0;

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

    fn new(f: &Face, ch: char, size: f32) -> Option<Self> {
        let id = f.glyph_index(ch)?;
        Self::new_from_id(f, id, size)
    }
}

struct Font<'a> {
    face: ttf_parser::Face<'a>,
}

impl<'a> common::Font<FontBackend<'a>> for Font<'a> {
    fn get_glyph(
        &self,
        ch: char,
        size: f32,
    ) -> Option<<FontBackend<'a> as common::FontBackend>::Glyph> {
        Glyph::new(&self.face, ch, size)
    }

    fn get_larger_glyph(
        &self,
        ch: char,
        size: f32,
    ) -> Option<<FontBackend<'a> as common::FontBackend>::Glyph> {
        let glyph_id = self.face.glyph_index(ch)?;
        let construction = self
            .face
            .tables()
            .math?
            .variants?
            .vertical_constructions
            .get(glyph_id)?;

        let glyph_id = construction.variants.get(1)?.variant_glyph;

        Glyph::new_from_id(&self.face, glyph_id, size)
    }
}

pub struct FontBackend<'a> {
    font: Font<'a>,
}

impl<'a> common::FontBackend for FontBackend<'a> {
    type Glyph = Glyph;

    fn get_font(&self, _family: common::Family) -> &dyn common::Font<Self> {
        &self.font
    }
}

impl Default for FontBackend<'static> {
    fn default() -> Self {
        let face =
            ttf_parser::Face::parse(include_bytes!("../data/NewCMMath-Regular.otf"), 0).unwrap();
        let font = Font { face };
        Self { font }
    }
}

pub struct Renderer<'a> {
    pixmap: &'a mut tiny_skia::Pixmap,
    backend: FontBackend<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(pixmap: &'a mut tiny_skia::Pixmap, backend: FontBackend<'a>) -> Self {
        Renderer { pixmap, backend }
    }

    pub fn font_backend(&self) -> &FontBackend<'a> {
        &self.backend
    }
}

impl<'a> common::Renderer for Renderer<'a> {
    type FontBackend = FontBackend<'static>;

    fn render_glyph(
        &mut self,
        glyph: &<Self::FontBackend as common::FontBackend>::Glyph,
        x0: f32,
        y0: f32,
    ) {
        const DPI: f32 = 96.0;
        let scale = DPI / 72.0;

        let ts = tiny_skia::Transform::from_translate(x0, y0).post_scale(scale, scale);
        self.pixmap.fill_path(
            &glyph.path,
            &Default::default(),
            tiny_skia::FillRule::EvenOdd,
            ts,
            None,
        );
    }
}
