#![forbid(unsafe_code)]
#![allow(unused_imports)]

use fltk::{app, draw, enums::*, frame, image as fl_image, prelude::*, window};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::io::Cursor;

#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "../glow/"]
struct Asset;

fn main() {
    let app = app::App::default();
    let img = ImageReader::new(Cursor::new(Asset::get("ex.jpg").unwrap().data.as_ref()))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let (w, h) = img.dimensions();

    let mut wind = window::Window::default().with_size(w as i32, h as i32);
    wind.make_resizable(true);
    frame::Frame::default_fill().draw(move |_| {
        draw::draw_image(&img.to_rgb8(), 0, 0, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
    });
    wind.end();
    wind.show();

    // Or just convert to fltk::image::RgbImage
    // let rgb = fl_image::RgbImage::new(&img.to_rgb8(), w, h, ColorDepth::Rgb8).unwrap();
    // frame.set_image(Some(rgb));

    app.run().unwrap();
}
