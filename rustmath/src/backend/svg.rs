use super::opentype;
use crate::common::Color;

mod svg_core;
mod xml;

#[derive(Clone)]
pub struct SvgRenderer {
    image: svg_core::Image,
}

impl opentype::OpenTypeRenderer for SvgRenderer {
    type Path = String;
    type OutlineBuilder = svg_core::OutlineBuilder;

    type Image = svg_core::Image;

    fn new(width: f32, height: f32) -> Self {
        let image = svg_core::Image::new(width, height);
        Self { image }
    }

    fn render_path(&mut self, x0: f32, y0: f32, path: &Self::Path, color: Color) {
        self.image.draw_path(x0, y0, path.clone(), color);
    }

    fn render_box(&mut self, x0: f32, y0: f32, width: f32, height: f32) {
        self.image.draw_rect(x0, y0, width, height);
    }

    fn finish(self) -> Self::Image {
        self.image
    }
}
