#![forbid(unsafe_code)]

mod model;

use {
    fltk::{
        app,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{Color, Cursor, Event, FrameType, Shortcut},
        frame::Frame,
        group::Flex,
        image::{SharedImage, SvgImage},
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        valuator::{Slider, SliderType},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    model::Model,
    std::{cell::RefCell, env, fs, path::Path, rc::Rc},
};

const PAD: i32 = 10;
const HEIGHT: i32 = 3 * PAD;
const HEARTBEAT: Event = Event::from_i32(404);
const NEXT: Event = Event::from_i32(405);
const PREV: Event = Event::from_i32(406);
const OPEN: Event = Event::from_i32(407);
const REMOVE: Event = Event::from_i32(408);

fn main() -> Result<(), FltkError> {
    let app = app::App::default();
    crate::window();
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::handle_main(HEARTBEAT).unwrap();
    app.run()
}

fn window() {
    const DEFAULT: [u8; 4] = [
        2,   // [2] window_width * U8 +
        130, // [3] window_width_fract
        1,   // [4] window_height * U8 +
        105, // [5] window_height_fract
    ];
    const U8: i32 = 255;
    const NAME: &str = "FlPicture";
    let file: String = env::var("HOME").unwrap() + "/.config/" + NAME;
    let state = Rc::from(RefCell::from(Model::default()));
    let params: Vec<u8> = if Path::new(&file).exists() {
        if let Ok(value) = fs::read(&file) {
            if value.len() == DEFAULT.len() {
                value
            } else {
                Vec::from(DEFAULT)
            }
        } else {
            Vec::from(DEFAULT)
        }
    } else {
        Vec::from(DEFAULT)
    };
    let mut element = Window::default()
        .with_size(
            params[0] as i32 * U8 + params[1] as i32,
            params[2] as i32 * U8 + params[3] as i32,
        )
        .center_screen();
    element.size_range(
        DEFAULT[0] as i32 * U8 + DEFAULT[1] as i32,
        DEFAULT[2] as i32 * U8 + DEFAULT[3] as i32,
        0,
        0,
    );
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/icon.svg")).unwrap(),
    ));
    element.make_resizable(true);
    crate::view(state.clone());
    element.handle(move |window, event| {
        if event == HEARTBEAT {
            let (temp, curr) = (&state.borrow().temp, state.borrow().curr);
            match temp.is_empty() {
                true => window.set_label(NAME),
                false => window.set_label(&format!("{} - {NAME}", temp[curr].clone())),
            };
            false
        } else if app::event() == Event::Close {
            fs::write(
                &file,
                [
                    (window.width() / U8) as u8,
                    (window.width() % U8) as u8,
                    (window.height() / U8) as u8,
                    (window.height() % U8) as u8,
                ],
            )
            .unwrap();
            app::quit();
            true
        } else {
            false
        }
    });
    element.end();
    element.show();
}

