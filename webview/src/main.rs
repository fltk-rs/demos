use fltk::{app, enums::Event, prelude::*, window};

fn main() {
    let _app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.show();

    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("https://google.com");
    
    // the webview handles the main loop
    wv.run();
}
