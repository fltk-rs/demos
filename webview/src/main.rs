use fltk::{enums::Color, prelude::*, *};
use webview_official_sys as wv;

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");

    let mut inner_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    inner_win.set_color(Color::White);
    inner_win.end();
    win.end();
    win.show();

    let w;
    unsafe {
        use std::os::raw;
        w = wv::webview_create(
            0,
            &mut inner_win.raw_handle() as *mut *mut raw::c_void as *mut raw::c_void,
        );
    }
    assert!(!w.is_null());

    unsafe {
        wv::webview_navigate(w, "https://google.com\0".as_ptr() as _);
    }

    app.run().unwrap();
    unsafe {
        wv::webview_destroy(w);
    }
}
