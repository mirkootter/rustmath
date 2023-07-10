use usvg::TreeParsing;

fn test_image(png_image: &[u8]) {
    assert!(png_image.starts_with(b"\x89PNG"));
    let source = crate::get_source_from_metadata(png_image).unwrap();

    assert!(!source.is_empty());

    let svg = crate::render_svg(&source, true).unwrap();
    let source_from_metadata = crate::get_source_from_metadata(svg.as_bytes()).unwrap();

    assert_eq!(source, source_from_metadata);

    let rendered = render_svg(&svg);
    let expected = load_png(png_image);

    assert_eq!(rendered.len(), expected.len());
    for (rendered, expected) in rendered.into_iter().zip(expected.into_iter()) {
        let diff = (rendered as i32) - (expected as i32);
        if diff.abs() > 128 {
            panic!("Difference too large: {}", diff);
        }
    }
}

fn render_svg(svg: &str) -> Vec<u8> {
    let tree = usvg::Tree::from_str(svg, &Default::default()).unwrap();
    let tree = resvg::Tree::from_usvg(&tree);

    let pixmap_size = tree.size.to_int_size();
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    tree.render(Default::default(), &mut pixmap.as_mut());

    pixmap.take()
}

fn load_png(png: &[u8]) -> Vec<u8> {
    let decoder = png::Decoder::new(png);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0u8; reader.output_buffer_size()];

    let info = reader.next_frame(&mut buf).unwrap();
    buf.resize(info.buffer_size(), 0);

    buf
}

#[test]
fn test_metadata_for_images() {
    for (id, img) in super::TEST_IMAGES.into_iter().enumerate() {
        eprintln!("Testing image {}", id);
        test_image(img);
    }
}
