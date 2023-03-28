use fltk::{
    prelude::*,
    *,
    image::IcoImage
};
use tiny_skia::*;

fn main() {
    let triangle = create_triangle();

    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.shader = Pattern::new(
        triangle.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Bicubic,
        1.0,
        Transform::from_row(1.5, -0.4, 0.0, -0.8, 5.0, 1.0),
    );

    let path = PathBuilder::from_circle(200.0, 200.0, 180.0).unwrap();

    let mut pixmap = Pixmap::new(400, 400).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(600, 400)
        .with_label("tiny-skia");
    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    win.make_resizable(true);
    win.set_icon(Some(icon));
    win.set_color(fltk::enums::Color::White);
    let mut frame = frame::Frame::default().with_size(400, 400).center_of(&win);
    win.end();
    win.show();
    draw::draw_rgba(&mut frame, pixmap.data()).unwrap();
    app.run().unwrap();
}

fn create_triangle() -> Pixmap {
    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 200);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(0.0, 20.0);
    pb.line_to(20.0, 20.0);
    pb.line_to(10.0, 0.0);
    pb.close();
    let path = pb.finish().unwrap();

    let mut pixmap = Pixmap::new(20, 20).unwrap();
    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );
    pixmap
}
