use fltk::{enums::Color, prelude::*, *};
use gstreamer_video::prelude::*;

#[derive(Copy, Clone)]
pub enum Message {
    Play,
    Stop,
}

fn main() {
    let app = app::App::default();
    gstreamer::init().unwrap();
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");
    win.make_resizable(true);

    // Create inner window to act as embedded media player
    let mut gst_win = window::Window::new(10, 10, 780, 520, "");
    gst_win.end();
    gst_win.set_color(Color::Black);

    let mut but_play = button::Button::new(320, 545, 80, 40, "@>");
    let mut but_stop = button::Button::new(400, 545, 80, 40, "@||");

    win.end();
    win.show();
    win.make_resizable(true);

    let handle = gst_win.raw_handle();

    // gstreamer requires a uri
    let uri = "../libvlc/video.mp4".to_owned();
    let mut path = String::from("file:///");
    let current_dir = std::env::current_dir().unwrap();
    let video_file = current_dir.join(uri);
    path += video_file.to_str().unwrap();

    let playbin = gstreamer::ElementFactory::make("playbin", None).unwrap();
    playbin.set_property("uri", &path).unwrap();
    let video_overlay = playbin
        .clone()
        .dynamic_cast::<gstreamer_video::VideoOverlay>()
        .unwrap();

    unsafe {
        video_overlay.set_window_handle(handle as _);
    }

    let (s, r) = app::channel::<Message>();

    but_play.emit(s, Message::Play);
    but_stop.emit(s, Message::Stop);

    while app.wait() {
        if let Some(val) = r.recv() {
            match val {
                Message::Play => {
                    playbin.set_state(gstreamer::State::Playing).ok();
                }
                Message::Stop => {
                    playbin.set_state(gstreamer::State::Paused).ok();
                }
            }
        }
    }
}
