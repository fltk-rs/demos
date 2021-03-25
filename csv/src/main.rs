use fltk::*;
use serde::Deserialize;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Deserialize)]
pub struct Price {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename = "Volume")]
    volume: usize,
}

lazy_static! {
    pub static ref PRICES: Mutex<Vec<Price>> = Mutex::new(vec![]);
}

fn main() {
    let files = std::fs::read_dir("historical_data").unwrap();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(79, 79, 79);
    app::background2(41, 41, 41);
    app::foreground(255, 255, 255);
    let mut wind = window::Window::default().with_size(800, 600);
    let mut browser = browser::Browser::new(5, 10, 100, 580, "");
    let mut frame = frame::Frame::default()
        .with_size(680, 580)
        .right_of(&browser, 10);
    wind.make_resizable(true);
    wind.show();

    browser.set_type(browser::BrowserType::Hold);
    for file in files {
        let entry = file.unwrap().file_name().into_string().unwrap();
        if entry.ends_with(".csv") {
            browser.add(&entry.strip_suffix(".csv").unwrap());
        }
    }

    frame.set_frame(FrameType::DownBox);
    frame.set_color(Color::Black);

    frame.draw2(|f| {
        let data = PRICES.lock().unwrap();
        let mut highest = data
            .iter()
            .map(|elem| elem.low)
            .collect::<Vec<f64>>()
            .iter()
            .cloned()
            .fold(0. / 0., f64::max);
        highest += (highest.to_string().len() * 10) as f64 / 3.;
        let factor = f.h() as f64 / highest;
        if data.len() != 0 {
            let step = f.w() / data.len() as i32;
            let mut idx = f.x() + step;
            for elem in &*data {
                let open = f.h() - (elem.open * factor) as i32;
                let high = f.h() - (elem.high * factor) as i32;
                let low = f.h() - (elem.low * factor) as i32;
                let close = f.h() - (elem.close * factor) as i32;
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

    browser.set_callback2(move |t| {
        if let Some(file) = t.selected_text() {
            let file = format!("historical_data/{}.csv", file);
            let mut rdr = csv::Reader::from_reader(std::fs::File::open(file).unwrap());
            let mut data = PRICES.lock().unwrap();
            data.clear();
            for result in rdr.deserialize() {
                let price: Price = result.unwrap();
                data.push(price);
            }
            frame.redraw();
        }
    });

    app.run().unwrap();
}
