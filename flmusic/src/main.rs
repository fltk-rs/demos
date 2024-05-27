#![forbid(unsafe_code)]

use {
    fltk::{
        app,
        app::WidgetId,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{Color, Event, FrameType, Shortcut},
        group::Flex,
        menu::{MenuButton, MenuFlag},
        misc::Progress,
        prelude::*,
        valuator::{Slider, SliderType},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    soloud::{audio::Wav, AudioExt, LoadExt, Soloud},
    std::{cell::RefCell, env, fs, path::Path, rc::Rc},
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const PREV: &str = "Prev";
const NEXT: &str = "Next";
const PLAY: &str = "Play";
const LIST: &str = "Browser";
const DIAL: &str = "Progress";
const VOL: &str = "Volume";

fn main() -> Result<(), FltkError> {
    let app = app::App::default();
    let player = Rc::from(RefCell::from(
        Soloud::default().expect("Cannot access audio backend"),
    ));
    let mut window = crate::window(player.clone());
    let mut page = Flex::default_fill().column(); //PAGE

    let mut header = Flex::default_fill(); //HEADER
    header.fixed(&crate::menu(), 50);
    let mut buttons = Flex::default_fill();
    crate::button(crate::PREV, "@#|<", &mut header).set_callback(crate::prev);
    crate::button(crate::PLAY, "@#>", &mut header).set_callback({
        let player = player.clone();
        move |play| {
            let browser = app::widget_from_id::<Browser>(crate::LIST).unwrap();
            let mut song = app::widget_from_id::<Progress>(crate::DIAL).unwrap();
            let vol = app::widget_from_id::<Slider>(crate::VOL).unwrap();
            if browser.size() > 0 {
                if player.borrow().active_voice_count() > 0 {
                    play.set_label("@#>");
                    play.set_tooltip("Start");
                    player.borrow().stop_all();
                } else {
                    play.set_label("@#||");
                    play.set_tooltip("Stop");
                    let mut wav = Wav::default();
                    if wav
                        .load(Path::new(&browser.selected_text().unwrap()))
                        .is_ok()
                    {
                        song.set_maximum(wav.length());
                        let handle = player.borrow().play(&wav);
                        while player.borrow().active_voice_count() > 0 {
                            app::wait();
                            app::sleep(0.01);
                            player.borrow_mut().set_volume(handle, vol.value() as f32);
                            song.set_value(player.borrow().stream_time(handle));
                            song.set_label(&format!(
                                "{:.1}%",
                                wav.length() / 600_f64 * player.borrow().stream_time(handle)
                            ));
                        }
                    }
                }
            }
        }
    });
    crate::button("Next", "@#>|", &mut header).set_callback(crate::next);
    buttons.end();
    crate::progress(crate::DIAL, false);
    crate::slider(crate::VOL, 6_f64, false, &mut header)
        .with_type(SliderType::Horizontal)
        .set_callback({
            let player = player.clone();
            move |slider| player.borrow_mut().set_global_volume(slider.value() as f32)
        });
    header.end();
    crate::browser(crate::LIST);
    page.end();
    window.end();
    window.show();
    {
        buttons.set_pad(0);
        header.fixed(&buttons, 90);
        header.set_pad(0);
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.fixed(&header, HEIGHT);
        page.set_frame(FrameType::FlatBox);
        ColorTheme::new(color_themes::DARK_THEME).apply();
    }
    app.run()
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label).with_id(tooltip);
    element.set_tooltip(tooltip);
    flex.fixed(&element, 26);
    element
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default().with_label("@#menu");
    element.add(
        "@#+  &Add",
        Shortcut::Ctrl | 'a',
        MenuFlag::Normal,
        move |_| {
            let mut dialog = FileChooser::new(
                std::env::var("HOME").unwrap(),
                "*.{mp3}",
                FileChooserType::Multi,
                "Choose File...",
            );
            dialog.show();
            while dialog.shown() {
                app::wait();
            }
            if dialog.count() > 0 {
                let mut browser = app::widget_from_id::<Browser>(crate::LIST).unwrap();
                for item in 1..=dialog.count() {
                    if let Some(file) = dialog.value(item) {
                        browser.add(&file);
                    };
                }
                browser.sort();
                browser.select(1);
                app::widget_from_id::<Button>(crate::PLAY)
                    .unwrap()
                    .activate();
                app::widget_from_id::<Button>(crate::NEXT)
                    .unwrap()
                    .activate();
                app::widget_from_id::<Button>(crate::PREV)
                    .unwrap()
                    .activate();
                app::widget_from_id::<Slider>(crate::VOL)
                    .unwrap()
                    .activate();
                app::widget_from_id::<Progress>(crate::DIAL)
                    .unwrap()
                    .activate();
            };
        },
    );
    element.add(
        "@#>  &Play",
        Shortcut::Ctrl | 'p',
        MenuFlag::Normal,
        move |_| {
            app::widget_from_id::<Button>(crate::PLAY)
                .unwrap()
                .do_callback()
        },
    );
    element.add(
        "@#|>  &Next",
        Shortcut::Ctrl | 'k',
        MenuFlag::Normal,
        move |_| app::widget_from_id::<Button>(NEXT).unwrap().do_callback(),
    );
    element.add(
        "@#<|  &Prev",
        Shortcut::Ctrl | 'j',
        MenuFlag::Normal,
        move |_| {
            app::widget_from_id::<Button>(crate::PREV)
                .unwrap()
                .do_callback()
        },
    );
    element.add(
        "@#1+  &Remove",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            let mut browser = app::widget_from_id::<Browser>(crate::LIST).unwrap();
            match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                Some(0) => browser.remove(browser.value()),
                Some(2) => {
                    if fs::remove_file(browser.selected_text().unwrap()).is_ok() {
                        browser.remove(browser.value());
                    }
                }
                _ => {}
            };
        },
    );
    let ord: i32 = element.add(
        "@#1+  &Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(Event::Close).unwrap();
        },
    );
    element.at(ord).unwrap().set_label_color(Color::Red);
    element
}

