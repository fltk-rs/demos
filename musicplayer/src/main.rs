use fltk::{
    app,
    enums::*,
    frame::*,
    prelude::*,
    window::*,
    image::IcoImage
};
use soloud::*;
use std::cell::RefCell;
use std::rc::Rc;

mod power_button;
use power_button::PowerButton;

mod fancy_slider;
use fancy_slider::FancySlider;

const TRACK: &str = "Alarm.mp3";

fn main() {
    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    let app = app::App::default();
    let mut wind = DoubleWindow::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("Music Player");
    wind.make_resizable(true);
    wind.set_icon(Some(icon));

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

    but.set_callback({
        let sl = sl.clone();
        move |_| {
            if sl.borrow().active_voice_count() > 0 {
                // Checks that no active audio is playing
                sl.borrow().stop_all();
                return;
            }
            let mut wav = audio::Wav::default();
            wav.load(&std::path::Path::new(TRACK)).unwrap();
            wav.set_looping(true);
            sl.borrow().play(&wav);
            while sl.borrow().active_voice_count() > 0 {
                app.wait();
            }
        }
    });

    slider.handle({
        let sl = sl.clone();
        move |s, ev| match ev {
            Event::Push => true,
            Event::Drag => {
                let slider_x = s.x() as f32 / 50.0;
                let (x, _y) = app::event_coords();
                if x > 45 && x < 350 {
                    s.set_pos(x - 15, 150);
                    sl.borrow_mut().set_global_volume(slider_x);
                }
                app::redraw();
                true
            }
            _ => false,
        }
    });

    wind.set_callback(move |_| {
        // Triggered when the window closes
        sl.borrow().stop_all(); // Stop any playing audio before quitting
        app.quit();
    });

    app.run().unwrap();
}
