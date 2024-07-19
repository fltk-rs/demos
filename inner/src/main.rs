#![forbid(unsafe_code)]
use fltk::{
    app,
    enums::{Event, Font},
    prelude::*,
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme};

fn main() {
    const STEP: i32 = 5;
    let mut window = crate::window();

    let inner = Window::default()
        .with_size(window.w() - STEP, window.h() - STEP)
        .center_of_parent();
    inner.end();
    window.end();
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
        .with_scheme(app::AppScheme::Base)
        .run()
        .unwrap();
}

fn window() -> Window {
    const NAME: &str = "FlRemote";
    let mut element = Window::default()
        .with_size(960, 540)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(false);
    element.set_xclass(NAME);
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            app::quit();
        }
    });
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::set_font(Font::Courier);
    element
}
