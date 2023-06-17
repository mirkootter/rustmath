fn main() {
    let src = "\\int\\sum x+\\mathop{log}y=2\\mitalpha";
    rustmath::save_png(src, "image.png");
}