fn slider(tooltip: &str, maximum: f64, state: bool, flex: &mut Flex) -> Slider {
    let mut element = Slider::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_maximum(maximum);
    element.set_value(maximum / 2_f64);
    match state {
        true => element.activate(),
        false => element.deactivate(),
    };
    flex.fixed(&element, 150);
    element
}

fn progress(tooltip: &str, state: bool) {
    let mut element = Progress::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_selection_color(Color::Black);
    match state {
        true => element.activate(),
        false => element.deactivate(),
    };
}

fn browser(tooltip: &str) {
    let mut element = Browser::default()
        .with_type(BrowserType::Hold)
        .with_id(tooltip);
    element.set_tooltip(tooltip);
    let file = env::var("HOME").unwrap() + "/.config/" + "FlMusic.bin";
    let model: Vec<String> = if Path::new(&file).exists() {
        if let Ok(value) = fs::read(&file) {
            rmp_serde::from_slice(&value).unwrap()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    for item in model {
        element.add(&item);
    }
    if element.size() > 0 {
        element.select(1);
    }
}

fn window(player: Rc<RefCell<Soloud>>) -> Window {
    const NAME: &str = "FlMusic";
    let mut element = Window::default()
        .with_size(640, 360)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(true);
    element.size_range(640, 360, 0, 0);
    element.set_xclass(NAME);
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            let file = env::var("HOME").unwrap() + "/.config/" + NAME;
            let browser = app::widget_from_id::<Browser>(crate::LIST).unwrap();
            fs::write(
                file,
                rmp_serde::to_vec(
                    &(1..=browser.size())
                        .map(|idx| browser.text(idx).unwrap())
                        .collect::<Vec<String>>(),
                )
                .unwrap(),
            )
            .unwrap();
            player.borrow().stop_all();
            app::quit();
        }
    });
    element
}

fn next(_: &mut Button) {
    let mut browser = app::widget_from_id::<Browser>(crate::LIST).unwrap();
    match browser.value() < browser.size() {
        true => browser.select(browser.value() + 1),
        false => browser.select(1),
    };
    browser.do_callback();
}

fn prev(_: &mut Button) {
    let mut browser = app::widget_from_id::<Browser>(crate::LIST).unwrap();
    match browser.value() > 1 {
        true => browser.select(browser.value() - 1),
        false => browser.select(browser.size()),
    };
    browser.do_callback();
}
