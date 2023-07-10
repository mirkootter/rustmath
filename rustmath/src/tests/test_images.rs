fn test_png_image(png: &[u8]) {
    let source = crate::get_source_from_png_metadata(png).unwrap();
    let encoded = crate::encode_png(&source, true).unwrap();

    assert_eq!(png, &encoded);
}

#[test]
fn test_images() {
    for (id, img) in super::TEST_IMAGES.into_iter().enumerate() {
        eprintln!("Testing image {}", id);
        test_png_image(img);
    }
}
