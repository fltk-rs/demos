#[cfg(target_os = "linux")]
use fltk::{enums::Color, prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");

    // Create inner window to act as embedded media player
    let mut inner = window::Window::new(10, 10, 780, 520, "");
    inner.end();
    inner.set_color(Color::Black);

    win.end();
    win.make_resizable(true);
    win.show();

    std::process::Command::new("mpv")
        .args([
            &format!("--wid={}", inner.raw_handle()),
            "../libvlc/video.mp4",
        ])
        .spawn()
        .unwrap();

    app.run().unwrap();
}
