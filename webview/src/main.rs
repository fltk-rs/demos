#![forbid(unsafe_code)]
#[cfg(target_os = "linux")]
use fltk::{app, prelude::*, window};

fn main() {
    let mut win = window::Window::default()
        .with_size(730, 430)
        .with_label("Webview");
    win.make_resizable(true);
    let mut wv_win = window::Window::default()
        .with_size(725, 425)
        .center_of_parent();
    win.end();
    win.show();

    let wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("https://google.com");

    // the webview handles the main loop
    app::App::default().run().unwrap();
}
