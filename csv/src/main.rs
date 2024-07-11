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
};

const HEARTBEAT: i32 = 404;
const NAME: &str = "FlCSV";

fn main() -> Result<(), FltkError> {
    app::GlobalState::<Model>::new(Model::new());
    let app = app::App::default();
    let mut window = crate::window();
    crate::view();
    window.end();
    window.show();
    app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
    app.run()
}

fn view() {
    let mut page = Flex::default_fill();
    {
        let mut left = Flex::default_fill().column();
        crate::browser("Browser");
        crate::button("Save image", &mut left);
        left.end();
        left.set_pad(10);
        page.fixed(&left, 90);
    }
    crate::frame("Canvas");
    page.end();
    page.set_pad(10);
    page.set_margin(10);
}

fn browser(tooltip: &str) -> Browser {
    app::GlobalState::<Model>::get().with(move |model| {
        for file in std::fs::read_dir("assets/historical_data").unwrap() {
            let entry = file.unwrap().file_name().into_string().unwrap();
            if entry.ends_with(".csv") {
                model
                    .temp
                    .push(entry.strip_suffix(".csv").unwrap().to_string());
            }
            model.choice(0);
        }
    });
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    element.handle(move |browser, event| {
        if event == Event::from_i32(HEARTBEAT) {
            let (curr, temp) = app::GlobalState::<Model>::get()
                .with(move |model| (model.curr, model.temp.clone()));
            if !temp.is_empty() {
                browser.clear();
                for item in temp {
                    browser.add(&item);
                }
                browser.select(curr as i32 + 1);
            }
            false
        } else if event == Event::Push {
            if browser.value() > 0 {
                let curr: usize = browser.value() as usize - 1;
                app::GlobalState::<Model>::get().with(move |model| model.choice(curr));
                app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
                app::redraw();
            };
            true
        } else {
            false
        }
    });
    element
}

fn button(tooltip: &str, flex: &mut Flex) {
    let mut element = Button::default().with_label("@#filesave");
    element.set_tooltip(tooltip);
    element.handle(move |button, event| {
        if event == Event::Push {
            button.deactivate();
            app::GlobalState::<Model>::get().with(move |model| model.save = true);
            app::redraw();
            button.activate();
        };
        true
    });
    flex.fixed(&element, 30);
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default();
    element.set_frame(FrameType::DownBox);
    element.set_tooltip(tooltip);
    element.set_color(Color::Black);
    element.set_callback(crate::save);
    element.draw(|frame| {
        let model = app::GlobalState::<Model>::get().with(move |model| model.clone());
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
            if model.save {
                app::GlobalState::<Model>::get().with(move |model| model.save = false);
                frame.do_callback();
            }
        }
    });
    element
}

fn save(frame: &mut Frame) {
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
}

fn window() -> Window {
    let mut element = Window::default()
        .with_size(640, 360)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(true);
    element.size_range(640, 360, 0, 0);
    element.set_xclass(NAME);
    element.handle(move |window, event| {
        if event == Event::from_i32(HEARTBEAT) {
            let value =
                app::GlobalState::<Model>::get().with(move |model| model.temp[model.curr].clone());
            window.set_label(&format!("{value} - {NAME}"));
            false
        } else if app::event() == Event::Close {
            app::quit();
            true
        } else {
            false
        }
    });
    ColorTheme::new(color_themes::DARK_THEME).apply();
    element
}
