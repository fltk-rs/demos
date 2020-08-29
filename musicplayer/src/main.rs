use fltk::{app, frame::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;
use soloud::*;

mod power_button;
use power_button::PowerButton;

mod fancy_slider;
use fancy_slider::FancySlider;

const TRACK: &str = "Alarm.mp3";

fn main() {
    let app = app::App::default();
    let mut wind = DoubleWindow::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("Music Player");

    let mut frm = Frame::new(160, 80, 80, 40, TRACK);
    frm.set_label_size(20);
    frm.set_label_color(Color::White);
    let mut slider = FancySlider::new(50, 150);
    let mut but = PowerButton::new(160, 210);

    let sl = Soloud::default().unwrap();

    wind.set_color(Color::Black);
    wind.end();
    wind.show();

    let sl = Rc::from(RefCell::from(sl));
    let sl_c = sl.clone();
    but.set_callback(Box::new(move || {
        if sl_c.borrow().active_voice_count() > 0 { // Checks that no active audio is playing
            sl_c.borrow().stop_all();
            return;
        }
    	let mut wav = audio::Wav::default();
        wav.load(&std::path::Path::new(TRACK)).unwrap();
        wav.set_looping(true);
        sl_c.borrow().play(&wav);
        while sl_c.borrow().active_voice_count() > 0 {
            app.wait().unwrap();
        }
    }));

    let sl_c = sl.clone();
    let mut slider_c =slider.clone();
    slider.handle(Box::new(move |ev| match ev {
        Event::Push => true,
        Event::Drag => {
            let slider_x = slider_c.x() as f32 / 50.0;
            let (x, _y) = app::event_coords();
            if x > 45 && x < 350 {
                slider_c.set_pos(x - 15, 150);
                sl_c.borrow_mut().set_global_volume(slider_x);
            }
            app::redraw();
            true
        },
        _ => false,
    }));

    wind.set_callback(Box::new(move || { // Callback when an app closes
        sl.borrow().stop_all(); // Stop any playing audio before quitting
        app.quit();
    }));

    app.run().unwrap();
}
