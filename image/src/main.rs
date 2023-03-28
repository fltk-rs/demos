#![allow(unused_imports)]

use fltk::{
    app,
    draw,
    enums::*,
    frame,
    image as fl_image,
    prelude::*,
    window,
    image::IcoImage
};
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
    let img = Asset::get("ex.jpg")?;
    let img = ImageReader::new(Cursor::new(img.data.as_ref()))
        .with_guessed_format()?
        .decode()?;
    let (w, h) = img.dimensions();

    let app = app::App::default();
    let mut wind = window::Window::default().with_size(w as i32, h as i32);
    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    wind.make_resizable(true);
    wind.set_icon(Some(icon));
    let mut frame = frame::Frame::default().size_of(&wind);
    wind.end();
    wind.show();

    frame.draw(move |_| {
        draw::draw_image(&img.to_rgb8(), 0, 0, w as i32, h as i32, ColorDepth::Rgb8).unwrap();
    });

    // // Or just convert to fltk::image::RgbImage
    // let rgb = fl_image::RgbImage::new(&img.to_rgb8(), w, h, ColorDepth::Rgb8)?;
    // frame.set_image(Some(rgb));

    app.run()?;
    Ok(())
}
