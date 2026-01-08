fn test_png_image(png: &[u8]) {
    let source = crate::get_source_from_png_metadata(png).unwrap();
    let encoded = crate::encode_png(&source, true).unwrap();

    assert_eq!(png, &encoded);
}

#[test]
fn test_images() {
    test_png_image(include_bytes!("../../../ci/test-images/basel.png"));
    test_png_image(include_bytes!("../../../ci/test-images/cauchy.png"));
    test_png_image(include_bytes!("../../../ci/test-images/euler.png"));
    test_png_image(include_bytes!("../../../ci/test-images/euler-lagrange.png"));
    test_png_image(include_bytes!("../../../ci/test-images/gamma.png"));
    test_png_image(include_bytes!(
        "../../../ci/test-images/minimal_surface.png"
    ));
    test_png_image(include_bytes!("../../../ci/test-images/parse_error.png"));
    test_png_image(include_bytes!("../../../ci/test-images/stokes.png"));
}
