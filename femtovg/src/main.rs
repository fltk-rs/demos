use femtovg::{renderer::OpenGl, Canvas, Color, Paint, Path};
use fltk::{
    app, enums,
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::GlutWindow,
};

fn main() {
    let app = app::App::default();
    let mut win = GlutWindow::default()
        .with_size(640, 480)
        .with_label("femtovg example");
    win.set_mode(enums::Mode::Opengl3);
    win.end();
    win.show();
    win.make_current();
    let renderer =
        OpenGl::new(|s| win.get_proc_address(s) as *const _).expect("Cannot create renderer");
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

    win.draw(move |w| {
        canvas.set_size(w.width() as u32, w.height() as u32, 1.);
        canvas.clear_rect(
            0,
            0,
            w.width() as u32,
            w.height() as u32,
            Color::rgbf(0.9, 0.9, 0.9),
        );
        let mut p = Path::new();
        p.rect(0.0, 0.0, w.width() as _, w.height() as _);
        canvas.fill_path(
            &mut p,
            Paint::linear_gradient(
                0.0,
                0.0,
                w.width() as _,
                0.0,
                Color::rgba(255, 0, 0, 255),
                Color::rgba(0, 0, 255, 255),
            ),
        );

        canvas.save();
        canvas.reset();
        canvas.restore();

        canvas.flush();
        w.swap_buffers();
    });
    app.run().unwrap();
}
