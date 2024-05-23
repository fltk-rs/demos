#![forbid(unsafe_code)]

use {
    fltk::{
        app,
        app::WidgetId,
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
    std::{collections::HashMap, env, fs, path::Path},
};

const HEIGHT: i32 = 30;
const PAD: i32 = 10;
const IMAGE: &str = "Image";
const SIZE: &str = "Size";
const LIST: &str = "List";
const NEXT: i32 = 101;
const PREV: i32 = 102;
const OPEN: i32 = 103;
const REM: i32 = 104;

#[derive(Clone)]
struct Model {
    cash: HashMap<String, SharedImage>,
    curr: String,
    size: i32,
}
impl Model {
    fn choice(&mut self, file: String) {
        if self.cash.contains_key(&file) {
            self.curr = file.clone();
        } else if let Ok(image) = SharedImage::load(file.clone()) {
            self.cash.insert(file.clone(), image.clone());
            self.curr = file.clone();
        }
    }
}

fn main() -> Result<(), FltkError> {
    app::GlobalState::<Model>::new(Model {
        cash: HashMap::new(),
        curr: String::new(),
        size: 100,
    });
    let app = app::App::default();
    let mut windows = crate::window();
    let mut page = Flex::default_fill().column(); //Page
    {
        let mut header = Flex::default(); //HEADER
        crate::menu(&mut header);
        crate::button("Open", "@#fileopen", crate::OPEN, &mut header);
        crate::button("Prev", "@#|<", crate::PREV, &mut header);
        crate::slider(crate::SIZE).set_callback(crate::size);
        crate::button("Next", "@#>|", crate::NEXT, &mut header);
        crate::button("Remove", "@#1+", crate::REM, &mut header);
        header.end();
        header.set_pad(0);
        page.fixed(&header, HEIGHT);
    }
    crate::frame(crate::IMAGE).set_frame(FrameType::DownBox);
    crate::browser(crate::LIST, &mut page);
    page.end();
    page.set_pad(PAD);
    page.set_margin(PAD);
    page.set_frame(FrameType::FlatBox);
    windows.end();
    windows.show();
    app.run()
}

fn button(tooltip: &str, label: &str, msg: i32, flex: &mut Flex) {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    element.set_callback(move |_| {
        app::handle_main(Event::from_i32(msg)).unwrap();
    });
    flex.fixed(&element, crate::HEIGHT);
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
    app::redraw();
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default_fill().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.draw(move |frame| {
        let model = app::GlobalState::<Model>::get().with(move |model| model.clone());
        if model.curr.is_empty() {
            frame.set_image(None::<SharedImage>);
        } else {
            let mut image = model.cash[&model.curr].clone();
            image.scale(
                frame.width() * model.size / 100,
                frame.height() * model.size / 100,
                true,
                true,
            );
            frame.set_image(Some(image));
        }
    });
    element
}

fn browser(tooltip: &str, flex: &mut Flex) {
    let mut element = Browser::default()
        .with_type(BrowserType::Hold)
        .with_id(tooltip);
    element.set_tooltip(tooltip);
    element.handle(crate::browser_handle);
    element.set_callback(move |browser| {
        if let Some(file) = browser.selected_text() {
            app::GlobalState::<Model>::get().with(move |model| model.choice(file.clone()));
            app::redraw();
        };
    });
    flex.fixed(&element, crate::HEIGHT * 2);
}

fn browser_handle(browser: &mut Browser, event: Event) -> bool {
    match event.bits() {
        crate::NEXT => {
            match browser.value() < browser.size() {
                true => browser.select(browser.value() + 1),
                false => browser.select(1),
            };
            browser.do_callback();
            true
        }
        crate::PREV => {
            match browser.value() > 1 {
                true => browser.select(browser.value() - 1),
                false => browser.select(browser.size()),
            };
            browser.do_callback();
            true
        }
        crate::REM => {
            match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                Some(0) => browser.remove(browser.value()),
                Some(2) => {
                    if fs::remove_file(browser.selected_text().unwrap()).is_ok() {
                        browser.remove(browser.value());
                    }
                }
                _ => {}
            };
            app::handle_main(Event::from_i32(crate::NEXT)).unwrap();
            true
        }
        crate::OPEN => {
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
                        browser.add(&file);
                    };
                }
                browser.sort();
                browser.select(1);
                browser.do_callback();
            };
            true
        }
        _ => false,
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
                fs::remove_file(&file).unwrap();
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
        .with_label(NAME)
        .center_screen();
    element.size_range(
        DEFAULT[0] as i32 * U8 + DEFAULT[1] as i32,
        DEFAULT[2] as i32 * U8 + DEFAULT[3] as i32,
        0,
        0,
    );
    element.set_xclass(NAME);
    element.make_resizable(true);
    element.set_callback(move |window| {
        if app::event() == Event::Close {
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
        }
    });
    ColorTheme::new(color_themes::DARK_THEME).apply();
    element
}
