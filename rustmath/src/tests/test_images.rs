fn png_extract_source(png: &[u8]) -> String {
    let decoder = png::Decoder::new(png);
    let reader = decoder.read_info().unwrap();

    let mut source = None;
    let mut rustmath_source = None;

    for text_chunk in &reader.info().utf8_text {
        eprintln!("keyword {:?}", &text_chunk.keyword);
        match &text_chunk.keyword as &str {
            "source" => source = Some(text_chunk.get_text().unwrap()),
            "rustmath_src" => rustmath_source = Some(text_chunk.get_text().unwrap()),
            _ => {}
        }
    }

    assert_eq!(source, Some("rustmath".to_owned()));
    rustmath_source.unwrap()
}

fn test_png_image(png: &[u8]) {
    let source = png_extract_source(png);
    let encoded = crate::encode_png(&source, true).unwrap();

    assert_eq!(png, &encoded);
}

#[test]
fn test_images() {
    test_png_image(include_bytes!("../../../ci/test-images/parse_error.png"));
    test_png_image(include_bytes!("../../../ci/test-images/cauchy.png"));
}
