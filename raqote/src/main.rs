use raqote::*;
use fltk::{
    app, enums, frame, image,
    prelude::{GroupExt, WidgetExt},
    window,
};

const WIDTH: i32 = 500;
const HEIGHT: i32 = 400;

fn main() {
    let mut dt = DrawTarget::new(WIDTH, HEIGHT);

    let mut pb = PathBuilder::new();
    pb.move_to(100., 10.);
    pb.cubic_to(150., 40., 175., 0., 200., 10.);
    pb.quad_to(120., 100., 80., 200.);
    pb.quad_to(150., 180., 300., 300.);
    pb.close();
    let path = pb.finish();

    let gradient = Source::new_radial_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.2,
                    color: Color::new(0xff, 0, 0xff, 0),
                },
                GradientStop {
                    position: 0.8,
                    color: Color::new(0xff, 0xff, 0xff, 0xff),
                },
                GradientStop {
                    position: 1.,
                    color: Color::new(0xff, 0xff, 0, 0xff),
                },
            ],
        },
        Point::new(150., 150.),
        128.,
        Spread::Pad,
    );
    dt.fill(&path, &gradient, &DrawOptions::new());

    let mut pb = PathBuilder::new();
    pb.move_to(100., 100.);
    pb.line_to(300., 300.);
    pb.line_to(200., 300.);
    let path = pb.finish();

    dt.stroke(
        &path,
        &Source::Solid(SolidSource {
            r: 0x0,
            g: 0x0,
            b: 0x80,
            a: 0x80,
        }),
        &StrokeStyle {
            cap: LineCap::Round,
            join: LineJoin::Round,
            width: 10.,
            miter_limit: 2.,
            dash_array: vec![10., 18.],
            dash_offset: 16.,
        },
        &DrawOptions::new(),
    );

    let app = app::App::default();
    let mut win = window::Window::default().with_size(WIDTH, HEIGHT);
    win.set_color(enums::Color::White);
    let mut frame = frame::Frame::default().size_of(&win);
    win.end();
    win.show();
    let img = image::RgbImage::new(dt.get_data_u8(), WIDTH as u32, HEIGHT as u32, 4).unwrap();
    frame.set_image(Some(img));
    win.redraw();
    app.run().unwrap();
}
