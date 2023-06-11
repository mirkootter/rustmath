mod backend;
mod common;
mod layout;
mod mathlist;
mod parser;

fn main() {
    let list = {
        let log = {
            let mut builder = mathlist::Builder::default();
            builder.add_ord('l');
            builder.add_ord('o');
            builder.add_ord('g');

            builder.finish()
        };

        let mut builder = mathlist::Builder::default();
        builder.add_op('∫');
        builder.add_op('∑');
        builder.add_ord('𝑥');
        builder.add_bin('+');
        builder.add_list(mathlist::AtomType::Op, log);
        builder.add_ord('𝑦');
        builder.add_rel('=');
        builder.add_ord('2');

        builder.finish()
    };

    let mut pixmap = tiny_skia::Pixmap::new(512, 512).unwrap();
    let mut renderer = backend::Renderer::new(&mut pixmap);
    let node = list.translate(renderer.font_backend(), 36.0, mathlist::Style::Display);

    node.render(&mut renderer, 50.0, 200.0 - 128.0);

    pixmap.save_png("image.png").unwrap();
}
