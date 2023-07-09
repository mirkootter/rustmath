pub trait OutlineBuilder<Path>: ttf_parser::OutlineBuilder + Default {
    fn finish(self, scale: f32) -> Path;
}

pub trait OpenTypeRenderer: Clone {
    type Path: Clone;
    type OutlineBuilder: OutlineBuilder<Self::Path>;
    type Image;

    fn new(width: f32, height: f32) -> Self;
    fn render_path(&mut self, x0: f32, y0: f32, path: &Self::Path, color: crate::common::Color);
    fn render_box(&mut self, x0: f32, y0: f32, width: f32, height: f32);

    fn finish(self) -> Self::Image;
}
