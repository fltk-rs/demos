#![allow(dead_code)]
use cairo::Context;
use fltk::{enums::*, prelude::*, draw::Rect, *};
use std::{cell::RefCell, rc::Rc};

fn draw_box_with_alpha(ctx: Context, rect: &Rect, color: Color, alpha: u8) {
    let (r, g, b) = color.to_rgb();
    ctx.save().unwrap();
    ctx.move_to(rect.x as f64, rect.y as f64);
    ctx.line_to((rect.x + rect.w) as f64, rect.y as f64);
    ctx.line_to((rect.x + rect.w) as f64, (rect.y + rect.h) as f64);
    ctx.line_to(rect.x as f64, (rect.y + rect.h) as f64);
    ctx.close_path();
    ctx.set_source_rgba(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0, alpha as f64 / 255.0);
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}

#[derive(Clone)]
struct CairoWidget {
    frm: frame::Frame,
    alpha: Rc<RefCell<u8>>,
}

impl CairoWidget {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut frm = frame::Frame::new(x, y, w, h, None).with_label(label);
        frm.super_draw_first(false); // required for windows
        let alpha = Rc::from(RefCell::from(255));
        frm.draw({
            let alpha = alpha.clone();
            move |w| {
                let cc = app::cairo::cc();
                let ctx = unsafe { Context::from_raw_none(cc as _) };
                draw_box_with_alpha(ctx, &Rect {x : w.x(), y : w.y(), w : w.w(), h : w.h()}, w.color(), *alpha.borrow());
                unsafe {
                    app::cairo::flush(cc); // required for windows
                }
            }
        });
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

fltk::widget_extends!(CairoWidget, frame::Frame, frm);

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    app::cairo::set_autolink_context(true);
    let mut win = window::Window::new(100, 100, 400, 300, "Cairo");
    win.set_color(Color::White);
    win.make_resizable(true);
    
    let mut box1 = CairoWidget::new(0, 0, 100, 100, "Box1");
    box1.set_color(Color::from_rgb(0, 0, 255));
    box1.set_alpha(100);
    let mut box2 = CairoWidget::new(75, 75, 100, 100, "Box2");
    box2.set_color(Color::Red);
    box2.set_alpha(100);
    let mut box3 = CairoWidget::new(150, 150, 100, 100, "Box3");
    box3.set_color(Color::Green);
    box3.set_alpha(100);

    win.end();
    win.show();

    app.run().unwrap();
}
