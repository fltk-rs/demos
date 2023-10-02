use fltk::{
    enums::Color,
    prelude::*,
    *,
    image::IcoImage
};

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Terminal");

    // Create inner window to act as embedded terminal
    let mut xterm_win = window::Window::new(10, 10, 780, 520, "");
    xterm_win.end();
    xterm_win.set_color(Color::Black);

    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    win.make_resizable(true);
    win.set_icon(Some(icon));
    win.end();
    win.show();
    win.make_resizable(true);

    let mut handle = xterm_win.raw_handle();
    std::process::Command::new("xterm")
        .args(&["-into", &format!("{}", handle), "-bg", "black", "-fg", "white", "-fa", "'Monospace'", "-fs", "10"])
        .spawn()
        .unwrap();

    app.run().unwrap();
}
