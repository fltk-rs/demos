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
    let files = std::fs::read_dir(".").unwrap();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);
    let mut browser = browser::Browser::new(5, 10, 300, 580, "");
    let mut frame = frame::Frame::default().with_size(480, 580).right_of(&browser, 10);
    wind.make_resizable(true);
    wind.show();

    browser.set_type(browser::BrowserType::Hold);    
    for file in files {
        let entry = file.unwrap().file_name().into_string().unwrap();
        if entry.ends_with(".csv") {
            browser.add(&entry);
        }
    }

    frame.set_frame(FrameType::DownBox);
    frame.set_color(Color::White);

    frame.draw2(|f| {
        let data = PRICES.lock().unwrap();
        if data.len() != 0 {
            let step = f.w() / data.len() as i32;
            let mut idx = f.x() + step;
            for elem in &*data {
                let open = f.h() - (elem.open * 5.) as i32 ;
                let high = f.h() - (elem.high * 5.) as i32;
                let low = f.h() - (elem.low * 5.) as i32;
                let close = f.h() - (elem.close * 5.) as i32;
                draw::set_draw_color(Color::Black);
                draw::draw_line(idx, high, idx, low);
                let col = if close > open { Color::Red } else { Color::Green };
                draw::set_draw_color(col);
                draw::draw_rectf(idx - 2, open, 4, i32::abs(close-open));
                draw::set_draw_color(Color::Black);
                idx += step;
            }
        }
    });

    browser.set_callback2(move |t| {
        if let Some(file) = t.selected_text() {
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
