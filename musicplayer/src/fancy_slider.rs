use fltk::app;
use fltk::frame::*;
use fltk::image::*;
use plotters::prelude::*;

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
    pub fn new() -> Self {
        let mut f = FancySlider {
            line: Frame::new(50, 163, 300, 5, ""),
            circ: Frame::new(40, 150, 50, 50, ""),
        };
        let circle = MyCircle::new();
        f.circ.set_image(Some(circle.image));
        f.line.set_frame(FrameType::RFlatBox);
        f.line.set_color(fltk::enums::Color::White);
        let mut circle = f.circ.clone(); 
        f.circ.handle(Box::new(move |ev| match ev {
            Event::Push => true,
            Event::Drag => {
                let (x, _y) = app::event_coords();
                if x > 45 && x < 350 {
                    circle.resize(x - 15, 150, 50, 50);
                }
                app::redraw();
                true
            },
            _ => false,
        }));
        f
    }
    pub fn value(&self) -> f32 {
        self.circ.x() as f32 / 50.0
    }
}

