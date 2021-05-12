use fltk::{enums::Color, prelude::*, *};
use webview_official_sys as wv;

#[cfg(target_os = "linux")]
use gdk_x11_sys as gdk;

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
    #[cfg(not(target_os = "linux"))]
    unsafe {
        use std::os::raw;
        w = wv::webview_create(
            0,
            &mut inner_win.raw_handle() as *mut *mut raw::c_void as *mut raw::c_void,
        );
    }
    #[cfg(target_os = "linux")]
    unsafe {
        gtk_sys::gtk_init(&mut 0, std::ptr::null_mut());
        let mn = gdk_sys::gdk_display_manager_get();
        // fltk::app::display() doesn't work for some reason
        let display = gdk_sys::gdk_display_manager_open_display(mn, concat!(env!("DISPLAY"), "\0").as_ptr() as _);
        let gdkwin = gdk::gdk_x11_window_foreign_new_for_display(
            display as _,
            inner_win.raw_handle(),
        );
        let gtkwid = gtk_sys::gtk_window_new(0);
        gtk_sys::gtk_widget_set_window(gtkwid, gdkwin);
        w = wv::webview_create(0, gtkwid as _);
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
