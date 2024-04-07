use fltk::{enums::Color, prelude::*, *};

fn main() {
    let mut win = window::Window::new(100, 100, 800, 600, "Terminal");

    // Create inner window to act as embedded terminal
    let mut inner = window::Window::new(10, 10, 780, 520, "");
    inner.end();
    inner.set_color(Color::Black);

    win.end();
    win.make_resizable(true);
    win.show();

    std::process::Command::new("xterm")
        .args([
            "-into",
            &format!("{}", inner.raw_handle()),
            "-bg",
            "black",
            "-fg",
            "white",
            "-fa",
            "'Monospace'",
            "-fs",
            "10",
        ])
        .spawn()
        .unwrap();

    app::App::default()
        .with_scheme(app::AppScheme::Gtk)
        .run()
        .unwrap();
}
