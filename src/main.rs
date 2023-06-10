mod layout;
mod mathlist;

fn main() {

    let list = {
        let mut builder = mathlist::Builder::default();
        builder.add_ord('𝑥');
        builder.add_bin('+');
        builder.add_ord('𝑦');
        builder.add_rel('=');
        builder.add_ord('2');
        
        builder.finish()
    };

    let face = ttf_parser::Face::parse(include_bytes!("../data/NewCMMath-Regular.otf"), 0).unwrap();
    let node = list.translate(&face, 36.0, mathlist::Style::Display);

    let mut pixmap = tiny_skia::Pixmap::new(512, 512).unwrap();
    node.render(&mut pixmap, 50.0, 200.0 - 128.0);

    pixmap.save_png("image.png").unwrap();
}