fn view(state: Rc<RefCell<Model>>) {
    let mut page = Flex::default_fill().column(); //Page
    {
        let mut header = Flex::default(); //HEADER
        crate::button("Open", "@#fileopen", &mut header).handle(
            glib::clone!(@strong state => move |button, event| {
                if [OPEN, Event::Push].contains(&event) {
                    button.deactivate();
                    let mut dialog = FileChooser::new(
                        std::env::var("HOME").unwrap(),
                        "*.{png,svg}",
                        FileChooserType::Multi,
                        "Choose File...",
                    );
                    dialog.show();
                    button.activate();
                    while dialog.shown() {
                        app::wait();
                    }
                    if dialog.count() > 0 {
                        for item in 1..=dialog.count() {
                            if let Some(file) = dialog.value(item) {
                                state.borrow_mut().temp.push(file);
                            };
                            state.borrow_mut().choice(0);
                        }
                    };
                    app::handle_main(HEARTBEAT).unwrap();
                    true
                } else {
                    false
                }
            }),
        );
        crate::button("Prev", "@#|<", &mut header).handle(
            glib::clone!(@strong state => move |button, event| {
                if [PREV, Event::Push].contains(&event) {
                    button.deactivate();
                    state.borrow_mut().dec();
                    app::handle_main(HEARTBEAT).unwrap();
                    button.activate();
                    true
                } else {
                    false
                }
            }),
        );
        crate::slider("Size").set_callback(glib::clone!(@strong state => move |slider| {
            state.borrow_mut().size = slider.value() as i32;
            app::handle_main(HEARTBEAT).unwrap();
        }));
        crate::button("Next", "@#>|", &mut header).handle(
            glib::clone!(@strong state => move |button, event| {
                if [NEXT, Event::Push].contains(&event) {
                    button.deactivate();
                    state.borrow_mut().inc();
                    app::handle_main(HEARTBEAT).unwrap();
                    button.activate();
                    true
                } else {
                    false
                }
            }),
        );
        crate::button("Remove", "@#1+", &mut header).handle(
            glib::clone!(@strong state => move |button, event| {
                if [REMOVE, Event::Push].contains(&event) {
                    if ! state.borrow().temp.is_empty() {
                        button.deactivate();
                        match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                            Some(0) => state.borrow_mut().remove(false),
                            Some(2) => state.borrow_mut().remove(true),
                            _ => {}
                        };
                        app::handle_main(HEARTBEAT).unwrap();
                        button.activate();
                    };
                    true
                } else {
                    false
                }
            }),
        );
        header.end();
        header.set_pad(0);
        page.fixed(&header, HEIGHT);
        crate::frame("Image").handle(glib::clone!(@strong state => move |frame, event| {
            match event {
                Event::Push => match app::event_mouse_button() {
                    app::MouseButton::Right => {
                        crate::menu().popup();
                        true
                    }
                    _ => false,
                },
                Event::Enter => {
                    frame.window().unwrap().set_cursor(Cursor::Hand);
                    true
                }
                Event::Leave => {
                    frame.window().unwrap().set_cursor(Cursor::Arrow);
                    true
                }
                HEARTBEAT => {
                    if state.borrow().temp.is_empty() {
                        frame.set_image(None::<SharedImage>);
                    } else {
                        let mut image = state.borrow().cash[&state.borrow().temp[state.borrow().curr]].clone();
                        image.scale(
                            frame.width() * state.borrow().size / 100,
                            frame.height() * state.borrow().size / 100,
                            true,
                            true,
                        );
                        frame.set_image(Some(image));
                        frame.redraw();
                    };
                    false
                }
                _ => false
            }
        }));
        crate::browser("List", &mut page).handle(
            glib::clone!(@strong state => move |browser, event| {
                match event {
                    HEARTBEAT => {
                        let (curr, temp) = (state.borrow().curr, &state.borrow().temp);
                        if !temp.is_empty() {
                            browser.clear();
                            for item in temp {
                                browser.add(item);
                            }
                            browser.select(curr as i32 + 1);
                        }
                        false
                    }
                    Event::Push => {
                        if browser.value() > 0 {
                            state.borrow_mut().choice(browser.value() as usize - 1);
                            app::handle_main(HEARTBEAT).unwrap();
                        };
                        true
                    }
                    _ => false
                }
            }),
        );
    }
    page.end();
    page.set_pad(PAD);
    page.set_margin(PAD);
    page.set_frame(FrameType::FlatBox);
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    flex.fixed(&element, crate::HEIGHT);
    element
}

fn slider(tooltip: &str) -> Slider {
    let mut element = Slider::default().with_type(SliderType::Horizontal);
    element.set_tooltip(tooltip);
    element.set_maximum(100f64);
    element.set_precision(0);
    element.set_value(element.maximum());
    element
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default_fill();
    element.set_frame(FrameType::DownBox);
    element.set_tooltip(tooltip);
    element
}

fn browser(tooltip: &str, flex: &mut Flex) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    flex.fixed(&element, crate::HEIGHT * 2);
    element
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default().with_type(MenuButtonType::Popup3);
    element.add(
        "&File/@#fileopen  &Open",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::OPEN).unwrap();
        },
    );
    let ord: i32 = element.add(
        "&File/@#1+  &Remove",
        Shortcut::Ctrl | 'd',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::REMOVE).unwrap();
        },
    );
    element.at(ord).unwrap().set_label_color(Color::Red);
    element.add(
        "&Image/@#>|  &Next",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::NEXT).unwrap();
        },
    );
    element.add(
        "&Image/@#|<  &Prev",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::PREV).unwrap();
        },
    );
    element
}
