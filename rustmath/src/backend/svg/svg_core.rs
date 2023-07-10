use super::super::opentype;

enum PathOp {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    QuadTo(f32, f32, f32, f32),
    CurveTo(f32, f32, f32, f32, f32, f32),
    Close,
}

impl PathOp {
    fn write(&self, scale: f32, out: &mut impl core::fmt::Write) -> core::fmt::Result {
        match self {
            PathOp::MoveTo(x, y) => write!(out, "M {} {}", x * scale, y * scale),
            PathOp::LineTo(x, y) => write!(out, "L {} {}", x * scale, y * scale),
            PathOp::QuadTo(x1, y1, x2, y2) => write!(
                out,
                "Q {} {} {} {}",
                x1 * scale,
                y1 * scale,
                x2 * scale,
                y2 * scale
            ),
            PathOp::CurveTo(x1, y1, x2, y2, x3, y3) => {
                write!(
                    out,
                    "C {} {} {} {} {} {}",
                    x1 * scale,
                    y1 * scale,
                    x2 * scale,
                    y2 * scale,
                    x3 * scale,
                    y3 * scale
                )
            }
            PathOp::Close => out.write_char('z'),
        }
    }
}

#[derive(Default)]
pub struct OutlineBuilder {
    ops: Vec<PathOp>,
}

impl opentype::OutlineBuilder<String> for OutlineBuilder {
    fn finish(self, scale: f32) -> String {
        use std::fmt::Write;

        let mut result = String::new();
        let mut first = true;
        for op in self.ops {
            if first {
                first = false;
                let _ = result.write_char(' ');
            }
            let _ = op.write(scale, &mut result);
        }
        result
    }
}

impl ttf_parser::OutlineBuilder for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.ops.push(PathOp::MoveTo(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.ops.push(PathOp::LineTo(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.ops.push(PathOp::QuadTo(x1, y1, x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.ops.push(PathOp::CurveTo(x1, y1, x2, y2, x, y));
    }

    fn close(&mut self) {
        self.ops.push(PathOp::Close);
    }
}

#[derive(Clone)]
struct Translation(f32, f32);

impl core::fmt::Display for Translation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.1 != 0. {
            return write!(f, " transform=\"translate({}, {})\"", self.0, self.1);
        } else if self.0 != 0. {
            return write!(f, " transform=\"translate({})\"", self.0);
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Color(&'static str);

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        write!(f, " fill=\"{}\"", self.0)
    }
}

impl From<crate::common::Color> for Color {
    fn from(value: crate::common::Color) -> Self {
        match value {
            crate::common::Color::Normal => Self(""),
            crate::common::Color::Error => Self("red"),
        }
    }
}

#[derive(Clone)]
enum Element {
    Rect {
        x0: f32,
        y0: f32,
        width: f32,
        height: f32,
    },
    Path(Translation, String, Color),
}

impl Element {
    pub fn write(&self, out: &mut impl core::fmt::Write) -> core::fmt::Result {
        match self {
            Element::Rect {
                x0,
                y0,
                width,
                height,
            } => writeln!(
                out,
                "    <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />",
                x0, y0, width, height
            ),
            Element::Path(transform, data, color) => {
                writeln!(out, "    <path{}{} d=\"{}\" />", transform, color, data)
            }
        }
    }
}

#[derive(Clone)]
pub struct Image {
    width: f32,
    height: f32,
    elements: Vec<Element>,
}

impl Image {
    pub fn new(width: f32, height: f32) -> Self {
        let elements = Vec::new();
        Image {
            width,
            height,
            elements,
        }
    }

    pub fn draw_rect(&mut self, x0: f32, y0: f32, width: f32, height: f32) {
        self.elements.push(Element::Rect {
            x0,
            y0,
            width,
            height,
        })
    }

    pub fn draw_path(
        &mut self,
        translate_x: f32,
        translate_y: f32,
        data: String,
        color: crate::common::Color,
    ) {
        self.elements.push(Element::Path(
            Translation(translate_x, translate_y),
            data,
            color.into(),
        ));
    }

    pub fn write(
        &self,
        metadata: &[(&str, &str)],
        out: &mut impl core::fmt::Write,
    ) -> core::fmt::Result {
        writeln!(
            out,
            "<svg version=\"1.1\" width=\"{}pt\" height=\"{}pt\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            self.width, self.height, self.width, self.height
        )?;
        for (meta_id, meta_value) in metadata {
            write!(out, "  <metadata id=\"{}\">", meta_id)?;
            super::xml::write_escaped_string(meta_value, out)?;
            writeln!(out, "</metadata>")?;
        }
        writeln!(
            out,
            "  <g transform=\"translate(0.0,{}) scale(1.0,-1.0)\">",
            self.height
        )?;
        for elem in &self.elements {
            elem.write(out)?;
        }
        writeln!(out, "  </g>")?;
        writeln!(out, "</svg>")
    }
}
