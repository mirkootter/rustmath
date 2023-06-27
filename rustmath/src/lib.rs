pub mod backend;
pub mod common;
pub mod layout;
pub mod mathlist;
pub mod parser;

fn render(src: &str) -> Option<tiny_skia::Pixmap> {
    let list = parser::parse(src)?;

    let fb = backend::FontBackend::default();
    let node = list.translate(&fb, 36.0, mathlist::Style::Display);

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

pub fn encode_png(src: &str) -> Option<Vec<u8>> {
    let pixmap = render(src)?;
    pixmap.encode_png().ok()
}

pub fn save_png(src: &str, filename: &str) {
    let pixmap = render(src).unwrap();
    pixmap.save_png(filename).unwrap();
}
