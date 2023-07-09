pub trait OutlineBuilder<Path>: ttf_parser::OutlineBuilder + Default {
    fn finish(self, scale: f32) -> Path;
}

pub trait OpenTypeRenderer: Clone {
    type Path: Clone;
    type OutlineBuilder: OutlineBuilder<Self::Path>;
}
