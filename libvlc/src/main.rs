#![forbid(unsafe_code)]
use fltk::{enums::Color, prelude::*, *};
use vlc::*;

#[derive(Copy, Clone)]
pub enum Message {
    Play,
    Stop,
}

fn main() {
    let app = app::App::default();
    let (sender, receiver) = app::channel::<Message>();
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");
    win.make_resizable(true);

    // Create inner window to act as embedded media player
    let mut vlc_win = window::Window::new(10, 10, 780, 520, "");
    vlc_win.end();
    vlc_win.set_color(Color::Black);

    button::Button::new(320, 545, 80, 40, "Play").emit(sender, Message::Play);
    button::Button::new(400, 545, 80, 40, "Stop").emit(sender, Message::Stop);

    win.end();
    win.show();
    win.make_resizable(true);

    // Instantiate vlc instance and media player
    let instance = Instance::new().unwrap();
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&Media::new_path(&instance, "video.mp4").unwrap());

    // Get vlc_win handle that we'll pass to libvlc
    // Linux u32, windows HWND, Mac NSWindow
    let handle = vlc_win.raw_handle();

    // Pass the handle to vlc
    // Method depends on the platform
    // For Linux
    #[cfg(target_os = "linux")]
    mdp.set_xwindow(handle as u32);
    // For Windows
    #[cfg(target_os = "windows")]
    mdp.set_hwnd(handle);
    // For MacOS
    #[cfg(target_os = "macos")]
    mdp.set_nsobject(utils::content_view(&vlc_win) as _);

    // Disable event handling on vlc's side
    // Do it thru fltk
    mdp.set_key_input(false);
    mdp.set_mouse_input(false);

    while app.wait() {
        if let Some(val) = receiver.recv() {
            match val {
                Message::Play => mdp.play().unwrap(),
                Message::Stop => mdp.stop(),
            }
        }
    }
}
