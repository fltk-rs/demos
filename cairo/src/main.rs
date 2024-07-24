#![allow(dead_code)]
#[cfg(target_os = "linux")]
use {
    cairo::Context,
    fltk::{enums::*, frame::Frame, image::SvgImage, prelude::*, *},
    std::{cell::RefCell, rc::Rc},
};

fn main() -> Result<(), FltkError> {
    let app = app::App::default().with_scheme(app::AppScheme::Base);
    let mut win = window::Window::default()
        .with_label("Demo: Cairo")
        .with_size(260, 260)
        .center_screen();
    {
        let mut box1 = CairoWidget::new(5, 5, 100, 100, "Box1");
        box1.set_color(Color::Red);
        box1.set_alpha(100);
        box1.frm.handle(crate::change);
        let mut box2 = CairoWidget::new(80, 80, 100, 100, "Box2");
        box2.set_color(Color::Yellow);
        box2.set_alpha(100);
        box2.frm.handle(crate::change);
        let mut box3 = CairoWidget::new(155, 155, 100, 100, "Box3");
        box3.set_color(Color::Green);
        box3.set_alpha(100);
        box3.frm.handle(crate::change);
    }
    win.end();
    win.set_color(Color::White);
    win.make_resizable(true);
    win.show();
    win.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    app::cairo::set_autolink_context(true);
    app.run();
}

fn draw_box_with_alpha(rect: &mut Frame) {
    let ctx = unsafe { Context::from_raw_none(app::cairo::cc() as _) };
    let (r, g, b) = rect.color().to_rgb();
    ctx.save().unwrap();
    ctx.move_to(rect.x() as f64, rect.y() as f64);
    ctx.line_to((rect.x() + rect.w()) as f64, rect.y() as f64);
    ctx.line_to((rect.x() + rect.w()) as f64, (rect.y() + rect.h()) as f64);
    ctx.line_to(rect.x() as f64, (rect.y() + rect.h()) as f64);
    ctx.close_path();
    ctx.set_source_rgba(
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
        100.0 / 255.0,
    );
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}

#[derive(Clone)]
struct CairoWidget {
    frm: Frame,
    alpha: Rc<RefCell<u8>>,
}

impl CairoWidget {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut frm = Frame::new(x, y, w, h, None).with_label(label);
        frm.super_draw_first(false); // required for windows
        let alpha = Rc::from(RefCell::from(255));
        frm.draw(draw_box_with_alpha);
        Self { frm, alpha }
    }

    pub fn set_alpha(&mut self, val: u8) {
        *self.alpha.borrow_mut() = val;
    }

    pub fn alpha(&self) -> u8 {
        *self.alpha.borrow()
    }

    pub fn draw<F: FnMut(&mut Self) + 'static>(&mut self, mut cb: F) {
        let mut frm = self.clone();
        let mut old_cb = unsafe { frm.draw_data().unwrap() };
        self.frm.draw(move |_| {
            old_cb();
            cb(&mut frm);
        });
    }
}

fltk::widget_extends!(CairoWidget, Frame, frm);

fn change(frame: &mut Frame, event: Event) -> bool {
    match event {
        Event::Released => {
            match frame.color() {
                Color::Red => frame.set_color(Color::DarkRed),
                Color::DarkRed => frame.set_color(Color::Red),
                Color::Yellow => frame.set_color(Color::DarkYellow),
                Color::DarkYellow => frame.set_color(Color::Yellow),
                Color::Green => frame.set_color(Color::DarkGreen),
                Color::DarkGreen => frame.set_color(Color::Green),
                _ => {}
            };
            app::redraw();
            true
        }
        Event::Enter => {
            frame.window().unwrap().set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            frame.window().unwrap().set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    }
}
