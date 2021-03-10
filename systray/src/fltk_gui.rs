use fltk::*;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref FLTK_WIN_SHOWN: AtomicBool = AtomicBool::new(false);
}   

pub fn fltk_gui() {
    if FLTK_WIN_SHOWN.load(Ordering::Relaxed) {
        return;
    }
    let mut win = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::new(10, 10, 380, 200, "");
    frame.set_frame(FrameType::EngravedBox);
    let mut but = button::Button::new(160, 220, 80, 40, "Click me!");
    win.end();
    win.show();
    win.set_callback2(|w| {
        if app::event() == Event::Hide || app::event() == Event::Close {
            FLTK_WIN_SHOWN.store(false, Ordering::Relaxed);
            w.hide();
        }
    });
    but.set_callback(move || frame.set_label("Hello world!"));
    FLTK_WIN_SHOWN.store(true, Ordering::Relaxed);
}
