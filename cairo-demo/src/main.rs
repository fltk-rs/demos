use cairo::{Context, Format, ImageSurface};
use fltk::{enums::*, prelude::*, *};
use std::convert::TryFrom;

fn convert_to_rgba(arr: &[u8]) -> Vec<u8> {
    let mut v = vec![];
    for (_, pixel) in arr.chunks_exact(4).enumerate() {
        v.push(pixel[2]);
        v.push(pixel[1]);
        v.push(pixel[0]);
        v.push(pixel[3]);
    }
    v
}

#[derive(Clone)]
struct MyWidget {
    frm: frame::Frame,
    ctx: Context,
}

impl MyWidget {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let frm = frame::Frame::new(x, y, w, h, None).with_label(label);
        let surface = ImageSurface::create(Format::ARgb32, w, h).expect("Couldn’t create surface");
        let ctx = Context::new(&surface).unwrap();
        Self { frm, ctx }
    }

    pub fn draw<F: FnMut(&mut Self) + 'static>(&mut self, mut cb: F) {
        let mut frm = self.clone();
        self.frm.draw(move |_| {
            cb(&mut frm);
        });
    }
}

fltk::widget_extends!(MyWidget, frame::Frame, frm);

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 600, 600, "Cairo");

    let mut wid = MyWidget::new(0, 0, 600, 600, "Label");

    wid.draw(move |w| {
        let ctx = &w.ctx;
        ctx.set_source_rgb(0.0, 0.0, 1.0);
        ctx.paint().unwrap();
        let surface = ImageSurface::try_from(ctx.target()).unwrap();
        surface
            .with_data(|s| {
                let temp = convert_to_rgba(s);
                draw::draw_image(&temp, w.x(), w.y(), 600, 600, ColorDepth::Rgba8).unwrap();
            })
            .unwrap();
    });

    win.end();
    win.show();

    app.run().unwrap();
}
