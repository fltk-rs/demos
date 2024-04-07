#![forbid(unsafe_code)]
use fltk::{prelude::*, *};

fn main() {
    let mut window = window::Window::default()
        .with_label("flrdp")
        .with_size(1280, 720)
        .center_screen();

    let inner = window::Window::new(10, 10, 1260, 700, "");
    inner.end();
    window.end();
    window.make_resizable(true);
    window.show();

    std::process::Command::new("xfreerdp")
        .args([
            "+home-drive",
            "+clipboard",
            "+fonts",
            "/cert:ignore",
            "/v:127.0.0.1",
            &format!("/parent-window:{}", inner.raw_handle()),
        ])
        .spawn()
        .unwrap();

    app::App::default()
        .with_scheme(app::AppScheme::Plastic)
        .run()
        .unwrap();
}
