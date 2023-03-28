use fltk::{
    enums::Color,
    prelude::*,
    *,
    image::IcoImage
};

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");
    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    win.make_resizable(true);
    win.set_icon(Some(icon));

    // Create inner window to act as embedded media player
    let mut mpv_win = window::Window::new(10, 10, 780, 520, "");
    mpv_win.end();
    mpv_win.set_color(Color::Black);

    win.end();
    win.show();
    win.make_resizable(true);

    let handle = mpv_win.raw_handle();
    std::process::Command::new("mpv")
        .args(&[&format!("--wid={}", handle as u64), "../libvlc/video.mp4"])
        .spawn()
        .unwrap();

    app.run().unwrap();
}
