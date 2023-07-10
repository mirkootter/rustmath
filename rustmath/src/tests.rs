mod test_generated_sources;
mod test_images;
mod test_svg;

const TEST_IMAGES: [&'static [u8]; 8] = [
    include_bytes!("../../ci/test-images/basel.png"),
    include_bytes!("../../ci/test-images/cauchy.png"),
    include_bytes!("../../ci/test-images/euler.png"),
    include_bytes!("../../ci/test-images/euler-lagrange.png"),
    include_bytes!("../../ci/test-images/gamma.png"),
    include_bytes!("../../ci/test-images/minimal_surface.png"),
    include_bytes!("../../ci/test-images/parse_error.png"),
    include_bytes!("../../ci/test-images/stokes.png"),
];
