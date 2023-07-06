pub mod backend;
pub mod common;
pub mod layout;
pub mod mathlist;
pub mod parser;

pub fn render_layout(
    fb: backend::FontBackend,
    node: layout::Node<backend::Glyph>,
) -> Option<tiny_skia::Pixmap> {
    const DPI: f32 = 96.0;
    let scale = DPI / 72.0;
    let x_padding = 10.0; // padding in pt
    let y_padding = 5.0; // padding in pt

    let width = node.advance(false) + 2.0 * x_padding;
    let height = node.height(false) + node.depth() + 2.0 * y_padding;

    let mut pixmap = tiny_skia::Pixmap::new(
        (width * scale).round() as u32,
        (height * scale).round() as u32,
    )
    .unwrap();
    let mut renderer = backend::Renderer::new(&mut pixmap, fb);

    node.render(&mut renderer, x_padding, y_padding + node.depth());
    Some(pixmap)
}

pub fn render_string(src: &str) -> Option<tiny_skia::Pixmap> {
    let list = parser::parse(src)?;

    let fb = backend::FontBackend::default();
    let node = list.translate(&fb, 36.0, mathlist::Style::Display);

    render_layout(fb, node)
}

pub fn encode_png(src: &str) -> Option<Vec<u8>> {
    let pixmap = render_string(src)?;

    let mut data = Vec::new();
    data.reserve_exact(pixmap.data().len());

    for pixel in pixmap.pixels() {
        let c = pixel.demultiply();
        data.push(c.red());
        data.push(c.green());
        data.push(c.blue());
        data.push(c.alpha());
    }

    let mut result = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut result, pixmap.width(), pixmap.height());
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().ok()?;
        writer.write_image_data(&data).ok()?;
    }

    Some(result)
}

pub fn save_png(src: &str, filename: &str) {
    let data = encode_png(src).unwrap();
    std::fs::write(filename, data).unwrap();
}
