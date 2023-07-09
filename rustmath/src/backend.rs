use crate::common::{self, construction::ConstructionPart, Color, Construction, FontStyle};
use ttf_parser::{Face, GlyphId};

mod opentype;

#[derive(Default)]
pub struct OutlineBuilder {
    path_builder: tiny_skia::PathBuilder,
}

impl opentype::OutlineBuilder<tiny_skia::Path> for OutlineBuilder {
    fn finish(self, scale: f32) -> tiny_skia::Path {
        let ts = tiny_skia::Transform::from_scale(scale, scale);
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

#[derive(Clone)]
pub struct Glyph<R: opentype::OpenTypeRenderer> {
    pub id: GlyphId,
    pub height: f32,
    pub depth: f32,
    pub advance: f32,
    pub italic_correction: f32,
    pub path: R::Path,
}

impl<R: opentype::OpenTypeRenderer> common::Glyph for Glyph<R> {
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

    fn set_advance(&mut self, advance: f32) {
        self.advance = advance;
    }
}

impl<R: opentype::OpenTypeRenderer> Glyph<R> {
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
        use self::opentype::OutlineBuilder;

        let scale = size / 1000.0;

        let advance = f.glyph_hor_advance(id)? as f32 * scale;

        let mut outline_builder = R::OutlineBuilder::default();
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

struct Font<'a, R: opentype::OpenTypeRenderer> {
    face: ttf_parser::Face<'a>,
    _phantom: std::marker::PhantomData<R>,
}

impl<'a, R: opentype::OpenTypeRenderer> Font<'a, R> {
    fn size_for_style(&self, size: f32, style: FontStyle) -> f32 {
        match style {
            FontStyle::Display | FontStyle::Text => size,
            FontStyle::Script => {
                size * (self
                    .face
                    .tables()
                    .math
                    .unwrap()
                    .constants
                    .unwrap()
                    .script_percent_scale_down() as f32
                    / 100.0)
            }
            FontStyle::SuperScript => {
                size * (self
                    .face
                    .tables()
                    .math
                    .unwrap()
                    .constants
                    .unwrap()
                    .script_script_percent_scale_down() as f32
                    / 100.0)
            }
        }
    }

    fn get_glyph_minsize(
        &self,
        construction: ttf_parser::math::GlyphConstruction<'a>,
        size: f32,
        min_size: f32,
    ) -> Option<Glyph<R>> {
        let min_size = (min_size * 1000.0 / size - 1e-3).round() as u16;
        for variant in construction.variants {
            if variant.advance_measurement >= min_size {
                return Glyph::new_from_id(&self.face, variant.variant_glyph, size);
            }
        }

        None
    }

    fn get_glyph_construction(
        &self,
        min_overlap: f32,
        assembly: ttf_parser::math::GlyphAssembly<'a>,
        size: f32,
    ) -> Construction<Glyph<R>> {
        let scale = size / 1000.0;
        let parts = assembly
            .parts
            .into_iter()
            .map(|part| ConstructionPart {
                glyph: Glyph::new_from_id(&self.face, part.glyph_id, size).unwrap(),
                size: scale * part.full_advance as f32,
                max_start_overlap: scale * part.start_connector_length as f32,
                max_end_overlap: scale * part.end_connector_length as f32,
                is_extender: part.part_flags.extender(),
            })
            .collect();
        Construction { min_overlap, parts }
    }
}

impl<'a, R: opentype::OpenTypeRenderer> common::Font<Glyph<R>> for Font<'a, R> {
    fn get_glyph(&self, ch: char, size: f32, style: FontStyle) -> Option<Glyph<R>> {
        Glyph::new(&self.face, ch, self.size_for_style(size, style))
    }

    fn get_larger_glyph(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
        include_italic_correction: bool,
    ) -> Option<Glyph<R>> {
        // TODO: Do not just get the second size, but the smallest
        // glyph which is larger than `display_operator_min_height`

        let glyph_id = self.face.glyph_index(ch)?;
        let construction = self
            .face
            .tables()
            .math?
            .variants?
            .vertical_constructions
            .get(glyph_id)?;

        let glyph_id = construction.variants.get(1)?.variant_glyph;

        let mut glyph = Glyph::new_from_id(&self.face, glyph_id, self.size_for_style(size, style))?;

        // TODO: The following is an ugly hack and most likely not correct
        // For example, the small Integral symbol has the same property. Maybe we have
        // to use the unicode math classes again

        // TODO: Most likely, the problem goes away once we implement subscript/superscript
        // kerning.

        // For large glyphs, the italic correction is already included in the advance width
        if include_italic_correction {
            glyph.italic_correction = 0.0;
        } else {
            glyph.advance -= glyph.italic_correction;
        }

        Some(glyph)
    }

    fn get_glyph_minwidth(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
        min_width: f32,
    ) -> Option<Glyph<R>> {
        let glyph_id = self.face.glyph_index(ch)?;
        let construction = self
            .face
            .tables()
            .math?
            .variants?
            .horizontal_constructions
            .get(glyph_id)?;
        let size = self.size_for_style(size, style);
        self.get_glyph_minsize(construction, size, min_width)
    }

    fn get_glyph_minheight(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
        min_height: f32,
    ) -> Option<Glyph<R>> {
        let glyph_id = self.face.glyph_index(ch)?;
        let construction = self
            .face
            .tables()
            .math?
            .variants?
            .vertical_constructions
            .get(glyph_id)?;
        let size = self.size_for_style(size, style);
        self.get_glyph_minsize(construction, size, min_height)
    }

    fn get_glyph_hor_construction(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
    ) -> Option<common::Construction<Glyph<R>>> {
        let glyph_id = self.face.glyph_index(ch)?;
        let variants = self.face.tables().math?.variants?;
        let min_overlap = variants.min_connector_overlap;
        let construction = variants.horizontal_constructions.get(glyph_id)?;
        let size = self.size_for_style(size, style);
        let scale = size / 1000.0;
        Some(self.get_glyph_construction(min_overlap as f32 * scale, construction.assembly?, size))
    }

    fn get_glyph_vert_construction(
        &self,
        ch: char,
        size: f32,
        style: FontStyle,
    ) -> Option<common::Construction<Glyph<R>>> {
        let glyph_id = self.face.glyph_index(ch)?;
        let variants = self.face.tables().math?.variants?;
        let min_overlap = variants.min_connector_overlap;
        let construction = variants.vertical_constructions.get(glyph_id)?;
        let size = self.size_for_style(size, style);
        let scale = size / 1000.0;
        Some(self.get_glyph_construction(min_overlap as f32 * scale, construction.assembly?, size))
    }

    fn calculate_script_params(
        &self,
        size: f32,
        style: FontStyle,
        cramped: bool,
    ) -> common::font_params::ScriptParams {
        let constants = self.face.tables().math.unwrap().constants.unwrap();

        let glyph_size = self.size_for_style(size, style);
        let scale = |v: &ttf_parser::math::MathValue| v.value as f32 * glyph_size / 1000.0;

        let subscript = common::font_params::SubScriptParams {
            shift_down: scale(&constants.subscript_shift_down()),
            top_max: scale(&constants.subscript_top_max()),
            baseline_drop_min: scale(&constants.subscript_baseline_drop_min()),
        };

        let superscript = common::font_params::SuperScriptParams {
            shift_up: match cramped {
                true => scale(&constants.superscript_shift_up_cramped()),
                false => scale(&constants.superscript_shift_up()),
            },
            bottom_min: scale(&constants.superscript_bottom_min()),
            baseline_drop_max: scale(&constants.superscript_baseline_drop_max()),
        };

        common::font_params::ScriptParams {
            subscript,
            superscript,
            sub_super_gap_min: scale(&constants.sub_superscript_gap_min()),
            super_bottom_max_with_subscript: scale(
                &constants.superscript_bottom_max_with_subscript(),
            ),
        }
    }

    fn calculate_general_params(
        &self,
        size: f32,
        style: FontStyle,
        _cramped: bool,
    ) -> common::font_params::GeneralParams {
        let constants = self.face.tables().math.unwrap().constants.unwrap();

        let glyph_size = self.size_for_style(size, style);
        let scale = |v: &ttf_parser::math::MathValue| v.value as f32 * glyph_size / 1000.0;

        common::font_params::GeneralParams {
            axis_height: scale(&constants.axis_height()),
        }
    }

    fn calculate_fraction_params(
        &self,
        size: f32,
        style: FontStyle,
        _cramped: bool,
    ) -> common::font_params::FractionParams {
        let constants = self.face.tables().math.unwrap().constants.unwrap();

        let glyph_size = self.size_for_style(size, style);
        let scale = |v: &ttf_parser::math::MathValue| v.value as f32 * glyph_size / 1000.0;

        let (numerator, denominator) = match style {
            FontStyle::Display => {
                let numerator = common::font_params::FractionPartParams {
                    shift: scale(&constants.fraction_numerator_display_style_shift_up()),
                    gap_min: scale(&constants.fraction_num_display_style_gap_min()),
                };
                let denominator = common::font_params::FractionPartParams {
                    shift: scale(&constants.fraction_denominator_display_style_shift_down()),
                    gap_min: scale(&constants.fraction_denom_display_style_gap_min()),
                };

                (numerator, denominator)
            }
            _ => {
                let numerator = common::font_params::FractionPartParams {
                    shift: scale(&constants.fraction_numerator_shift_up()),
                    gap_min: scale(&constants.fraction_numerator_gap_min()),
                };
                let denominator = common::font_params::FractionPartParams {
                    shift: scale(&constants.fraction_denominator_shift_down()),
                    gap_min: scale(&constants.fraction_denominator_gap_min()),
                };

                (numerator, denominator)
            }
        };

        common::font_params::FractionParams {
            numerator,
            denominator,
            rule_thickness: scale(&constants.fraction_rule_thickness()),
        }
    }

    fn get_fallback_glyph(&self, size: f32, style: FontStyle) -> Glyph<R> {
        // TODO: Better character?
        Self::get_glyph(self, '?', size, style).unwrap()
    }
}

pub struct FontBackend<'a> {
    font: Font<'a, TinySkiaRenderer>,
}

impl<'a> common::FontBackend for FontBackend<'a> {
    type Glyph = Glyph<TinySkiaRenderer>;

