use fltk::{
    app,
    enums::Event,
    prelude::*,
    window,
    image::IcoImage
};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(730, 430)
        .with_label("Webview");
    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    win.make_resizable(true);
    win.set_icon(Some(icon));
    let mut wv_win = window::Window::default()
        .with_size(725, 425)
        .center_of_parent();
    win.end();
    win.show();

    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("https://google.com");

    // the webview handles the main loop
    app.run().unwrap();
}
