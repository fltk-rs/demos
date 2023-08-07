use cairo::{Format, ImageSurface};
use fltk::{enums::*, prelude::*, *};

mod surface;

#[derive(Clone)]
struct CairoButton {
    btn: button::Button,
}

impl CairoButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut btn = button::Button::new(x, y, w, h, None).with_label(label);
        btn.super_draw(false);
        btn.draw(|w| {
            draw::draw_rect_fill(w.x(), w.y(), w.w(), w.h(), Color::White);
            let mut surface = ImageSurface::create(Format::ARgb32, w.w(), w.h())
                .expect("Couldn’t create surface");
            surface::draw_surface(&mut surface, w.w(), w.h());
            if !w.value() {
                cairo_blur::blur_image_surface(&mut surface, 20);
            }
            surface
                .with_data(|s| {
                    let mut img = image::RgbImage::new(s, w.w(), w.h(), ColorDepth::Rgba8).unwrap();
                    img.draw(w.x(), w.y(), w.w(), w.h());
                })
                .unwrap();
            draw::set_draw_color(Color::Black);
            draw::set_font(Font::Helvetica, app::font_size());
            if !w.value() {
                draw::draw_rbox(
                    w.x() + 1,
                    w.y() + 1,
                    w.w() - 6,
                    w.h() - 6,
                    15,
                    true,
                    Color::White,
                );
                draw::draw_text2(
                    &w.label(),
                    w.x() + 1,
                    w.y() + 1,
                    w.w() - 6,
                    w.h() - 6,
                    Align::Center,
                );
            } else {
                draw::draw_rbox(
                    w.x() + 1,
                    w.y() + 1,
                    w.w() - 4,
                    w.h() - 4,
                    15,
                    true,
                    Color::White,
                );
                draw::draw_text2(
                    &w.label(),
                    w.x() + 1,
                    w.y() + 1,
                    w.w() - 4,
                    w.h() - 4,
                    Align::Center,
                );
            }
        });
        Self { btn }
    }
}

fltk::widget_extends!(CairoButton, button::Button, btn);

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 600, 600, "Cairo");
    win.set_color(Color::White);

    let mut btn = CairoButton::new(100, 100, 200, 200, "Label");
    btn.set_callback(|_| println!("clicked!"));

    win.end();
    win.show();

    app.run().unwrap();
}
