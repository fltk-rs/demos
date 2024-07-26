use fltk::{prelude::*, *};
use std::num::NonZeroU32;

fn main() {
    const width: u32 = 400;
    const height: u32 = 300;
    let a = app::App::default();
    let mut w = window::Window::default().with_size(width as i32, height as i32);
    w.end();
    w.show();
    w.wait_for_expose();
    
    let context = softbuffer::Context::new(w.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, w.clone()).unwrap();
    surface
        .resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        )
        .unwrap();

    app::add_timeout(1.0, move || {
        let mut buffer = surface.buffer_mut().unwrap();
        for index in 0..(width * height) {
            let y = index / width;
            let x = index % width;
            let red = x % 255;
            let green = y % 255;
            let blue = (x * y) % 255;
    
            buffer[index as usize] = blue | (green << 8) | (red << 16);
        }
        buffer.present().unwrap();
    });
    

    a.run().unwrap();
}