#![forbid(unsafe_code)]
use {
    cairo::{Context, Format, ImageSurface},
    fltk::{button::Button, enums::*, frame::Frame, group::Flex, prelude::*, window::Window, *},
};

#[derive(Debug, Clone)]
struct Model {
    value: u8,
}
impl Model {
    fn inc(&mut self) {
        if self.value < 255 {
            self.value = self.value.saturating_add(1);
        }
    }
    fn dec(&mut self) {
        if self.value > 0 {
            self.value = self.value.saturating_sub(1);
        }
    }
}

fn main() -> Result<(), FltkError> {
    app::GlobalState::<Model>::new(Model { value: 0 });
    let app = app::App::default();
    let mut window = crate::window();
    let page = Flex::default()
        .with_size(300, 200)
        .center_of_parent()
        .column();
    Frame::default().draw(move |frame| {
        let value = app::GlobalState::<Model>::get().with(move |model| model.value);
            draw::set_draw_color(Color::Black);
            draw::set_font(Font::CourierBold, 14);
            draw::draw_text2(
                &value.to_string(),
                frame.x() + 1,
                frame.y() + 1,
                frame.w() - 6,
                frame.h() - 6,
                Align::Center,
            );
    });
    let row = Flex::default();
    for label in ["@#<", "@#>"] {
        crate::cairobutton()
            .with_label(label)
            .set_callback(crate::count);
    }
    row.end();
    page.end();
    window.end();
    window.show();
    app::redraw();
    app.run()
}

fn window() -> Window {
    const NAME: &str = "Demo: Cairo";
    let mut element = Window::default()
        .with_label(NAME)
        .with_size(640, 360)
        .center_screen();
    element.set_xclass(NAME);
    element.set_color(Color::White);
    element.make_resizable(true);
    element.draw(move |window| {
        let value = app::GlobalState::<Model>::get().with(move |model| model.value);
        window.set_label(&format!("{value} - {NAME}"));
    });
    element
}

fn count(button: &mut Button) {
    let label = button.label();
    app::GlobalState::<Model>::get().with(move |model| match label == "@#<" {
        true => model.dec(),
        false => model.inc(),
    });
    app::redraw();
}

fn cairobutton() -> Button {
    let mut element = button::Button::default();
    element.super_draw(false);
    element.draw(move |button| {
        draw::draw_rect_fill(button.x(), button.y(), button.w(), button.h(), Color::White);
        let mut surface = ImageSurface::create(Format::ARgb32, button.w(), button.h())
            .expect("Couldn’t create surface");
        crate::draw_surface(&mut surface, button.w(), button.h());
        if !button.value() {
            cairo_blur::blur_image_surface(&mut surface, 20);
        }
        surface
            .with_data(|s| {
                let mut img =
                    image::RgbImage::new(s, button.w(), button.h(), ColorDepth::Rgba8).unwrap();
                img.draw(button.x(), button.y(), button.w(), button.h());
            })
            .unwrap();
        draw::set_draw_color(Color::Black);
        draw::set_font(Font::Helvetica, app::font_size());
        if button.value() {
            draw::draw_rbox(
                button.x() + 1,
                button.y() + 1,
                button.w() - 4,
                button.h() - 4,
                15,
                true,
                Color::White,
            );
            draw::draw_text2(
                &button.label(),
                button.x() + 1,
                button.y() + 1,
                button.w() - 4,
                button.h() - 4,
                Align::Center,
            );
        } else {
            draw::draw_rbox(
                button.x() + 1,
                button.y() + 1,
                button.w() - 6,
                button.h() - 6,
                15,
                true,
                Color::White,
            );
            draw::draw_text2(
                &button.label(),
                button.x() + 1,
                button.y() + 1,
                button.w() - 6,
                button.h() - 6,
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
