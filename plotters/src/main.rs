use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(500, 400);
    win.end();
    win.show();
    app.run().unwrap();
}
