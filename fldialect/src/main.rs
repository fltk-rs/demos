#![forbid(unsafe_code)]

mod commands;

use {
    fltk::{
        app,
        app::WidgetId,
        button::{Button, ButtonType},
        dialog::{alert_default, FileChooser, FileChooserType, HelpDialog},
        enums::{Color, Cursor, Event, Font, Shortcut},
        frame::Frame,
        group::{Flex, FlexType},
        image::SvgImage,
        menu::{Choice, MenuButton, MenuFlag},
        prelude::{
            ButtonExt, DisplayExt, GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt,
        },
        text::{TextBuffer, TextEditor, WrapMode},
        valuator::{Counter, CounterType, Dial},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    std::{env, fs, path::Path, thread},
};

const NAME: &str = "FlDialect";
const WIDTH: i32 = 105;
const SPACE: i32 = 10;
const HEIGHT: i32 = SPACE * 3;

fn main() {
    if commands::once() {
        app();
    }
}

fn app() {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let (mut window, params) = crate::window();

    let mut page = Flex::default_fill().column().with_id("Page");

    let mut header = Flex::default_fill().with_id("Header"); // HEADER
    crate::menu("Menu", params[4], &mut header);
    Frame::default();
    let lang = commands::list();
    crate::choice("From", &lang, params[5], &mut header).set_callback(move |_| crate::rename());
    crate::button("Switch", "@#refresh", &mut header).set_callback(crate::switch);
    crate::choice("To", &lang, params[6], &mut header).set_callback(move |_| crate::rename());
    Frame::default();
    crate::button("Speak", "@#<", &mut header).with_type(ButtonType::Toggle);
    header.end();

    let mut hero = Flex::default().column().with_id("Hero"); //HERO
    crate::text("Source");
    crate::handle("Handle");
    crate::text("Target");
    hero.end();

    let mut footer = Flex::default_fill().with_id("Footer"); //FOOTER
    crate::button("Open...", "@#fileopen", &mut footer).set_callback(crate::open);
    Frame::default();
    crate::choice("Fonts", "Courier|Helvetica|Times", params[7], &mut footer)
        .set_callback(crate::font);
    crate::button("Translate", "@#circle", &mut footer).set_callback(crate::trans);
    crate::counter("Size", params[8] as f64, &mut footer).with_type(CounterType::Simple);
    crate::dial("Spinner", &mut footer);
    Frame::default();
    crate::button("Save as...", "@#filesaveas", &mut footer).set_callback(crate::save);

    footer.end();

    page.end();

    window.end();
    window.show();
    {
        header.set_pad(SPACE);
        hero.set_pad(0);
        footer.set_pad(SPACE);
        page.fixed(&header, HEIGHT);
        page.fixed(&footer, HEIGHT);
        page.set_margin(SPACE);
        page.set_pad(SPACE);
        crate::rename();
        app::widget_from_id::<Choice>("Fonts")
            .unwrap()
            .do_callback();
        app::widget_from_id::<Counter>("Size")
            .unwrap()
            .do_callback();
        let menu = app::widget_from_id::<MenuButton>("Menu").unwrap();
        menu.at(1).unwrap().do_callback(&menu);
    }
    app.run().unwrap();
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label).with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT / 2);
    flex.fixed(&element, HEIGHT);
    element
}

fn handle(tooltip: &str) -> Frame {
    let mut element = Frame::default().with_id(tooltip);
    element.handle(move |frame, event| {
        let mut hero = app::widget_from_id::<Flex>("Hero").unwrap();
        match event {
            Event::Push => true,
            Event::Drag => {
                let source = app::widget_from_id::<TextEditor>("Source").unwrap();
                match hero.get_type() {
                    FlexType::Column => {
                        if (hero.y()..=hero.height() + hero.y() - frame.height())
                            .contains(&app::event_y())
                        {
                            hero.fixed(&source, app::event_y() - hero.y());
                        }
                    }
                    FlexType::Row => {
                        if (hero.x()..=hero.width() + hero.x() - frame.width())
                            .contains(&app::event_x())
                        {
                            hero.fixed(&source, app::event_x() - hero.x());
                        }
                    }
                }
                app::redraw();
                true
            }
            Event::Enter => {
                frame.window().unwrap().set_cursor(match hero.get_type() {
                    FlexType::Column => Cursor::NS,
                    FlexType::Row => Cursor::WE,
                });
                true
            }
            Event::Leave => {
                frame.window().unwrap().set_cursor(Cursor::Arrow);
                true
            }
            _ => false,
        }
    });
    element
}

