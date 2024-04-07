#![forbid(unsafe_code)]
#[cfg(target_os = "linux")]
use {
    cairo::{Context, Format, ImageSurface},
    fltk::{enums::*, prelude::*, *},
};

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
            draw_surface(&mut surface, w.w(), w.h());
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
    let app = app::App::default().with_scheme(app::AppScheme::Base);
    let mut win = window::Window::default()
        .with_label("Demo: Cairo")
        .with_size(600, 600)
        .center_screen();

    CairoButton::new(100, 100, 200, 200, "Label").set_callback(|_| println!("clicked!"));

    win.end();
    win.set_color(Color::White);
    win.make_resizable(true);
    win.show();

    app.run().unwrap();
}

fn draw_surface(surface: &mut ImageSurface, w: i32, h: i32) {
    let ctx = Context::new(surface).unwrap();
    ctx.save().unwrap();
    let corner_radius = h as f64 / 10.0;
    let radius = corner_radius / 1.0;
    let degrees = std::f64::consts::PI / 180.0;

    ctx.new_sub_path();
    ctx.arc(w as f64 - radius, radius, radius, -90. * degrees, 0.0);
    ctx.arc(
        w as f64 - radius,
        h as f64 - radius,
        radius,
        0.0,
        90. * degrees,
    );
    ctx.arc(
        radius,
        h as f64 - radius,
        radius,
        90. * degrees,
        180. * degrees,
    );
    ctx.arc(radius, radius, radius, 180. * degrees, 270. * degrees);
    ctx.close_path();

    ctx.set_source_rgba(150.0 / 255.0, 150.0 / 255.0, 150.0 / 255.0, 40.0 / 255.0);
    ctx.set_line_width(4.);
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}
