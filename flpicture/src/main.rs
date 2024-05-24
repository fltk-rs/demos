#![forbid(unsafe_code)]

use {
    fltk::{
        app,
        app::WidgetId,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{CallbackTrigger, Color, Event, FrameType, Shortcut},
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

#[derive(Debug, Clone)]
struct Model {
    cash: HashMap<String, SharedImage>,
    temp: Vec<String>,
    curr: usize,
    size: i32,
}
impl Model {
    fn choice(&mut self, curr: usize) {
        if self.cash.contains_key(&self.temp[curr]) {
            self.curr = curr;
        } else if let Ok(image) = SharedImage::load(&self.temp[curr]) {
            self.cash.insert(self.temp[curr].clone(), image.clone());
            self.curr = curr;
        }
    }
    fn remove(&mut self) {
        self.temp.remove(self.curr);
        self.inc();
    }
    fn inc(&mut self) {
        if !self.temp.is_empty() {
            match self.curr < self.temp.len() - 1 {
                true => self.curr += 1,
                false => self.curr = 0,
            };
            self.choice(self.curr);
        }
    }
    fn dec(&mut self) {
        if !self.temp.is_empty() {
            match self.curr > 0 {
                true => self.curr -= 1,
                false => self.curr = self.temp.len() - 1,
            };
            self.choice(self.curr);
        }
    }
}

fn main() -> Result<(), FltkError> {
    app::GlobalState::<Model>::new(Model {
        cash: HashMap::new(),
        temp: Vec::new(),
        size: 100,
        curr: 0,
    });
    let app = app::App::default();
    let mut window = crate::window();
    let mut page = Flex::default_fill().column(); //Page
    {
        let mut header = Flex::default(); //HEADER
        crate::menu(&mut header);
        crate::button("Open", "@#fileopen", &mut header).set_callback(crate::open);
        crate::button("Prev", "@#|<", &mut header).set_callback(crate::shift);
        crate::slider(crate::SIZE).set_callback(crate::size);
        crate::button("Next", "@#>|", &mut header).set_callback(crate::shift);
        crate::button("Remove", "@#1+", &mut header).set_callback(crate::remove);
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
    window.end();
    window.show();
    app.run()
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
    app::redraw();
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default_fill();
    element.set_tooltip(tooltip);
    element.draw(move |frame| {
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
        }
    });
    element
}

fn browser(tooltip: &str, flex: &mut Flex) {
    let mut element = Browser::default()
        .with_type(BrowserType::Hold)
        .with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_trigger(CallbackTrigger::Changed);
    element.set_callback(move |browser| {
        if browser.value() > 0 {
            let curr: usize = browser.value() as usize - 1;
            app::GlobalState::<Model>::get().with(move |model| model.choice(curr));
            app::redraw();
        }
    });
    element.draw(move |browser| {
        let (curr, temp) =
            app::GlobalState::<Model>::get().with(move |model| (model.curr, model.temp.clone()));
        browser.clear();
        for item in temp {
            browser.add(&item);
        }
        browser.select(curr as i32 + 1);
    });
    flex.fixed(&element, crate::HEIGHT * 2);
}

fn remove(button: &mut Button) {
    button.deactivate();
    let (curr, temp) =
        app::GlobalState::<Model>::get().with(move |model| (model.curr, model.temp.clone()));
    if !temp.is_empty() {
        match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
            Some(0) => app::GlobalState::<Model>::get().with(move |model| model.remove()),
            Some(2) => {
                if fs::remove_file(temp[curr].clone()).is_ok() {
                    app::GlobalState::<Model>::get().with(move |model| model.remove());
                }
            }
            _ => {}
        };
    };
    button.activate();
    app::redraw();
}

fn shift(button: &mut Button) {
    button.deactivate();
    let label = button.label();
    app::GlobalState::<Model>::get().with(move |model| match label == "@#<" {
        true => model.dec(),
        false => model.inc(),
    });
    button.activate();
    app::redraw();
}

fn open(button: &mut Button) {
    button.deactivate();
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
        app::GlobalState::<Model>::get().with(move |model| {
            for item in 1..=dialog.count() {
                if let Some(file) = dialog.value(item) {
                    model.temp.push(file);
                };
                model.choice(0);
            }
        });
    };
    button.activate();
    app::redraw();
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
    element.draw(move |window| {
        let (temp, curr) =
            app::GlobalState::<Model>::get().with(move |model| (model.temp.clone(), model.curr));
        match temp.is_empty() {
            true => window.set_label(NAME),
            false => window.set_label(&format!("{} - {NAME}", temp[curr].clone())),
        };
    });
    ColorTheme::new(color_themes::DARK_THEME).apply();
    element
}
