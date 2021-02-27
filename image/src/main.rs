#![allow(unused_imports)]

use fltk::{app, draw, frame, image as fl_image, prelude::*, window};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::error::Error;
use std::io::Cursor;

#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "../glow/"]
struct Asset;

fn main() -> Result<(), Box<dyn Error>> {
    let img = Asset::get("ex.jpg").ok_or_else(|| "")?;
    let img = ImageReader::new(Cursor::new(img))
        .with_guessed_format()?
        .decode()?;
    let (w, h) = img.dimensions();

    let app = app::App::default();
    let mut wind = window::Window::default().with_size(w as i32, h as i32);
    let mut frame = frame::Frame::default().size_of(&wind);
    wind.end();
    wind.show();

    frame.draw(move || {
        draw::draw_image(&img.to_rgb8(), 0, 0, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
    });

    // // Or just convert to fltk::image::RgbImage
    // let rgb = fl_image::RgbImage::new(&img.to_rgb8(), w, h, ColorDepth::Rgb8)?;
    // frame.set_image(Some(rgb));

    app.run()?;
    Ok(())
}
