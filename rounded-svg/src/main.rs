use fltk::{enums::*, prelude::*, *};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use svg::node::element::Rectangle;
use svg::Document;

struct RoundedImageDisplay {
    frame_: frame::Frame,
    // wrap the following in an Rc RefCell since we need to mutate them after passing into callbacks
    bordercolor_: Rc<RefCell<[u8; 3]>>,
    radius_: Rc<RefCell<i32>>,
}

impl RoundedImageDisplay {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: Option<&'static str>) -> Self {
        let mut frame_ = frame::Frame::new(x, y, w, h, title);
        let radius_ = 20;
        let bordercolor_ = [0x80, 0x80, 0x80];
        frame_.set_frame(FrameType::BorderBox);
        let radius_ = Rc::from(RefCell::from(radius_));
        let bordercolor_ = Rc::from(RefCell::from(bordercolor_));
        frame_.draw({
            let radius_ = radius_.clone();
            let bordercolor_ = bordercolor_.clone();
            move |f| {
                let radius_ = radius_.borrow();
                let bordercolor_ = bordercolor_.borrow();

                let rect = Rectangle::new()
                    .set("x", 0 - *radius_ / 2)
                    .set("y", 0 - *radius_ / 2)
                    .set("rx", *radius_)
                    .set("ry", *radius_)
                    .set("width", f.w() + *radius_)
                    .set("height", f.h() + *radius_)
                    .set("fill", "none")
                    .set(
                        "stroke",
                        format!(
                            "rgb({},{},{})",
                            bordercolor_[0], bordercolor_[1], bordercolor_[2],
                        ),
                    )
                    .set("stroke-width", *radius_);

                let document = Document::new()
                    .set("viewBox", (0, 0, f.w(), f.h()))
                    .add(rect);
                let mut svg = image::SvgImage::from_data(&document.to_string()).unwrap();
                svg.draw(f.x(), f.y(), f.w(), f.h())
            }
        });
        Self {
            frame_,
            radius_,
            bordercolor_,
        }
    }

    pub fn bordercolor(&mut self, r: u8, g: u8, b: u8) {
        let mut bordercolor = self.bordercolor_.borrow_mut();
        bordercolor[0] = r;
        bordercolor[1] = g;
        bordercolor[2] = b;
        self.frame_.parent().unwrap().redraw();
    }

    pub fn radius(&mut self, val: i32) {
        *self.radius_.borrow_mut() = val;
        self.frame_.parent().unwrap().redraw();
    }
}

// impl Deref and DerefMut to emulate inheritance of RoundedImageDisplay from Frame (Fl_Box)
impl Deref for RoundedImageDisplay {
    type Target = frame::Frame;

    fn deref(&self) -> &Self::Target {
        &self.frame_
    }
}

impl DerefMut for RoundedImageDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frame_
    }
}

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);

    let border = [0x80, 0xa0, 0x80]; // gray green
    let mut win = window::Window::default()
        .with_size(1000, 800)
        .with_label("Rounded Corners");

    win.set_color(Color::from_rgb(border[0], border[1], border[2]));

    let jpg = image::JpegImage::load("../opengl/ex.jpg").expect("Failed to open jpg file");

    let mut rimage = RoundedImageDisplay::new(10, 10, jpg.w(), jpg.h(), None);
    rimage.bordercolor(border[0], border[1], border[2]);
    rimage.radius(50);
    rimage.set_image(Some(jpg));

    let mut slider = valuator::Slider::new(1000 - 50, 10, 20, 200, "border\nradius");
    slider.set_align(Align::Bottom);
    slider.set_bounds(0., 200.);
    slider.set_value(20.);
    slider.do_callback();
    slider.set_color(Color::from_rgb(
        (border[0] as f64 / 1.5) as u8,
        (border[1] as f64 / 1.5) as u8,
        (border[2] as f64 / 1.5) as u8,
    ));
    slider.set_callback(move |s| {
        rimage.radius(s.value() as i32);
    });

    win.end();
    win.show();
    a.run().unwrap();
}
