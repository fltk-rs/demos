#![forbid(unsafe_code)]
mod model;

use {
    ::image::{ImageBuffer, RgbImage},
    fltk::{
        browser::{Browser, BrowserType},
        button::Button,
        enums::*,
        frame::Frame,
        group::Flex,
        prelude::*,
        surface::ImageSurface,
        window::Window,
        *,
    },
    fltk_theme::{color_themes, ColorTheme},
    model::Model,
    std::{cell::RefCell, rc::Rc},
};

const HEARTBEAT: Event = Event::from_i32(404);
const SAVE: Event = Event::from_i32(405);
const PAD: i32 = 10;
const HEIGHT: i32 = 3 * PAD;
const WIDTH: i32 = 3 * HEIGHT;

fn main() -> Result<(), FltkError> {
    let app = app::App::default();
    crate::window();
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::handle_main(HEARTBEAT).unwrap();
    app.run()
}

fn window() {
    const HEIGHT: i32 = 360;
    const WIDTH: i32 = 640;
    const NAME: &str = "FlCSV";
    let state = Rc::from(RefCell::from(Model::default()));
    let mut element = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(true);
    element.size_range(WIDTH, HEIGHT, 0, 0);
    element.set_xclass(NAME);
    crate::view(state.clone());
    element.handle(move |window, event| {
        if event == HEARTBEAT {
            let value = &state.borrow().temp[state.borrow().curr];
            window.set_label(&format!("{value} - {NAME}"));
            false
        } else if app::event() == Event::Close {
            app::quit();
            true
        } else {
            false
        }
    });
    element.end();
    element.show();
}

fn view(state: Rc<RefCell<Model>>) {
    let mut page = Flex::default_fill();
    {
        let mut left = Flex::default_fill().column();
        crate::browser("Browser", state.clone());
        left.fixed(
            &crate::button("Save image").with_label("@#filesave"),
            HEIGHT,
        );
        left.end();
        left.set_pad(PAD);
        page.fixed(&left, WIDTH);
        crate::frame("Canvas", state);
    }
    page.end();
    page.set_pad(PAD);
    page.set_margin(PAD);
}

fn browser(tooltip: &str, state: Rc<RefCell<Model>>) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    element.handle(move |browser, event| match event {
        HEARTBEAT => {
            let (curr, temp) = (state.borrow().curr, &state.borrow().temp);
            if !temp.is_empty() {
                browser.clear();
                for item in temp {
                    browser.add(item);
                }
                browser.select(curr as i32 + 1);
            }
            false
        }
        Event::Push => {
            if browser.value() > 0 {
                state.borrow_mut().choice(browser.value() as usize - 1);
                app::handle_main(HEARTBEAT).unwrap();
                app::redraw();
            };
            true
        }
        _ => false,
    });
    element
}

fn button(tooltip: &str) -> Button {
    let mut element = Button::default();
    element.set_tooltip(tooltip);
    element.set_callback(move |_| {
        app::handle_main(SAVE).unwrap();
    });
    element
}

fn frame(tooltip: &str, state: Rc<RefCell<Model>>) -> Frame {
    let mut element = Frame::default();
    element.set_frame(FrameType::DownBox);
    element.set_tooltip(tooltip);
    element.set_color(Color::Black);
    element.handle(crate::save);
    element.draw(move |frame| {
        let model = state.borrow().clone();
        if let Some(data) = model.cash.get(&model.temp[model.curr]) {
            let mut highest = data
                .iter()
                .map(|elem| elem.low)
                .collect::<Vec<f64>>()
                .iter()
                .cloned()
                .fold(f64::NAN, f64::max);
            highest += (highest.to_string().len() * 10) as f64 / 3.;
            let factor = frame.h() as f64 / highest;
            if !data.is_empty() {
                let step = frame.w() / data.len() as i32;
                let mut idx = frame.x() + step;
                for elem in data {
                    let open = frame.h() - (elem.open * factor) as i32;
                    let high = frame.h() - (elem.high * factor) as i32;
                    let low = frame.h() - (elem.low * factor) as i32;
                    let close = frame.h() - (elem.close * factor) as i32;
                    draw::draw_line(idx, high, idx, low);
                    let col = if close > open {
                        Color::Red
                    } else {
                        Color::Green
                    };
                    draw::set_draw_color(col);
                    draw::draw_rectf(idx - 2, open, 4, i32::abs(close - open));
                    draw::set_draw_color(Color::White);
                    idx += step;
                }
            };
        }
    });
    element
}

fn save(frame: &mut Frame, event: Event) -> bool {
    if event == crate::SAVE {
        let sur = ImageSurface::new(frame.w(), frame.h(), false);
        ImageSurface::push_current(&sur);
        draw::set_draw_color(enums::Color::White);
        draw::draw_rectf(0, 0, frame.w(), frame.h());
        sur.draw(frame, 0, 0);
        let img = sur.image().unwrap();
        ImageSurface::pop_current();
        let mut imgbuf: RgbImage = ImageBuffer::new(frame.w() as _, frame.h() as _); // this is from the image crate
        imgbuf.copy_from_slice(&img.to_rgb_data());
        imgbuf
            .save(frame.window().unwrap().label() + ".jpg")
            .unwrap();
        true
    } else {
        false
    }
}
