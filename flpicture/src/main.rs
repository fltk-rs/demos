#![forbid(unsafe_code)]

use {
    fltk::{
        app,
        app::WidgetId,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{CallbackTrigger, Color, Event, Shortcut},
        frame::Frame,
        group::Flex,
        image::SharedImage,
        menu::{MenuButton, MenuFlag},
        prelude::{
            BrowserExt, GroupExt, ImageExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt,
        },
        valuator::{Slider, SliderType},
        window::Window,
    },
    std::{env, fs, path::Path},
};

const HEIGHT: i32 = 30;
const PAD: i32 = 10;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut windows = crate::window();
    let mut page = Flex::default_fill().column().with_id("Page");
    let mut header = Flex::default().with_id("Header");
    crate::menu("Main menu", &mut header);
    crate::button("Open", "@#fileopen", &mut header).set_callback(crate::open);
    crate::button("Prev", "@#|<", &mut header).set_callback(crate::prev);
    crate::slider("Size").with_type(SliderType::Horizontal);
    crate::button("Next", "@#>|", &mut header).set_callback(crate::next);
    crate::button("Remove", "@#1+", &mut header).set_callback(crate::rem);
    header.end();
    crate::frame("Image");
    crate::browser("List", &mut page).with_type(BrowserType::Hold);
    page.end();
    windows.end();
    windows.show();
    {
        header.set_pad(0);
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.fixed(&header, HEIGHT);
    }
    app.run().unwrap();
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_id(tooltip).with_label(label);
    element.set_tooltip(tooltip);
    flex.fixed(&element, crate::HEIGHT);
    element
}

fn slider(tooltip: &str) -> Slider {
    let mut element = Slider::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_maximum(100f64);
    element.set_precision(0);
    element.set_value(element.maximum());
    element.set_callback(move |size| {
        let mut frame = app::widget_from_id::<Frame>("Image").unwrap();
        let browser = app::widget_from_id::<Browser>("List").unwrap();
        if let Ok(mut image) = SharedImage::load(browser.selected_text().unwrap()) {
            image.scale(
                (frame.width() as f64 * size.value()) as i32 / 100,
                (frame.height() as f64 * size.value()) as i32 / 100,
                true,
                true,
            );
            frame.set_image(Some(image));
            app::redraw();
        };
    });
    element
}

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default_fill().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_image(None::<SharedImage>);
    element
}

fn browser(tooltip: &str, flex: &mut Flex) -> Browser {
    let mut element = Browser::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_trigger(CallbackTrigger::Changed);
    element.set_callback(move |browser| {
        let mut frame = app::widget_from_id::<Frame>("Image").unwrap();
        let size = app::widget_from_id::<Slider>("Size").unwrap();
        if browser.value() < 1 {
            frame.set_image(None::<SharedImage>);
        } else if let Ok(mut image) = SharedImage::load(browser.selected_text().unwrap()) {
            image.scale(
                (frame.width() as f64 * size.value()) as i32 / 100,
                (frame.height() as f64 * size.value()) as i32 / 100,
                true,
                true,
            );
            frame.set_image(Some(image));
        };
        frame.redraw();
        browser.redraw();
    });
    flex.fixed(&element, crate::HEIGHT * 2);
    element
}

fn menu(tooltip: &str, flex: &mut Flex) -> MenuButton {
    let mut element = MenuButton::default().with_id(tooltip).with_label("@#menu");
    element.set_tooltip(tooltip);
    element.add(
        "&File/@#fileopen  &Open",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| app::widget_from_id::<Button>("Open").unwrap().do_callback(),
    );
    let ord: i32 = element.add(
        "&File/@#1+  &Remove",
        Shortcut::Ctrl | 'd',
        MenuFlag::Normal,
        move |_| {
            app::widget_from_id::<Button>("Remove")
                .unwrap()
                .do_callback()
        },
    );
    element.at(ord).unwrap().set_label_color(Color::Red);
    element.add(
        "&Image/@#>|  &Next",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| app::widget_from_id::<Button>("Next").unwrap().do_callback(),
    );
    element.add(
        "&Image/@#|<  &Prev",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| app::widget_from_id::<Button>("Prev").unwrap().do_callback(),
    );
    flex.fixed(&element, 50);
    element
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
    element
}

fn open(_: &mut Button) {
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
        let mut browser = app::widget_from_id::<Browser>("List").unwrap();
        for item in 1..=dialog.count() {
            if let Some(file) = dialog.value(item) {
                browser.add(&file);
            };
        }
        browser.sort();
        browser.select(1);
        browser.do_callback();
    };
}

fn next(_: &mut Button) {
    let mut browser = app::widget_from_id::<Browser>("List").unwrap();
    match browser.value() < browser.size() {
        true => browser.select(browser.value() + 1),
        false => browser.select(1),
    };
    browser.do_callback();
}

fn prev(_: &mut Button) {
    let mut browser = app::widget_from_id::<Browser>("List").unwrap();
    match browser.value() > 1 {
        true => browser.select(browser.value() - 1),
        false => browser.select(browser.size()),
    };
    browser.do_callback();
}

fn rem(_: &mut Button) {
    let mut browser = app::widget_from_id::<Browser>("List").unwrap();
    let mut next = app::widget_from_id::<Button>("Next").unwrap();
    match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
        Some(0) => browser.remove(browser.value()),
        Some(2) => {
            if fs::remove_file(browser.selected_text().unwrap()).is_ok() {
                browser.remove(browser.value());
            }
        }
        _ => {}
    };
    next.do_callback();
}
