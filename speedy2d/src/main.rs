use fltk::{
    app,
    enums::Event,
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    utils,
    window::{GlWindow, Window},
};
use speedy2d::{
    color::Color,
    dimen::Vector2,
    image::{ImageDataType, ImageSmoothingMode},
    GLRenderer,
};

fn main() {
    let mut fb: Vec<u8> = vec![0u8; 300 * 300 * 3];
    for (iter, pixel) in fb.chunks_exact_mut(3).enumerate() {
        let x = iter % 300;
        let y = iter / 300;
        let (red, green, blue) = utils::hex2rgb((x ^ y) as u32);
        pixel.copy_from_slice(&[red, green, blue]);
    }

    let app = app::App::default();
    let mut main_win = Window::default().with_size(730, 430);
    main_win.make_resizable(true);
    let mut win = GlWindow::default().with_size(300, 300).center_of(&main_win);
    win.end();
    main_win.end();
    main_win.show();
    win.make_current();

    win.handle(|_, ev| match ev {
        Event::Push => {
            println!("Pushed");
            true
        }
        _ => false,
    });

    gl::load_with(|s| win.get_proc_address(s));

    let mut renderer = unsafe { GLRenderer::new_for_current_context((300, 300)) }.unwrap();

    renderer.draw_frame(|graphics| {
        graphics.clear_screen(Color::WHITE);
        let handle = graphics
            .create_image_from_raw_pixels(
                ImageDataType::RGB,
                ImageSmoothingMode::Linear,
                Vector2::new(300, 300),
                &fb,
            )
            .unwrap();
        graphics.draw_image(Vector2::new(0., 0.), &handle);
    });

    app.run().unwrap();
}
