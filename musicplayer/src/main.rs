#![forbid(unsafe_code)]
mod fancy_slider;
mod power_button;

use {
    fltk::{app, enums::*, frame::*, prelude::*, window::*},
    soloud::{audio, AudioExt, LoadExt, Soloud},
    std::{cell::RefCell, rc::Rc},
};

fn main() {
    const TRACK: &str = "assets/Alarm.mp3";
    let app = app::App::default();
    let mut wind = DoubleWindow::default()
        .with_label("Music Player")
        .with_size(400, 300)
        .center_screen();

    let mut frm = Frame::new(160, 80, 80, 40, TRACK);
    frm.set_label_color(Color::White);
    frm.set_label_size(20);
    let mut slider = fancy_slider::FancySlider::new(50, 150);
    let mut but = power_button::PowerButton::new(160, 210);

    wind.end();
    wind.make_resizable(true);
    wind.set_color(Color::Black);
    wind.show();

    let sl = Rc::from(RefCell::from(Soloud::default().unwrap()));

    let sl_clone = sl.clone();
    but.set_callback({
        move |_| {
            if sl_clone.borrow().active_voice_count() > 0 {
                // Checks that no active audio is playing
                sl_clone.borrow().stop_all();
            } else {
                let mut wav = audio::Wav::default();
                if wav.load(std::path::Path::new(TRACK)).is_ok() {
                    wav.set_looping(true);
                    sl_clone.borrow().play(&wav);
                    while sl_clone.borrow().active_voice_count() > 0 {
                        app::wait();
                    }
                }
            }
        }
    });

    let sl_clone = sl.clone();
    slider.handle(move |slider, ev| match ev {
        Event::Push => true,
        Event::Drag => {
            let slider_x = slider.x() as f32 / 50.0;
            let (x, _y) = app::event_coords();
            if x > 45 && x < 350 {
                slider.set_pos(x - 15, 150);
                sl_clone.borrow_mut().set_global_volume(slider_x);
            }
            app::redraw();
            true
        }
        _ => false,
    });

    wind.set_callback(move |_| {
        // Triggered when the window closes
        sl.borrow().stop_all(); // Stop any playing audio before quitting
        app.quit();
    });

    app.run().unwrap();
}
