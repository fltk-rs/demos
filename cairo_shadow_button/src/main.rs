#![forbid(unsafe_code)]
mod model;
use {
    cairo::{Context, Format, ImageSurface},
    fltk::{
        app,
        button::Button,
        draw,
        enums::{Align, Color, ColorDepth, Cursor, Event, Font, FrameType, Shortcut},
        frame::Frame,
        group::Flex,
        image::{RgbImage, SvgImage},
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        window::Window,
    },
    model::Model,
    std::{cell::RefCell, rc::Rc},
};

const HEARTBEAT: Event = Event::from_i32(404);
const INC: Event = Event::from_i32(405);
const DEC: Event = Event::from_i32(406);

fn main() -> Result<(), FltkError> {
    let app = app::App::default();
    crate::window();
    app::handle_main(HEARTBEAT).unwrap();
    app.run()
}

fn window() {
    const NAME: &str = "FlCairoButton";
    let mut element = Window::default()
        .with_label(NAME)
        .with_size(640, 360)
        .center_screen();
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    element.set_color(Color::from_u32(0xfdf6e3));
    element.make_resizable(false);
    let state = Rc::from(RefCell::from(Model::default()));
    crate::view(state.clone());
    element.end();
    element.show();
    element.handle(move |window, event| {
        if event == HEARTBEAT {
            window.set_label(&format!("{} - {NAME}", state.borrow().value));
            false
        } else if app::event() == Event::Close {
            app::quit();
            true
        } else {
            false
        }
    });
}

fn view(state: Rc<RefCell<Model>>) {
    let mut page = Flex::default()
        .with_size(600, 200)
        .center_of_parent()
        .column();
    {
        let hero = Flex::default();
        crate::button().with_label("@#<").set_callback(move |_| {
            app::handle_main(crate::DEC).unwrap();
        });
        crate::frame(state.clone());
        crate::button().with_label("@#>").set_callback(move |_| {
            app::handle_main(crate::INC).unwrap();
        });
        hero.end();
    }
    page.end();
    page.set_pad(0);
    page.set_margin(0);
    page.handle(move |_, event| match event {
        INC => {
            state.borrow_mut().inc();
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        DEC => {
            state.borrow_mut().dec();
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        _ => false,
    });
}

fn frame(state: Rc<RefCell<Model>>) -> Frame {
    let mut element = Frame::default();
    element.set_label_size(60);
    element.handle(move |frame, event| match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                crate::menu().popup();
                true
            }
            _ => false,
        },
        HEARTBEAT => {
            frame.set_label(&state.clone().borrow().value.to_string());
            true
        }
        Event::Enter => {
            frame.window().unwrap().set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            frame.window().unwrap().set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    });
    element
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default().with_type(MenuButtonType::Popup3);
    element.set_frame(FrameType::FlatBox);
    element.add(
        "@#+  &Increment",
        Shortcut::Ctrl | '+',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(INC).unwrap();
        },
    );
    element.add(
        "@#-  &Decrement",
        Shortcut::Ctrl | '-',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(DEC).unwrap();
        },
    );
    element
}

fn button() -> Button {
    let mut element = Button::default();
    element.super_draw(false);
    element.draw(move |button| {
        draw::draw_rect_fill(
            button.x(),
            button.y(),
            button.w(),
            button.h(),
            Color::from_u32(0xfdf6e3),
        );
        let mut surface = ImageSurface::create(Format::ARgb32, button.w(), button.h())
            .expect("Couldn’t create surface");
        crate::draw_surface(&mut surface, button.w(), button.h());
        if !button.value() {
            cairo_blur::blur_image_surface(&mut surface, 20);
        }
        surface
            .with_data(|s| {
                let mut img = RgbImage::new(s, button.w(), button.h(), ColorDepth::Rgba8).unwrap();
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
