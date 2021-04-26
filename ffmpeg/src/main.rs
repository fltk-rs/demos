use fltk::{
    app, button,
    enums::{Color, FrameType},
    frame,
    image,
    prelude::*,
    window,
};
use signal_hook::{consts::signal::SIGINT, iterator::Signals};
use std::{env, error, fs, process, thread};

lazy_static::lazy_static! {
    pub static ref VIDEO_TEMP_DIR: String = env::temp_dir().join("video_mp4").to_string_lossy().to_string();
}

struct MyApp {}

impl Drop for MyApp {
    fn drop(&mut self) {
        fs::remove_dir_all(&*VIDEO_TEMP_DIR).unwrap();
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    fs::create_dir(&*VIDEO_TEMP_DIR).ok();

    process::Command::new("ffmpeg")
        .args(&[
            "-i",
            "video.mp4",
            &format!("{}/%d.bmp", &*VIDEO_TEMP_DIR),
            "-y",
        ])
        .status()
        .unwrap();

    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            fs::remove_dir_all(&*VIDEO_TEMP_DIR).unwrap();
        }
    });

    let _a = MyApp {};
    let app = app::App::default();
    let mut win = window::Window::default().with_size(600, 400);
    let mut frame = frame::Frame::default()
        .with_size(400, 300)
        .center_of_parent();
    frame.set_frame(FrameType::FlatBox);
    frame.set_color(Color::Black);
    let mut but = button::Button::new(260, 355, 80, 40, "@+6>");
    win.end();
    win.show();

    but.set_callback(move |_| {
        let mut frame = frame.clone();
        thread::spawn(move || {
            let mut i = 1;
            loop {
                let file = format!("{}/{}.bmp", &*VIDEO_TEMP_DIR, i);
                if std::path::Path::new(&file).exists() {
                    let bmp = image::BmpImage::load(&file);
                    if let Ok(bmp) = bmp {
                        frame.set_image(Some(bmp));
                    }
                    fs::remove_file(file).unwrap();
                } else {
                    break;
                }
                app::sleep(0.001);
                frame.redraw();
                i += 1;
            }
        });
    });

    Ok(app.run()?)
}