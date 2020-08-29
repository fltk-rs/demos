use fltk::frame::*;
use fltk::image::*;
use plotters::prelude::*;
use std::ops::{Deref, DerefMut};

struct MyCircle {
    image: SvgImage,
}

impl MyCircle {
    pub fn new() -> MyCircle {
        let mut buf = String::from("");
        let mut root = SVGBackend::with_string(&mut buf, (300, 200));
        root.draw_circle(
            (100, 100),
            50,
            &Into::<ShapeStyle>::into(&RED).filled(),
            true,
        ).unwrap();
        drop(root);
        let mut img = SvgImage::from_data(&buf).unwrap();
        img.scale(50, 50, true, true);
        MyCircle {
            image: img,
        }
    }
}

pub struct FancySlider {
    line: Frame,
    circ: Frame,
}

impl FancySlider {
    pub fn new(x: i32, y: i32) -> Self {
        let mut f = FancySlider {
            line: Frame::new(x, y + 13, 300, 5, ""),
            circ: Frame::new(x - 10, y, 50, 50, ""),
        };
        let circle = MyCircle::new();
        f.circ.set_image(Some(circle.image));
        f.line.set_frame(FrameType::RFlatBox);
        f.line.set_color(fltk::enums::Color::White);
        f
    }
}

impl Deref for FancySlider {
    type Target = Frame;

    fn deref(&self) -> &Self::Target {
        &self.circ
    }
}

impl DerefMut for FancySlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.circ
    }
}