mod calendar;

use chrono::prelude::*;
use fltk::{prelude::*, *};

fn main() {
    let mut win = window::Window::default()
        .with_label("Demo: Calendar")
        .with_size(400, 300)
        .center_screen();
    button::Button::new(160, 200, 80, 40, "Click").set_callback(move |_| {
        let cal = calendar::Calendar::default(); // or calendar::Calendar::new(200, 100);
        let date = cal.get_date();
        println!("{:?}", date);
        if let Some(date) = date {
            println!("{:?}", date.year());
            println!("{:?}", date.month());
            println!("{:?}", date.day());
        }
    });
    win.end();
    win.make_resizable(true);
    win.show();
    app::background(0xd3, 0xd3, 0xd3);
    app::App::default()
        .with_scheme(app::Scheme::Gtk)
        .run()
        .unwrap();
}
