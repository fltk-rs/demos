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
    std::{cell::RefCell, collections::HashMap, rc::Rc},
};

const NAME: &str = "FlPicture";
const PAD: i32 = 10;
const HEIGHT: i32 = 3 * PAD;
const WIDTH: i32 = 3 * HEIGHT;
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
    const WIDTH: i32 = 640;
    const HEIGHT: i32 = 360;
    let mut element = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(true);
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    let state = Rc::from(RefCell::from(Model::default()));
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
            state.borrow().save();
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
    let mut page = Flex::default_fill().column();
    {
        let mut header = Flex::default();
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
        crate::slider("Size", state.clone());
        crate::button("Next", &mut header)
            .with_label("@#>|")
            .set_callback(move |_| {
                app::handle_main(crate::NEXT).unwrap();
            });
        crate::button("Remove", &mut header)
            .with_label("@#1+")
            .set_callback(move |_| {
                app::handle_main(crate::REMOVE).unwrap();
            });
        header.end();
        header.set_pad(0);
        page.fixed(&header, HEIGHT);
        crate::canvas("Image", state.clone());
        page.fixed(&crate::browser("List", state.clone()), WIDTH);
    }
    page.end();
    page.set_pad(PAD);
    page.set_margin(PAD);
    page.set_frame(FrameType::FlatBox);
    page.handle(move |_, event| match event {
        REMOVE => {
            if !state.borrow().temp.is_empty() {
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
                "*.{png,svg}",
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
                        state.borrow_mut().temp.push(file);
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
    flex.fixed(&element, crate::HEIGHT);
    element
}

fn slider(tooltip: &str, state: Rc<RefCell<Model>>) {
    let mut element = Slider::default().with_type(SliderType::Horizontal);
    element.set_tooltip(tooltip);
    element.set_maximum(100f64);
    element.set_precision(0);
    element.set_value(element.maximum());
    element.set_callback(move |slider| {
        state.borrow_mut().size = slider.value() as i32;
        app::handle_main(HEARTBEAT).unwrap();
    });
}

fn canvas(tooltip: &str, state: Rc<RefCell<Model>>) {
    let cash = Rc::from(RefCell::from(HashMap::new()));
    let mut element = Frame::default_fill();
    element.set_frame(FrameType::DownBox);
    element.set_tooltip(tooltip);
    element.handle(move |frame, event| match event {
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
                let file = state.borrow().curr();
                if !cash.borrow().contains_key(&file) {
                    if let Ok(image) = SharedImage::load(&file) {
                        cash.borrow_mut().insert(file.clone(), image.clone());
                    }
                }
                let mut image = cash.borrow().get(&file).unwrap().clone();
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
        _ => false,
    });
}

fn browser(tooltip: &str, state: Rc<RefCell<Model>>) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    element.handle(move |browser, event| match event {
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
        _ => false,
    });
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
