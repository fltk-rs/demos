use {
    femtovg::{renderer::OpenGl, Canvas, Color, Paint, Path},
    fltk::{
        app, enums,
        prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
        window::GlWindow,
    },
};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Base);
    let mut win = GlWindow::default()
        .with_label("femtovg example")
        .with_size(640, 480);
    win.end();
    win.set_mode(enums::Mode::Opengl3);
    win.make_resizable(true);
    win.show();
    win.make_current();
    win.draw(crate::canvas);
    app.run().unwrap();
}

fn canvas(w: &mut GlWindow) {
    let renderer = unsafe {
        OpenGl::new_from_function(|s| w.get_proc_address(s) as *const _)
            .expect("Cannot create renderer")
    };
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
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
        &p,
        &Paint::linear_gradient(
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
}
