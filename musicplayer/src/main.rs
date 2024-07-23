#![forbid(unsafe_code)]

use {
    fltk::{app, draw, enums::*, frame::*, group::*, image::*, prelude::*, valuator::*, window::*},
    soloud::{audio, AudioExt, LoadExt, Soloud},
    std::{cell::RefCell, rc::Rc},
};

const TRACK: &str = "../../assets/Alarm.mp3";
const PAD: i32 = 10;
const HEIGHT: i32 = 3 * PAD;
const WIDTH: i32 = 3 * HEIGHT;

fn main() -> Result<(), FltkError> {
    let state = Rc::from(RefCell::from(Soloud::default().unwrap()));
    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("Music Player")
        .with_size(640, 360)
        .center_screen();

    let mut page = Flex::default_fill().column();
    {
        Frame::default();
        page.fixed(&crate::label(), HEIGHT);
        page.fixed(&crate::slider(state.clone()), PAD);
        let mut footer = Flex::default();
        {
            Frame::default();
            footer.fixed(&crate::button(state.clone()), WIDTH);
            Frame::default();
        }
        footer.end();
        page.fixed(&footer, WIDTH);
        Frame::default();
    }
    page.end();
    page.set_pad(WIDTH);
    page.set_margin(HEIGHT);
    wind.end();
    wind.make_resizable(true);
    wind.set_color(Color::Black);
    wind.show();
    wind.set_callback(move |_| {
        // Triggered when the window closes
        state.borrow().stop_all(); // Stop any playing audio before quitting
        app.quit();
    });

    app.run()
}

fn label() -> Frame {
    let mut element = Frame::default().with_label(TRACK).center_of_parent();
    element.set_label_color(Color::White);
    element.set_label_size(HEIGHT);
    element
}

fn button(state: Rc<RefCell<Soloud>>) -> Frame {
    const POWER: &str = include_str!("../../assets/button.svg");
    let mut element = Frame::default();
    element.set_tooltip("start");
    element.handle(move |frame, event| match event {
        Event::Push => {
            if state.borrow().active_voice_count() > 0 {
                // Checks that no active audio is playing
                frame.set_tooltip("play");
                frame.redraw();
                state.borrow().stop_all();
            } else {
                frame.set_tooltip("stop");
                frame.redraw();
                let mut wav = audio::Wav::default();
                if wav.load(TRACK).is_ok() {
                    wav.set_looping(true);
                    state.borrow().play(&wav);
                    while state.borrow().active_voice_count() > 0 {
                        app::wait();
                        app::sleep(0.02);
                    }
                }
            }
            true
        }
        _ => false,
    });
    element.draw(move |frame| {
        let image_data = match frame.tooltip().unwrap().as_str() {
            "stop" => POWER.to_string().replace("red", "green"),
            _ => POWER.to_string(),
        };
        let mut svg = SvgImage::from_data(&image_data).unwrap();
        svg.scale(frame.width(), frame.height(), true, true);
        svg.draw(frame.x(), frame.y(), frame.width(), frame.height());
    });
    element
}

fn slider(state: Rc<RefCell<Soloud>>) -> Slider {
    let mut element = Slider::default().with_type(SliderType::Horizontal);
    element.set_color(Color::from_u32(0x868db1));
    element.set_frame(FrameType::RFlatBox);
    element.set_maximum(3.0);
    element.set_value(0.0);
    element.draw(move |slider| {
        draw::set_draw_color(Color::Blue);
        draw::draw_pie(
            (slider.x() - HEIGHT / 2)
                + (slider.value() * (slider.w()) as f64 / slider.maximum()) as i32,
            slider.y() - slider.h(),
            HEIGHT,
            HEIGHT,
            0_f64,
            360_f64,
        );
    });
    element.handle(move |slider, event| match event {
        Event::Push => true,
        Event::Drag => {
            if (slider.x()..slider.w() + slider.x()).contains(&app::event_coords().0) {
                state.borrow_mut().set_global_volume(slider.value() as f32);
            }
            app::redraw();
            true
        }
        _ => false,
    });
    element
}
