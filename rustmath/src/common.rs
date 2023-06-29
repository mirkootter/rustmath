pub mod construction;
pub mod font_params;

pub use construction::Construction;

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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FontStyle {
    Display,
    Text,
    Script,
    SuperScript,
}

pub trait Glyph: Clone {
    fn height(&self) -> f32;
    fn depth(&self) -> f32;
    fn advance(&self) -> f32;
    fn italic_correction(&self) -> f32;

    fn set_advance(&mut self, advance: f32);
}

pub trait FontBackend {
    type Glyph: Glyph;

    fn get_font(&self, family: Family) -> &dyn Font<Self::Glyph>;
}

pub trait Font<G: Glyph> {
    fn get_fallback_glyph(&self, size: f32, style: FontStyle) -> G;
    fn get_glyph(&self, ch: char, size: f32, style: FontStyle) -> Option<G>;
    fn get_larger_glyph(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
        include_italic_correction: bool,
    ) -> Option<G>;

    fn get_glyph_minwidth(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
        min_width: f32,
    ) -> Option<G>;
    fn get_glyph_minheight(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
        min_height: f32,
    ) -> Option<G>;
    fn get_glyph_hor_construction(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
    ) -> Option<Construction<G>>;
    fn get_glyph_vert_construction(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
    ) -> Option<Construction<G>>;

    fn calculate_script_params(
        &self,
        size: f32,
        style: FontStyle,
        cramped: bool,
    ) -> font_params::ScriptParams;

    fn calculate_general_params(
        &self,
        size: f32,
        style: FontStyle,
        cramped: bool,
    ) -> font_params::GeneralParams;

    fn calculate_fraction_params(
        &self,
        size: f32,
        style: FontStyle,
        cramped: bool,
    ) -> font_params::FractionParams;
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

    fn render_box(&mut self, x0: f32, y0: f32, width: f32, height: f32);
}
