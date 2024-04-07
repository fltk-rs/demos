#![forbid(unsafe_code)]
mod calendar;

use chrono::prelude::*;
use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Plastic);
    let mut win = window::Window::default()
        .with_label("Demo: Calendar")
        .with_size(400, 300)
        .center_screen();
    button::Button::default()
        .with_label("Click")
        .with_size(80, 40)
        .center_of_parent()
        .set_callback(move |_| {
            let cal = calendar::Calendar::default(); // or calendar::Calendar::new(200, 100);
            if let Some(date) = cal.get_date() {
                println!("{:?}", date.year());
                println!("{:?}", date.month());
                println!("{:?}", date.day());
            }
        });
    win.end();
    win.make_resizable(true);
    win.show();
    app::background(0xd3, 0xd3, 0xd3);
    app.run().unwrap();
}
