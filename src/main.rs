mod backend;
mod common;
mod layout;
mod mathlist;
mod parser;

fn main() {
    let list: mathlist::MathList<backend::Glyph> =
        parser::Node::parse("\\int\\sum x+\\mathop{log}y=2")
            .unwrap()
            .to_mathlist()
            .unwrap();

    let mut pixmap = tiny_skia::Pixmap::new(512, 512).unwrap();
    let mut renderer = backend::Renderer::new(&mut pixmap);
    let node = list.translate(renderer.font_backend(), 36.0, mathlist::Style::Display);

    node.render(&mut renderer, 50.0, 200.0 - 128.0);

    pixmap.save_png("image.png").unwrap();
}
