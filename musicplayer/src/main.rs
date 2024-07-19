#![forbid(unsafe_code)]

use {
    fltk::{app, draw, enums::*, frame::*, group::*, image::*, prelude::*, valuator::*, window::*},
    soloud::{audio, AudioExt, LoadExt, Soloud},
    std::{cell::RefCell, rc::Rc},
};

const TRACK: &str = "assets/Alarm.mp3";
const HEIGHT: i32 = 30;

fn main() -> Result<(), FltkError> {
    let sl = Rc::from(RefCell::from(Soloud::default().unwrap()));
    let app = app::App::default();
    let mut wind = Window::default()
        .with_label("Music Player")
        .with_size(640, 360)
        .center_screen();

    let mut page = Flex::default_fill().column();
    Frame::default();
    crate::label(&mut page);
    Frame::default();
    crate::slider(&mut page).handle({
        let sl_clone = sl.clone();
        move |slider, ev| match ev {
            Event::Push => true,
            Event::Drag => {
                let slider_x = slider.x() as f32 / 50.0;
                let (x, _y) = app::event_coords();
                if x > 45 && x < 350 {
                    sl_clone.borrow_mut().set_global_volume(slider_x);
                }
                app::redraw();
                true
            }
            _ => false,
        }
    });
    Frame::default();
    let mut footer = Flex::default();
    Frame::default();
    crate::frame(&mut footer).set_callback({
        let sl_clone = sl.clone();
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
    Frame::default();
    footer.end();
    Frame::default();
    page.end();
    page.set_pad(30);
    page.set_margin(10);
    page.fixed(&footer, 80);
    wind.end();
    wind.make_resizable(true);
    wind.set_color(Color::Black);
    wind.show();
    wind.set_callback(move |_| {
        // Triggered when the window closes
        sl.borrow().stop_all(); // Stop any playing audio before quitting
        app.quit();
    });

    app.run()
}

fn label(flex: &mut Flex) {
    let mut element = Frame::default().with_label(TRACK).center_of_parent();
    element.set_label_color(Color::White);
    element.set_label_size(20);
    flex.fixed(&element, HEIGHT);
}

fn frame(flex: &mut Flex) -> Frame {
    const POWER: &str = r#"<?xml version="1.0" encoding="iso-8859-1"?>
<!-- Generator: Adobe Illustrator 19.1.0, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
     viewBox="0 0 315.083 315.083" style="enable-background:new 0 0 315.083 315.083;" xml:space="preserve">
<g id="Layer_1">
    <linearGradient id="SVGID_1_" gradientUnits="userSpaceOnUse" x1="157.5417" y1="4.5417" x2="157.5417" y2="310.5417">
        <stop  offset="0" style="stop-color:#939598"/>
        <stop  offset="0.25" style="stop-color:#414042"/>
        <stop  offset="0.5" style="stop-color:#252223"/>
        <stop  offset="1" style="stop-color:#000000"/>
    </linearGradient>
    <circle style="fill:url(#SVGID_1_);" cx="157.542" cy="157.542" r="153"/>
</g>
<g id="Layer_2">
    <linearGradient id="SVGID_2_" gradientUnits="userSpaceOnUse" x1="157.5417" y1="292.5417" x2="157.5417" y2="22.5417">
        <stop  offset="0" style="stop-color:#58595B"/>
        <stop  offset="0.1" style="stop-color:#414042"/>
        <stop  offset="0.2" style="stop-color:#242122"/>
        <stop  offset="1" style="stop-color:#000000"/>
    </linearGradient>
    <circle style="fill:url(#SVGID_2_);stroke:#58595B;stroke-miterlimit:10;" cx="157.542" cy="157.542" r="135"/>
</g>
<g id="Layer_4">
    <radialGradient id="SVGID_3_" cx="157.5417" cy="89.9217" r="62.2727" gradientUnits="userSpaceOnUse">
        <stop  offset="0" style="stop-color:#58595B"/>
        <stop  offset="0.5" style="stop-color:#414042"/>
        <stop  offset="1" style="stop-color:#231F20"/>
    </radialGradient>
    <radialGradient id="SVGID_4_" cx="157.5417" cy="89.9217" r="62.7723" gradientUnits="userSpaceOnUse">
        <stop  offset="0" style="stop-color:#FFFFFF"/>
        <stop  offset="0.6561" style="stop-color:#231F20"/>
        <stop  offset="1" style="stop-color:#000000"/>
    </radialGradient>

        <ellipse style="fill:url(#SVGID_3_);stroke:url(#SVGID_4_);stroke-miterlimit:10;" cx="157.542" cy="89.922" rx="59.833" ry="64.62"/>
</g>
<g id="Layer_6">
    <path style="fill:none;stroke:red;stroke-width:10;stroke-linecap:round;stroke-miterlimit:10;" d="M119.358,119.358
        c-9.772,9.772-15.816,23.272-15.816,38.184c0,14.912,6.044,28.412,15.816,38.184s23.272,15.816,38.184,15.816
        c14.912,0,28.412-6.044,38.184-15.816s15.816-23.272,15.816-38.184c0-14.912-6.044-28.412-15.816-38.184"/>

        <line style="fill:none;stroke:red;stroke-width:10;stroke-linecap:round;stroke-miterlimit:10;" x1="157.542" y1="154.542" x2="157.542" y2="100.542"/>
</g>
</svg>"#;
    let mut element = Frame::default();
    let on = Rc::from(RefCell::from(false));
    element.draw({
        let on = on.clone();
        move |frame| {
            let image_data = if *on.borrow() {
                POWER.to_string().replace("red", "green")
            } else {
                POWER.to_string()
            };
            let mut svg = SvgImage::from_data(&image_data).unwrap();
            svg.scale(frame.width(), frame.height(), true, true);
            svg.draw(frame.x(), frame.y(), frame.width(), frame.height());
        }
    });
    element.handle({
        let on = on.clone();
        move |frame, event| match event {
            Event::Push => {
                let prev = *on.borrow();
                *on.borrow_mut() = !prev;
                frame.do_callback();
                frame.redraw();
                true
            }
            _ => false,
        }
    });
    flex.fixed(&element, 80);
    element
}

fn slider(flex: &mut Flex) -> Slider {
    let mut element = Slider::default().with_type(SliderType::Horizontal);
    element.set_color(Color::from_u32(0x868db1));
    element.set_frame(FrameType::RFlatBox);
    element.draw(|slider| {
        draw::set_draw_color(Color::Blue);
        draw::draw_pie(
            slider.x() - 10 + (slider.w() as f64 * slider.value()) as i32,
            slider.y() - 10,
            30,
            30,
            0.,
            360.,
        );
    });
    flex.fixed(&element, 10);
    element
}
