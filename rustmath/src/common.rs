#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Normal,
    Error,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Family {
    Roman,
    Italic,
}

pub trait Glyph {
    fn height(&self) -> f32;
    fn depth(&self) -> f32;
    fn advance(&self) -> f32;
    fn italic_correction(&self) -> f32;
}

pub trait FontBackend {
    type Glyph: Glyph;

    fn get_font(&self, family: Family) -> &dyn Font<Self>;
}

pub trait Font<B: FontBackend> {
    fn get_glyph(&self, ch: char, size: f32) -> Option<B::Glyph>;
    fn get_larger_glyph(&self, ch: char, size: f32) -> Option<B::Glyph>;
}

pub trait Renderer {
    type FontBackend: FontBackend;

    fn render_glyph(
        &mut self,
        glyph: &<Self::FontBackend as FontBackend>::Glyph,
        x0: f32,
        y0: f32,
        color: Color,
    );
}
