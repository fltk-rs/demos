use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.show();
    let mut wv = fltk_webview::Webview::from(false, &mut wv_win);
    wv.navigate("http://wikipedia.com");
    app.run().unwrap();
}