fn counter(tooltip: &str, value: f64, flex: &mut Flex) -> Counter {
    let mut element = Counter::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_range(14_f64, 22_f64);
    element.set_precision(0);
    element.set_value(value);
    element.set_callback(move |size| {
        for label in ["Source", "Target"] {
            if let Some(mut text) = app::widget_from_id::<TextEditor>(label) {
                text.set_text_size(size.value() as i32);
                text.redraw();
            }
        }
    });
    flex.fixed(&element, WIDTH - HEIGHT - SPACE);
    element.do_callback();
    element
}

fn dial(tooltip: &str, flex: &mut Flex) -> Dial {
    let mut element = Dial::default().with_id(tooltip);
    element.set_maximum(99_f64);
    element.set_precision(0);
    element.set_callback(move |dial| dial.set_value(dial.value() + 1_f64));
    element.deactivate();
    flex.fixed(&element, HEIGHT);
    element
}

fn choice(tooltip: &str, choice: &str, value: u8, flex: &mut Flex) -> Choice {
    let mut element = Choice::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value as i32);
    flex.fixed(&element, WIDTH);
    element
}

fn text(tooltip: &str) -> TextEditor {
    let mut element = TextEditor::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element
}

fn menu(tooltip: &str, theme: u8, flex: &mut Flex) -> MenuButton {
    let mut element = MenuButton::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    let idx = element.add(
        "&View/&Night mode\t",
        Shortcut::Ctrl | 'n',
        MenuFlag::Toggle,
        crate::theme,
    );
    if theme != 0 {
        element.at(idx).unwrap().set();
    };
    let idx: i32 = element.add(
        "&View/&Footer\t",
        Shortcut::None,
        MenuFlag::Toggle,
        crate::hide,
    );
    element.at(idx).unwrap().set();
    element.add(
        "@#circle  T&ranslate",
        Shortcut::Ctrl | 'r',
        MenuFlag::Normal,
        move |_| {
            app::widget_from_id::<Button>("Translate")
                .unwrap()
                .do_callback()
        },
    );
    element.add(
        "@#search  &Info",
        Shortcut::Ctrl | 'i',
        MenuFlag::Normal,
        crate::info,
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
    flex.fixed(&element, HEIGHT);
    element
}

fn info(_: &mut MenuButton) {
    const INFO: &str = r#"<p>
<a href=\"https://gitlab.com/kbit/kbit.gitlab.io/-/tree/master/app/front/flcalculator\">FlCalculator</a>
 is similar to
 <a href=\"https://apps.gnome.org/Calculator\">Calculator</a>
 written using
 <a href=\"https://fltk-rs.github.io/fltk-rs\">FLTK-RS</a>
</p>"#;
    let mut dialog = HelpDialog::default();
    dialog.set_value(INFO);
    dialog.set_text_size(16);
    dialog.show();
    while dialog.shown() {
        app::wait();
    }
}

fn open(_: &mut Button) {
    let mut dialog = FileChooser::new(
        std::env::var("HOME").unwrap(),
        "*.{txt,md}",
        FileChooserType::Single,
        "Open ...",
    );
    dialog.show();
    while dialog.shown() {
        app::wait();
    }
    if dialog.count() > 0 {
        if let Some(file) = dialog.value(1) {
            app::widget_from_id::<TextEditor>("Source")
                .unwrap()
                .buffer()
                .unwrap()
                .load_file(std::path::Path::new(&file))
                .ok()
                .unwrap();
        };
    };
}
fn save(_: &mut Button) {
    if let Some(mut source) = app::widget_from_id::<TextEditor>("Source")
        .unwrap()
        .buffer()
    {
        if !source.text().is_empty() {
            let mut dialog = FileChooser::new(
                std::env::var("HOME").unwrap(),
                "*.{txt,md}",
                FileChooserType::Create,
                "Save ...",
            );
            dialog.show();
            while dialog.shown() {
                app::wait();
            }
            if dialog.count() > 0 {
                if let Some(file) = dialog.value(1) {
                    source.save_file(std::path::Path::new(&file)).ok().unwrap();
                };
            };
        } else {
            alert_default("Target is empty.");
        };
    };
}

fn hide(_: &mut MenuButton) {
    let mut page = app::widget_from_id::<Flex>("Page").unwrap();
    let mut footer = app::widget_from_id::<Flex>("Footer").unwrap();
    if footer.visible() {
        page.fixed(&footer, 0);
        footer.hide();
    } else {
        page.fixed(&footer, HEIGHT);
        footer.show();
    };
    page.redraw();
}

fn rename() {
    app::first_window().unwrap().set_label(&format!(
        "Translate from {} to {} - {}",
        app::widget_from_id::<Choice>("From")
            .unwrap()
            .choice()
            .unwrap(),
        app::widget_from_id::<Choice>("To")
            .unwrap()
            .choice()
            .unwrap(),
        NAME
    ));
}

pub fn switch(_: &mut Button) {
    let mut from = app::widget_from_id::<Choice>("From").unwrap();
    let mut to = app::widget_from_id::<Choice>("To").unwrap();
    if from.value() != to.value() {
        let temp = from.value();
        from.set_value(to.value());
        to.set_value(temp);
        crate::rename();
    }
}

pub fn theme(menu: &mut MenuButton) {
    const COLORS: [[Color; 2]; 2] = [
        [Color::from_hex(0xfdf6e3), Color::from_hex(0x586e75)],
        [Color::from_hex(0x002b36), Color::from_hex(0x93a1a1)],
    ];
    let item = menu.at(1).unwrap();
    app::set_scheme(if item.value() {
        ColorTheme::new(color_themes::DARK_THEME).apply();
        app::Scheme::Plastic
    } else {
        ColorTheme::new(color_themes::TAN_THEME).apply();
        app::Scheme::Base
    });
    for label in ["Source", "Target"] {
        let mut text = app::widget_from_id::<TextEditor>(label).unwrap();
        text.set_color(COLORS[item.value() as usize][0]);
        text.set_text_color(COLORS[item.value() as usize][1]);
    }
}

pub fn trans(button: &mut Button) {
    let from = app::widget_from_id::<Choice>("From")
        .unwrap()
        .choice()
        .unwrap();
    let to = app::widget_from_id::<Choice>("To")
        .unwrap()
        .choice()
        .unwrap();
    let source = app::widget_from_id::<TextEditor>("Source")
        .unwrap()
        .buffer()
        .unwrap()
        .text();
    if from != to && !source.is_empty() {
        button.deactivate();
        let voice = app::widget_from_id::<Button>("Speak").unwrap().value();
        let handler = thread::spawn(move || -> String { commands::run(voice, from, to, source) });
        let mut dial = app::widget_from_id::<Dial>("Spinner").unwrap();
        while !handler.is_finished() {
            app::wait();
            app::sleep(0.02);
            dial.do_callback();
        }
        if let Ok(msg) = handler.join() {
            app::widget_from_id::<TextEditor>("Target")
                .unwrap()
                .buffer()
                .unwrap()
                .set_text(&msg);
            println!("{}", msg);
            button.activate();
        };
    };
}

fn window() -> (Window, Vec<u8>) {
    const SVG: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:cc="http://creativecommons.org/ns#" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:svg="http://www.w3.org/2000/svg" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="254" height="93" clip-path="url(#clipPath18)" id="svg2">
  <metadata id="metadata4">
    <rdf:RDF>
      <cc:Work rdf:about="">
        <dc:format>image/svg+xml</dc:format>
        <dc:type rdf:resource="http://purl.org/dc/dcmitype/StillImage"/>
        <dc:title/>
      </cc:Work>
    </rdf:RDF>
  </metadata>
  <defs id="defs6">
    <linearGradient id="linearGradient8" x1="159" y1="91" x2="23" y2="13" gradientUnits="userSpaceOnUse" spreadMethod="reflect">
      <stop id="stop10" style="stop-color:#000000;stop-opacity:0" offset="0"/>
      <stop id="stop12" style="stop-color:#000000;stop-opacity:0.192" offset="0.33"/>
      <stop id="stop14" style="stop-color:#000000;stop-opacity:0.5" offset="0.72"/>
      <stop id="stop16" style="stop-color:#000000;stop-opacity:1" offset="1"/>
    </linearGradient>
  </defs>
  <rect width="254" height="93" id="rect22" style="fill:#d6ddf2;stroke:#7c808d;stroke-width:4"/>
  <path d="m 271,-31.5 -71,71 0,-36.5 -90,0 0,17 28,0 0,53 -46,0 0,-70 -89,0 0,87 17,0 0,-34.5 36,0 0,-17 -36,0 0,-18.5 55,0 0,70 80,0 0,-70 28,0 0,70 17,0 0,-36 71,71 z M 254,84 216.75,46.75 254,9.5" id="path24" style="fill:#7c808d;stroke:#7c808d;stroke-width:6;stroke-linejoin:round"/>
  <rect width="254" height="93" id="rect26" style="fill:url(#linearGradient8)"/>
  <path d="m 72,11.5 -60.5,0 0,78.5 m 0,-43 44.5,0 m 27.5,-44 0,78.5 51.5,0 m -25,-70 70,0 m -33.5,0 0,78.5 m 45,-87 0,87 m 71,-101 -57.75,57.75 57.75,57.75" id="path28" style="fill:none;stroke:#ffffff;stroke-width:17"/>
</svg>"#;
    const DEFAULT: [u8; 9] = [
        1,   // [0] window_width * U8 +
        105, // [1] window_width_fract
        2,   // [2] window_height * U8 +
        130, // [3] window_height_fract
        0,   // [4] theme
        119, // [5] header_from
        35,  // [6] header_to
        1,   // [7] footer_font
        14,  // [8] footer_size
    ];
    const U8: i32 = 255;
    const CONFIG: &str = "/.config/";
    let file = env::var("HOME").unwrap() + CONFIG + NAME;
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
    element.set_icon(Some(SvgImage::from_data(SVG).unwrap()));
    element.handle(move |window, event| {
        if event == Event::Resize {
            let mut flex = app::widget_from_id::<Flex>("Hero").unwrap();
            let from = app::widget_from_id::<TextEditor>("Source").unwrap();
            if window.width() < window.height() {
                flex.set_type(FlexType::Column);
                flex.fixed(&from, 0);
            } else {
                flex.set_type(FlexType::Row);
                flex.fixed(&from, 0);
            };
            flex.fixed(&app::widget_from_id::<Frame>("Handle").unwrap(), SPACE);
            true
        } else {
            false
        }
    });
    element.set_callback(move |window| {
        if app::event() == Event::Close {
            fs::write(
                &file,
                [
                    (window.width() / U8) as u8,
                    (window.width() % U8) as u8,
                    (window.height() / U8) as u8,
                    (window.height() % U8) as u8,
                    app::widget_from_id::<MenuButton>("Menu")
                        .unwrap()
                        .at(1)
                        .unwrap()
                        .value() as u8,
                    app::widget_from_id::<Choice>("From").unwrap().value() as u8,
                    app::widget_from_id::<Choice>("To").unwrap().value() as u8,
                    app::widget_from_id::<Choice>("Fonts").unwrap().value() as u8,
                    app::widget_from_id::<Counter>("Size").unwrap().value() as u8,
                ],
            )
            .unwrap();
            app::quit();
        }
    });
    (element, params)
}
fn font(font: &mut Choice) {
    for label in ["Source", "Target"] {
        if let Some(mut text) = app::widget_from_id::<TextEditor>(label) {
            text.set_text_font(Font::by_name(&font.choice().unwrap()));
            text.redraw();
        }
    }
}
