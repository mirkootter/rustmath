pub mod backend;
pub mod common;
pub mod layout;
pub mod mathlist;
pub mod parser;

#[cfg(test)]
mod tests;

pub fn render_layout<R: backend::opentype::OpenTypeRenderer>(
    fb: backend::opentype::FontBackend<R>,
    node: layout::Node<<backend::opentype::FontBackend<'_, R> as common::FontBackend>::Glyph>,
) -> Option<R::Image> {
    let x_padding = 10.0; // padding in pt
    let y_padding = 5.0; // padding in pt

    let width = node.advance(false) + 2.0 * x_padding;
    let height = node.height(false) + node.depth() + 2.0 * y_padding;

    let mut canvas = R::new(width, height);
    let mut renderer = backend::opentype::Renderer::new(&mut canvas, fb);

    node.render(&mut renderer, x_padding, y_padding + node.depth());
    Some(canvas.finish())
}

#[cfg(feature = "tiny-skia")]
pub fn render_string(src: &str) -> Option<tiny_skia::Pixmap> {
    use backend::raster::TinySkiaRenderer;

    let list = parser::parse(src)?;

    let fb = backend::opentype::FontBackend::<TinySkiaRenderer>::default();
    let node = list.translate(&fb, 36.0, mathlist::Style::Display);

    render_layout(fb, node)
}

#[cfg(feature = "png")]
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

#[cfg(feature = "png")]
fn get_source_from_png_metadata(png: &[u8]) -> Option<String> {
    if !png.starts_with(b"\x89PNG") {
        return None;
    }

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

#[cfg(feature = "png")]
pub fn save_png(src: &str, include_meta_data: bool, filename: &str) {
    let data = encode_png(src, include_meta_data).unwrap();
    std::fs::write(filename, data).unwrap();
}

#[cfg(feature = "svg")]
pub fn render_svg(src: &str, include_meta_data: bool) -> Option<String> {
    use backend::svg::SvgRenderer;

    let list = parser::parse(src)?;

    let fb = backend::opentype::FontBackend::<SvgRenderer>::default();
    let node = list.translate(&fb, 36.0, mathlist::Style::Display);

    let image = render_layout(fb, node)?;

    let mut result = String::new();
    if include_meta_data {
        let metadata: &[(&str, &str)] = &[("source", "rustmath"), ("rustmath_src", src)];
        image.write(metadata, &mut result).ok()?;
    } else {
        image.write(&[], &mut result).ok()?;
    }

    Some(result)
}

#[cfg(feature = "svg")]
fn get_source_from_svg_metadata(png: &[u8]) -> Option<String> {
    let s = core::str::from_utf8(png).ok()?;
    let metadata = backend::svg::parse_metadata(s)?;

    let mut source = None;
    let mut rustmath_source = None;

    for (keyword, value) in metadata {
        match keyword {
            "source" => source = Some(value),
            "rustmath_src" => rustmath_source = Some(value),
            _ => {}
        }
    }

    let source = source?;

    if &source != "rustmath" {
        return None;
    }

    rustmath_source
}

pub fn get_source_from_metadata(_data: &[u8]) -> Option<String> {
    #[cfg(feature = "png")]
    {
        let result = get_source_from_png_metadata(_data);
        if result.is_some() {
            return result;
        }
    }

    #[cfg(feature = "svg")]
    {
        let result = get_source_from_svg_metadata(_data);
        if result.is_some() {
            return result;
        }
    }

    None
}
