#![forbid(unsafe_code)]

mod model;

use {
    fltk::{
        app,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{Color, Cursor, Event, FrameType, Shortcut},
        group::Flex,
        image::SvgImage,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        misc::Progress,
        prelude::*,
        valuator::{Slider, SliderType},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    model::Model,
    soloud::{audio::Wav, AudioExt, LoadExt, Soloud},
    std::{cell::RefCell, env, rc::Rc},
};

const HEARTBEAT: Event = Event::from_i32(404);
const REMOVE: Event = Event::from_i32(405);
const OPEN: Event = Event::from_i32(406);
const NEXT: Event = Event::from_i32(407);
const PREV: Event = Event::from_i32(408);
const PLAY: Event = Event::from_i32(409);
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

fn main() -> Result<(), FltkError> {
    let app = app::App::default();
    crate::window();
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::handle_main(HEARTBEAT).unwrap();
    app.run()
}

fn window() {
    const WIDTH: i32 = 640;
    const HEIGHT: i32 = 360;
    const NAME: &str = "FlMusic";
    let mut element = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(true);
    element.size_range(WIDTH, HEIGHT, 0, 0);
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    let file = env::var("HOME").unwrap() + "/.config/" + NAME;
    let state = Rc::from(RefCell::from(Model::default(&file)));
    let play = Rc::from(RefCell::from(
        Soloud::default().expect("Cannot access audio backend"),
    ));
    crate::view(state.clone(), play.clone());
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            play.borrow().stop_all();
            state.borrow().save(&file);
            app::quit();
        }
    });
    element.end();
    element.show();
}

fn view(state: Rc<RefCell<Model>>, play: Rc<RefCell<Soloud>>) {
    let mut page = Flex::default_fill().column();
    {
        let mut header = Flex::default_fill();
        {
            let mut buttons = Flex::default_fill();
            {
                crate::button("Open", &mut header)
                    .with_label("@#fileopen")
                    .set_callback(move |_| {
                        app::handle_main(crate::OPEN).unwrap();
                    });
                crate::button("Prev", &mut header)
                    .with_label("@#|<")
                    .set_callback(move |_| {
                        app::handle_main(crate::PREV).unwrap();
                    });
                crate::button("Play", &mut header)
                .with_label("@#>")
                .handle(glib::clone!(@strong state, @strong play => move |button, event| {
                    if [PLAY, Event::Push].contains(&event) {
                        if ! state.borrow().list.is_empty() {
                            if play.borrow().active_voice_count() > 0 {
                                button.set_label("@#>");
                                button.set_tooltip("Start");
                                play.borrow().stop_all();
                            } else {
                                button.set_label("@#||");
                                button.set_tooltip("Stop");
                                let mut wav = Wav::default();
                                if wav.load(&state.borrow().list[state.borrow().curr]).is_ok() {
                                    state.borrow_mut().duration = wav.length();
                                    let handle = play.borrow().play(&wav);
                                    while play.borrow().active_voice_count() > 0 {
                                        app::wait();
                                        app::sleep(0.02);
                                        state.borrow_mut().time = play.borrow().stream_time(handle);
                                        app::handle_main(HEARTBEAT).unwrap();
                                    }
                                }
                            };
                        }
                        true
                    } else {
                        false
                    }
                }));
                crate::button("Next", &mut header)
                    .with_label("@#>|")
                    .set_callback(move |_| {
                        app::handle_main(crate::NEXT).unwrap();
                    });
            }
            buttons.end();
            buttons.set_pad(0);
            header.fixed(&buttons, HEIGHT * buttons.children());
            crate::progress("Progress", state.clone());
            header.fixed(&crate::slider("Volume", play.clone()), 150);
            crate::button("Remove", &mut header)
                .with_label("@#1+")
                .set_callback(move |_| {
                    app::handle_main(crate::REMOVE).unwrap();
                });
        }
        header.end();
        header.set_pad(0);
        page.fixed(&header, HEIGHT);
        crate::browser("List", state.clone());
    }
    page.end();
    page.set_pad(PAD);
    page.set_margin(PAD);
    page.set_frame(FrameType::FlatBox);
    page.handle(move |_, event| match event {
        REMOVE => {
            if !state.borrow().list.is_empty() {
                match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                    Some(0) => state.borrow_mut().remove(false),
                    Some(2) => state.borrow_mut().remove(true),
                    _ => {}
                };
                app::handle_main(HEARTBEAT).unwrap();
            };
            true
        }
        OPEN => {
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
                for item in 1..=dialog.count() {
                    if let Some(file) = dialog.value(item) {
                        state.borrow_mut().list.push(file);
                    };
                    state.borrow_mut().choice(0);
                }
            };
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        NEXT => {
            state.borrow_mut().inc();
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        PREV => {
            state.borrow_mut().dec();
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        _ => false,
    });
}

fn button(tooltip: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default();
    element.set_tooltip(tooltip);
    flex.fixed(&element, HEIGHT);
    element
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default().with_type(MenuButtonType::Popup3);
    element.add(
        "@#fileopen  &Open...",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(OPEN).unwrap();
        },
    );
    element.add(
        "@#>  &Play",
        Shortcut::Ctrl | 'p',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(PLAY).unwrap();
        },
    );
    element.add(
        "@#|>  &Next",
        Shortcut::Ctrl | 'k',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(NEXT).unwrap();
        },
    );
    element.add(
        "@#<|  &Prev",
        Shortcut::Ctrl | 'j',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(PREV).unwrap();
        },
    );
    element.add(
        "@#1+  &Remove",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            app::handle_main(REMOVE).unwrap();
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

fn slider(tooltip: &str, play: Rc<RefCell<Soloud>>) -> Slider {
    let mut element = Slider::default().with_type(SliderType::Horizontal);
    element.set_tooltip(tooltip);
    element.set_maximum(3_f64);
    element.set_value(element.maximum());
    element.set_callback(move |slider| play.borrow_mut().set_global_volume(slider.value() as f32));
    element
}

fn progress(tooltip: &str, state: Rc<RefCell<Model>>) {
    let mut element = Progress::default();
    element.set_tooltip(tooltip);
    element.handle(move |progress, event| {
        if event == HEARTBEAT {
            progress.set_maximum(state.borrow().duration);
            progress.set_value(state.borrow().time);
        }
        false
    });
}

fn browser(tooltip: &str, state: Rc<RefCell<Model>>) {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    element.handle(move |browser, event| match event {
        HEARTBEAT => {
            let (curr, temp) = (state.borrow().curr, &state.borrow().list);
            if !temp.is_empty() {
                browser.clear();
                for item in temp {
                    browser.add(item);
                }
                browser.select(curr as i32 + 1);
            }
            false
        }
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                crate::menu().popup();
                true
            }
            _ => {
                if browser.value() > 0 {
                    state.borrow_mut().choice(browser.value() as usize - 1);
                    app::handle_main(HEARTBEAT).unwrap();
                };
                true
            }
        },
        Event::Enter => {
            browser.window().unwrap().set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            browser.window().unwrap().set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    });
}
