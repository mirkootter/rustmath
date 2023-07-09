pub mod backend;
pub mod common;
pub mod layout;
pub mod mathlist;
pub mod parser;

#[cfg(test)]
mod tests;

pub fn render_layout(
    fb: backend::FontBackend,
    node: layout::Node<<backend::FontBackend<'_> as common::FontBackend>::Glyph>,
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

pub fn encode_png(src: &str, include_meta_data: bool) -> Option<Vec<u8>> {
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
        if include_meta_data {
            encoder
                .add_itxt_chunk("source".to_owned(), "rustmath".to_owned())
                .ok()?;
            encoder
                .add_itxt_chunk("rustmath_src".to_owned(), src.to_owned())
                .ok()?;
        }
        let mut writer = encoder.write_header().ok()?;
        writer.write_image_data(&data).ok()?;
    }

    Some(result)
}

pub fn get_source_from_png_metadata(png: &[u8]) -> Option<String> {
    let decoder = png::Decoder::new(png);
    let reader = decoder.read_info().ok()?;

    let mut source = None;
    let mut rustmath_source = None;

    for text_chunk in &reader.info().utf8_text {
        match &text_chunk.keyword as &str {
            "source" => source = Some(text_chunk.get_text().ok()?),
            "rustmath_src" => rustmath_source = Some(text_chunk.get_text().ok()?),
            _ => {}
        }
    }

    let source = source?;

    if &source != "rustmath" {
        return None;
    }

    rustmath_source
}

pub fn save_png(src: &str, include_meta_data: bool, filename: &str) {
    let data = encode_png(src, include_meta_data).unwrap();
    std::fs::write(filename, data).unwrap();
}
