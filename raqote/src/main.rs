use fltk::{
    app, draw, enums, frame,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window,
};
use raqote::*;
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH: i32 = 736;
const HEIGHT: i32 = 431;

fn main() {
    let dt = DrawTarget::new(WIDTH, HEIGHT);
    let dt = Rc::from(RefCell::from(dt));
    let dt_c = dt.clone();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(WIDTH + 10, HEIGHT + 10);
    let mut frame = frame::Frame::new(5, 5, WIDTH, HEIGHT, "");
    frame.set_frame(enums::FrameType::DownBox);
    frame.set_color(enums::Color::White);
    win.make_resizable(true);
    win.end();
    win.show();

    let mut x = 0;
    let mut y = 0;
    frame.handle(move |f, ev| match ev {
        enums::Event::Push => {
            let coords = app::event_coords();
            let path = draw_line(coords.0, coords.1, coords.0, coords.1);
            dt.borrow_mut().stroke(
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
                    width: 5.,
                    miter_limit: 0.,
                    dash_array: vec![],
                    dash_offset: 0.,
                },
                &DrawOptions::new(),
            );
            x = coords.0;
            y = coords.1;
            f.redraw();
            true
        }
        enums::Event::Drag => {
            let coords = app::event_coords();
            let path = draw_line(x, y, coords.0, coords.1);
            dt.borrow_mut().stroke(
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
                    width: 5.,
                    miter_limit: 0.,
                    dash_array: vec![],
                    dash_offset: 0.,
                },
                &DrawOptions::new(),
            );
            x = coords.0;
            y = coords.1;
            f.redraw();
            true
        }
        _ => false,
    });
    unsafe {
        draw::draw_rgba_nocopy(&mut frame, dt_c.borrow().get_data_u8());
    }
    app.run().unwrap();
}

pub fn draw_line(x: i32, y: i32, x2: i32, y2: i32) -> Path {
    let mut pb = PathBuilder::new();
    pb.move_to(x as f32, y as f32);
    pb.line_to(x2 as f32, y2 as f32);
    pb.finish()
}
