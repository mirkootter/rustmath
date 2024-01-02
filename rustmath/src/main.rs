use rustmath::{
    backend::{
        opentype::{self, OpenTypeRenderer},
        svg::SvgRenderer,
    },
    common::{Color, FontBackend, FontStyle, Glyph, Renderer},
};

fn main() {
    let fb = opentype::FontBackend::<SvgRenderer>::default();

    let font = fb.get_font(rustmath::common::Family::Italic);
    let sum_glyph = font
        .get_larger_glyph('∑', 36., FontStyle::Display, false)
        .unwrap();
    let x_glyph = font.get_glyph('𝑥', 36., FontStyle::Display).unwrap();
    let n_glyph = font.get_glyph('𝑛', 36., FontStyle::SuperScript).unwrap();
    let script_params = font.calculate_script_params(36., FontStyle::Display, false);

    let mut canvas = SvgRenderer::new(150., 100.);
    let mut renderer = opentype::Renderer::new(&mut canvas, fb);

    let mut x = 20.;
    let mut y = 20.;

    renderer.render_glyph(&sum_glyph, x, y, Color::Normal);
    x += sum_glyph.advance();

    renderer.render_glyph(&x_glyph, x, y, Color::Normal);
    x += x_glyph.advance();

    y += script_params.superscript.shift_up;
    renderer.render_glyph(&n_glyph, x, y, Color::Normal);

    let svg = canvas.finish();
    let mut result = String::new();
    svg.write(&[], &mut result).unwrap();

    std::fs::write("test.svg", result).unwrap();
}
