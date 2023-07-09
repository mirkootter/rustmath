use super::opentype;
use crate::common::Color;

#[derive(Default)]
pub struct OutlineBuilder {
    path_builder: tiny_skia::PathBuilder,
}

impl opentype::OutlineBuilder<tiny_skia::Path> for OutlineBuilder {
    fn finish(self, scale: f32) -> tiny_skia::Path {
        let ts = tiny_skia::Transform::from_scale(scale, scale);
        self.path_builder.finish().unwrap().transform(ts).unwrap()
    }
}

impl ttf_parser::OutlineBuilder for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path_builder.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path_builder.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path_builder.quad_to(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path_builder.cubic_to(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        self.path_builder.close();
    }
}

#[derive(Clone)]
pub struct TinySkiaRenderer {
    pixmap: tiny_skia::Pixmap,
}

impl opentype::OpenTypeRenderer for TinySkiaRenderer {
    type Path = tiny_skia::Path;
    type OutlineBuilder = OutlineBuilder;

    type Image = tiny_skia::Pixmap;

    fn new(width: f32, height: f32) -> Self {
        const DPI: f32 = 96.0;
        let scale = DPI / 72.0;
        let pixmap = tiny_skia::Pixmap::new(
            (width * scale).round() as u32,
            (height * scale).round() as u32,
        )
        .unwrap();

        Self { pixmap }
    }

    fn render_path(&mut self, x0: f32, y0: f32, path: &Self::Path, color: crate::common::Color) {
        const DPI: f32 = 96.0;
        let scale = DPI / 72.0;

        let paint = {
            let mut paint = tiny_skia::Paint::default();
            match color {
                Color::Normal => {}
                Color::Error => paint.set_color_rgba8(255, 0, 0, 255),
            }
            paint
        };

        let ts = tiny_skia::Transform::from_translate(x0, y0)
            .post_scale(scale, -scale)
            .post_translate(0.0, self.pixmap.height() as f32);
        self.pixmap
            .fill_path(path, &paint, tiny_skia::FillRule::EvenOdd, ts, None);
    }

    fn render_box(&mut self, x0: f32, y0: f32, width: f32, height: f32) {
        const DPI: f32 = 96.0;
        let scale = DPI / 72.0;

        let ts = tiny_skia::Transform::from_scale(scale, -scale)
            .post_translate(0.0, self.pixmap.height() as f32);

        let rect = tiny_skia::Rect::from_ltrb(x0, y0, x0 + width, y0 + height).unwrap();
        self.pixmap
            .fill_rect(rect, &tiny_skia::Paint::default(), ts, None);
    }

    fn finish(self) -> Self::Image {
        self.pixmap
    }
}
