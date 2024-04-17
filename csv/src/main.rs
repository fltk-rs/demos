#![forbid(unsafe_code)]
use {
    ::image::{ImageBuffer, RgbImage},
    fltk::{
        browser::{Browser, BrowserType},
        button::Button,
        enums::*,
        frame::Frame,
        group::Flex,
        prelude::*,
        window::Window,
        *,
    },
    fltk_theme::{color_themes, ColorTheme},
    serde::Deserialize,
    std::sync::Mutex,
};

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Deserialize)]
pub struct Price {
    #[serde(rename = "Date")]
    _date: String,
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename = "Volume")]
    _volume: usize,
}

const NAME: &str = "FlCSV";

lazy_static! {
    pub static ref PRICES: Mutex<Vec<Price>> = Mutex::new(vec![]);
}

fn main() {
    let app = app::App::default();
    let mut window = crate::window();
    let mut page = Flex::default_fill();
    let mut left = Flex::default_fill().column();
    crate::browser("Browser");
    let save = crate::button("Save image");
    left.end();
    crate::frame("Canvas");
    page.end();
    window.end();
    window.show();
    {
        left.fixed(&save, 30);
        left.set_pad(10);
        page.fixed(&left, 90);
        page.set_pad(10);
        page.set_margin(10);
        ColorTheme::new(color_themes::DARK_THEME).apply();
    }
    app.run().unwrap();
}

fn browser(tooltip: &str) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    for file in std::fs::read_dir("assets/historical_data").unwrap() {
        let entry = file.unwrap().file_name().into_string().unwrap();
        if entry.ends_with(".csv") {
            element.add(entry.strip_suffix(".csv").unwrap());
        }
    }
    element.set_callback(move |browser| {
        if let Some(file) = browser.selected_text() {
            let mut window = app::first_window().unwrap();
            window.set_label(&format!("{file} - {NAME}"));
            let file = format!("assets/historical_data/{}.csv", file);
            let mut rdr = csv::Reader::from_reader(std::fs::File::open(file).unwrap());
            let mut data = PRICES.lock().unwrap();
            data.clear();
            for result in rdr.deserialize() {
                let price: Price = result.unwrap();
                data.push(price);
            }
            window.redraw();
        }
    });
    element.set_tooltip(tooltip);
    element
}

fn button(tooltip: &str) -> Button {
    let mut element = Button::default().with_label("@#filesave");
    element.set_tooltip(tooltip);
    element.set_callback(move |_| {
        let frame = app::widget_from_id::<Frame>("Canvas").unwrap();
        let sur = surface::ImageSurface::new(frame.w(), frame.h(), false);
        surface::ImageSurface::push_current(&sur);
        draw::set_draw_color(enums::Color::White);
        draw::draw_rectf(0, 0, frame.w(), frame.h());
        sur.draw(&frame, 0, 0);
        let img = sur.image().unwrap();
        surface::ImageSurface::pop_current();
        let mut imgbuf: RgbImage = ImageBuffer::new(frame.w() as _, frame.h() as _); // this is from the image crate
        imgbuf.copy_from_slice(&img.to_rgb_data());
        imgbuf
            .save(app::first_window().unwrap().label() + ".jpg")
            .unwrap();
    });
    element
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default().with_id(tooltip);
    element.set_frame(FrameType::DownBox);
    element.set_tooltip(tooltip);
    element.set_color(Color::Black);
    element.draw(|frame| {
        let data = PRICES.lock().unwrap();
        let mut highest = data
            .iter()
            .map(|elem| elem.low)
            .collect::<Vec<f64>>()
            .iter()
            .cloned()
            .fold(f64::NAN, f64::max);
        highest += (highest.to_string().len() * 10) as f64 / 3.;
        let factor = frame.h() as f64 / highest;
        if data.len() != 0 {
            let step = frame.w() / data.len() as i32;
            let mut idx = frame.x() + step;
            for elem in &*data {
                let open = frame.h() - (elem.open * factor) as i32;
                let high = frame.h() - (elem.high * factor) as i32;
                let low = frame.h() - (elem.low * factor) as i32;
                let close = frame.h() - (elem.close * factor) as i32;
                draw::set_draw_color(Color::White);
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
        }
    });
    element
}

fn window() -> Window {
    let mut element = Window::default()
        .with_size(640, 360)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(true);
    element.size_range(640, 360, 0, 0);
    element.set_xclass(NAME);
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            app::quit();
        }
    });
    element
}
