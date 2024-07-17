#![forbid(unsafe_code)]

mod model;

use {
    fltk::{
        app,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{Color, Event, FrameType, Shortcut},
        frame::Frame,
        group::Flex,
        image::SharedImage,
        menu::{MenuButton, MenuFlag},
        prelude::*,
        valuator::{Slider, SliderType},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    model::Model,
    std::{env, fs, path::Path},
};

const HEIGHT: i32 = 30;
const PAD: i32 = 10;
const HEARTBEAT: i32 = 404;
const SHIFT: i32 = 405;
const OPEN: i32 = 406;
const REM: i32 = 407;

fn main() -> Result<(), FltkError> {
    app::GlobalState::<Model>::new(Model::new());
    let app = app::App::default();
    let mut window = crate::window();
    crate::view();
    window.end();
    window.show();
    app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
    app.run()
}

fn view() {
    let mut page = Flex::default_fill().column(); //Page
    {
        let mut header = Flex::default(); //HEADER
        crate::menu(&mut header);
        crate::button("Open", "@#fileopen", &mut header).handle(crate::open);
        crate::button("Prev", "@#|<", &mut header).handle(crate::shift);
        crate::slider("Size").set_callback(crate::size);
        crate::button("Next", "@#>|", &mut header).handle(crate::shift);
        crate::button("Remove", "@#1+", &mut header).handle(crate::remove);
        header.end();
        header.set_pad(0);
        page.fixed(&header, HEIGHT);
    }
    crate::frame("Image").handle(crate::show);
    crate::browser("List", &mut page).handle(crate::choice);
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

fn size(size: &mut Slider) {
    let size = size.value() as i32;
    app::GlobalState::<Model>::get().with(move |model| model.size = size);
    app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default_fill();
    element.set_frame(FrameType::DownBox);
    element.set_tooltip(tooltip);
    element
}

fn choice(browser: &mut Browser, event: Event) -> bool {
    if event == Event::from_i32(HEARTBEAT) {
        let (curr, temp) = app::GlobalState::<Model>::get()
            .with(move |model| (model.curr, model.temp.clone()));
        if !temp.is_empty() {
            browser.clear();
            for item in temp {
                browser.add(&item);
            }
            browser.select(curr as i32 + 1);
        }
        false
    } else if event == Event::Push {
        if browser.value() > 0 {
            let curr: usize = browser.value() as usize - 1;
            app::GlobalState::<Model>::get().with(move |model| model.choice(curr));
            app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
        };
        true
    } else {
        false
    }
}
fn show(frame: &mut Frame, event: Event) -> bool {
    if event == Event::from_i32(HEARTBEAT) {
        let model = app::GlobalState::<Model>::get().with(move |model| model.clone());
        if model.temp.is_empty() {
            frame.set_image(None::<SharedImage>);
        } else {
            let mut image = model.cash[&model.temp[model.curr]].clone();
            image.scale(
                frame.width() * model.size / 100,
                frame.height() * model.size / 100,
                true,
                true,
            );
            frame.set_image(Some(image));
            frame.redraw();
        };
        false
    } else {
        false
    }
}

fn browser(tooltip: &str, flex: &mut Flex) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    flex.fixed(&element, crate::HEIGHT * 2);
    element
}

fn remove(_: &mut Button, event: Event) -> bool {
    if [Event::from_i32(REM), Event::Push].contains(&event) {
        if !app::GlobalState::<Model>::get().with(move |model| model.empty()) {
            match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                Some(0) => app::GlobalState::<Model>::get().with(move |model| model.remove(false)),
                Some(2) => app::GlobalState::<Model>::get().with(move |model| model.remove(true)),
                _ => {}
            };
            app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
        };
        true
    } else {
        false
    }
}

fn shift(button: &mut Button, event: Event) -> bool {
    if [Event::from_i32(SHIFT), Event::Push].contains(&event) {
        button.deactivate();
        let label = button.label();
        app::GlobalState::<Model>::get().with(move |model| match label == "@#<" {
            true => model.dec(),
            false => model.inc(),
        });
        app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
        button.activate();
        true
    } else {
        false
    }
}

fn open(button: &mut Button, event: Event) -> bool {
    if [Event::from_i32(OPEN), Event::Push].contains(&event) {
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
            app::GlobalState::<Model>::get().with(move |model| {
                for item in 1..=dialog.count() {
                    if let Some(file) = dialog.value(item) {
                        model.temp.push(file);
                    };
                    model.choice(0);
                }
            });
        };
        app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
        true
    } else {
        false
    }
}

fn menu(flex: &mut Flex) {
    let mut element = MenuButton::default().with_label("@#menu");
    element.set_tooltip("Main menu");
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
            app::handle_main(crate::REM).unwrap();
        },
    );
    element.at(ord).unwrap().set_label_color(Color::Red);
    element.add(
        "&Image/@#>|  &Next",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            app::GlobalState::<Model>::get().with(move |model| model.inc());
            app::handle_main(crate::HEARTBEAT).unwrap();
        },
    );
    element.add(
        "&Image/@#|<  &Prev",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            app::GlobalState::<Model>::get().with(move |model| model.dec());
            app::handle_main(crate::HEARTBEAT).unwrap();
        },
    );
    flex.fixed(&element, 50);
}

fn window() -> Window {
    const DEFAULT: [u8; 4] = [
        2,   // [2] window_width * U8 +
        130, // [3] window_width_fract
        1,   // [4] window_height * U8 +
        105, // [5] window_height_fract
    ];
    const U8: i32 = 255;
    const NAME: &str = "FlPicture";
    let file: String = env::var("HOME").unwrap() + "/.config/" + NAME;
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
    element.make_resizable(true);
    element.handle(move |window, event| {
        if event == Event::from_i32(HEARTBEAT) {
            let (temp, curr) =
                app::GlobalState::<Model>::get().with(move |model| (model.temp.clone(), model.curr));
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
    ColorTheme::new(color_themes::DARK_THEME).apply();
    element
}
