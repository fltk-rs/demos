#![forbid(unsafe_code)]
use {
    cairo::{Context, Format, ImageSurface},
    fltk::{button::Button, enums::*, frame::Frame, group::Flex, prelude::*, window::Window, *},
};

const INC: i32 = 101;
const DEC: i32 = 102;

fn main() -> Result<(), FltkError> {
    let mut value: u8 = 0;
    let app = app::App::default();
    let mut window = crate::window();
    let page = Flex::default()
        .with_size(300, 200)
        .center_of_parent()
        .column();
    Frame::default()
        .with_label(&value.to_string())
        .handle(move |frame, event| match event.bits() {
            crate::INC => {
                if value < 255 {
                    value = value.saturating_add(1);
                    frame.set_label(&value.to_string());
                }
                true
            }
            crate::DEC => {
                if value > 0 {
                    value = value.saturating_sub(1);
                    frame.set_label(&value.to_string());
                }
                true
            }
            _ => false,
        });
    let row = Flex::default();
    for label in ["@#<", "@#>"] {
        crate::cairobutton()
            .with_label(label)
            .set_callback(move |button| {
                app::handle_main(match button.label() == "@#<" {
                    true => crate::DEC,
                    false => crate::INC,
                })
                .unwrap();
            });
    }
    row.end();
    page.end();
    window.end();
    window.show();
    app.run()
}

fn window() -> Window {
    let mut element = Window::default()
        .with_label("Demo: Cairo")
        .with_size(640, 360)
        .center_screen();
    element.set_color(Color::White);
    element.make_resizable(true);
    element
}

fn cairobutton() -> Button {
    let mut element = button::Button::default();
    element.super_draw(false);
    element.draw(|w| {
        draw::draw_rect_fill(w.x(), w.y(), w.w(), w.h(), Color::White);
        let mut surface =
            ImageSurface::create(Format::ARgb32, w.w(), w.h()).expect("Couldn’t create surface");
        crate::draw_surface(&mut surface, w.w(), w.h());
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
    element
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
