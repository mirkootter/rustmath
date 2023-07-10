fn test_metadata(png_image: &[u8]) {
    assert!(png_image.starts_with(b"\x89PNG"));
    let source = crate::get_source_from_metadata(png_image).unwrap();

    assert!(!source.is_empty());

    let svg = crate::render_svg(&source, true).unwrap();
    let source_from_metadata = crate::get_source_from_metadata(svg.as_bytes()).unwrap();

    assert_eq!(source, source_from_metadata);
}

#[test]
fn test_metadata_for_images() {
    for (id, img) in super::TEST_IMAGES.into_iter().enumerate() {
        eprintln!("Testing image {}", id);
        test_metadata(img);
    }
}