    fn get_font(&self, _family: common::Family) -> &dyn common::Font<Self::Glyph> {
        &self.font
    }
}

impl Default for FontBackend<'static> {
    fn default() -> Self {
        let face =
            ttf_parser::Face::parse(include_bytes!("../data/NewCMMath-Regular.otf"), 0).unwrap();
        let font = Font {
            face,
            _phantom: Default::default(),
        };
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
        color: Color,
    ) {
        const DPI: f32 = 96.0;
        let scale = DPI / 72.0;

        let paint = {
            let mut paint = tiny_skia::Paint::default();
            match color {
                Color::Normal => {}
                Color::Error => paint.set_color_rgba8(255, 0, 0, 255),
            }
            paint
        };

        let ts = tiny_skia::Transform::from_translate(x0, y0)
            .post_scale(scale, -scale)
            .post_translate(0.0, self.pixmap.height() as f32);
        self.pixmap
            .fill_path(&glyph.path, &paint, tiny_skia::FillRule::EvenOdd, ts, None);
    }

    fn render_box(&mut self, x0: f32, y0: f32, width: f32, height: f32) {
        const DPI: f32 = 96.0;
        let scale = DPI / 72.0;

        let ts = tiny_skia::Transform::from_scale(scale, -scale)
            .post_translate(0.0, self.pixmap.height() as f32);

        let rect = tiny_skia::Rect::from_ltrb(x0, y0, x0 + width, y0 + height).unwrap();
        self.pixmap
            .fill_rect(rect, &tiny_skia::Paint::default(), ts, None);
    }
}

#[derive(Clone)]
pub struct TinySkiaRenderer;

impl opentype::OpenTypeRenderer for TinySkiaRenderer {
    type Path = tiny_skia::Path;
    type OutlineBuilder = OutlineBuilder;
}
