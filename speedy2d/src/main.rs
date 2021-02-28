use fltk::{
    app,
    prelude::{WidgetExt, GroupExt, WindowExt},
    window::GlutWindow,
    utils
};
use speedy2d::GLRenderer;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageDataType, ImageSmoothingMode};

fn main() {
    let mut fb: Vec<u8> = vec![0u8; 128 * 128 * 3];
    for (iter, pixel) in fb.chunks_exact_mut(3).enumerate() {
        let x = iter % 128;
        let y = iter / 128;
        let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
        pixel.copy_from_slice(&[red, green, blue]);
    }

    let app = app::App::default();
    let mut win = GlutWindow::default().with_size(640, 480);
    win.end();
    win.show();
    win.make_current();

    gl::load_with(|s| win.get_proc_address(s));

    let mut renderer = unsafe { GLRenderer::new_for_current_context((640, 480)) }.unwrap();

    renderer.draw_frame(|graphics| {
        graphics.clear_screen(Color::WHITE);
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        let handle = graphics
        .create_image_from_raw_pixels(
            ImageDataType::RGB,
            ImageSmoothingMode::Linear,
            Vector2::new(128, 128),
            &fb,
        )
        .unwrap();
        graphics.draw_image(Vector2::new(200., 200.), &handle);
    });

    app.run().unwrap();
}
