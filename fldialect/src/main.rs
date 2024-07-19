#![forbid(unsafe_code)]

use {
    fltk::{
        app,
        app::WidgetId,
        button::{Button, ButtonType},
        dialog::{alert_default, FileChooser, FileChooserType, HelpDialog},
        enums::{CallbackTrigger, Color, Cursor, Event, Font, FrameType, Shortcut},
        frame::Frame,
        group::{Flex, FlexType},
        image::SvgImage,
        menu::{Choice, MenuButton, MenuFlag},
        misc::InputChoice,
        prelude::{
            ButtonExt, DisplayExt, GroupExt, InputExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt,
            WindowExt,
        },
        text::{TextBuffer, TextEditor, WrapMode},
        valuator::{Counter, CounterType, Dial},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    std::{env, fs, path::Path, process::Command, thread},
};

const NAME: &str = "FlDialect";
const DIAL: &str = "Spinner";
const FROM: &str = "From";
const TO: &str = "To";
const SOURCE: &str = "Source";
const TARGET: &str = "Target";
const PAGE: &str = "Page";
const HERO: &str = "Hero";
const FOOTER: &str = "Footer";
const FONTS: &str = "Fonts";
const SIZE: &str = "Size";
const TRANSLATE: &str = "Translate";
const SPEAK: &str = "Speak";
const HANDLE: &str = "Handle";
const WIDTH: i32 = 125;
const SPACE: i32 = 10;
const HEIGHT: i32 = SPACE * 3;

fn main() {
    if crate::once() {
        app();
    }
}

fn app() {
    app::GlobalState::new(crate::list());
    let app = app::App::default();
    let (mut window, params) = crate::window();
    {
        let mut page = Flex::default_fill().column().with_id(crate::PAGE);
        {
            let mut header = Flex::default_fill(); // HEADER
            header.fixed(&crate::menu(), HEIGHT);
            Frame::default();
            let mut bar = Flex::default();
            crate::input(crate::FROM);
            crate::button("Switch", "@#refresh", &mut bar).set_callback(crate::switch);
            crate::input(crate::TO);
            bar.end();
            bar.set_pad(0);
            header.fixed(&bar, WIDTH * 2 + HEIGHT);
            Frame::default();
            crate::button(crate::SPEAK, "@#<", &mut header).with_type(ButtonType::Toggle);
            header.end();
            header.set_pad(SPACE);
            page.fixed(&header, HEIGHT);
        }
        {
            let mut hero = Flex::default_fill().column().with_id(crate::HERO); //HERO
            crate::text(crate::SOURCE);
            hero.fixed(&crate::handle(crate::HANDLE), SPACE);
            crate::text(crate::TARGET);
            hero.end();
            hero.handle(crate::resize);
            hero.set_pad(0);
            hero.set_margin(0);
        }
        {
            let mut footer = Flex::default_fill().with_id(crate::FOOTER); //FOOTER
            crate::button("Open...", "@#fileopen", &mut footer).set_callback(crate::open);
            Frame::default();
            footer.fixed(&Frame::default(), HEIGHT);
            let mut bar = Flex::default();
            crate::choice("Fonts", &app::fonts().join("|"), params[4]).set_callback(crate::font);
            crate::button(crate::TRANSLATE, "@#circle", &mut bar).set_callback(crate::translate);
            crate::counter("Size", params[5] as f64);
            bar.end();
            bar.set_pad(0);
            footer.fixed(&bar, WIDTH * 2 + HEIGHT);
            footer.fixed(&crate::dial(), HEIGHT);
            Frame::default();
            crate::button("Save as...", "@#filesaveas", &mut footer).set_callback(crate::save);
            footer.end();
            footer.set_pad(0);
            page.fixed(&footer, HEIGHT);
        }
        page.end();
        page.set_margin(SPACE);
        page.set_pad(SPACE);
        page.set_frame(FrameType::FlatBox);
    }
    window.end();
    window.show();
    ColorTheme::new(color_themes::DARK_THEME).apply();
    crate::rename();
    app::widget_from_id::<Choice>(crate::FONTS)
        .unwrap()
        .do_callback();
    app::widget_from_id::<Counter>(crate::SIZE)
        .unwrap()
        .do_callback();
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
        let mut flex = app::widget_from_id::<Flex>(crate::HERO).unwrap();
        match event {
            Event::Push => true,
            Event::Drag => {
                let child = flex.child(0).unwrap();
                match flex.get_type() {
                    FlexType::Column => {
                        if (flex.y()..=flex.height() + flex.y() - frame.height())
                            .contains(&app::event_y())
                        {
                            flex.fixed(&child, app::event_y() - flex.y());
                        }
                    }
                    FlexType::Row => {
                        if (flex.x()..=flex.width() + flex.x() - frame.width())
                            .contains(&app::event_x())
                        {
                            flex.fixed(&child, app::event_x() - flex.x());
                        }
                    }
                }
                app::redraw();
                true
            }
            Event::Enter => {
                frame.window().unwrap().set_cursor(match flex.get_type() {
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

fn counter(tooltip: &str, value: f64) -> Counter {
    let mut element = Counter::default()
        .with_type(CounterType::Simple)
        .with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_range(14_f64, 22_f64);
    element.set_precision(0);
    element.set_value(value);
    element.set_callback(move |size| {
        for label in [crate::SOURCE, crate::TARGET] {
            if let Some(mut text) = app::widget_from_id::<TextEditor>(label) {
                text.set_text_size(size.value() as i32);
                text.redraw();
            }
        }
    });
    element.do_callback();
    element
}

fn dial() -> Dial {
    const DIAL: u8 = 120;
    let mut element = Dial::default().with_id(crate::DIAL);
    element.deactivate();
    element.set_maximum((DIAL / 4 * 3) as f64);
    element.set_precision(0);
    element.set_callback(move |dial| {
        dial.set_value(if dial.value() == (DIAL - 1) as f64 {
            dial.minimum()
        } else {
            dial.value() + 1f64
        })
    });
    element
}

fn choice(tooltip: &str, choice: &str, value: u8) -> Choice {
    let mut element = Choice::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value as i32);
    element
}

fn input(tooltip: &'static str) {
    let mut element = InputChoice::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    let mut choice = element.clone();
    element.input().set_trigger(CallbackTrigger::Changed);
    element.input().set_callback(move |input| {
        choice.clear();
        for lang in app::GlobalState::<Vec<String>>::get().with(|languages| languages.clone()) {
            if lang
                .to_lowercase()
                .starts_with(&input.value().to_lowercase())
            {
                choice.add(&lang);
            }
        }
    });
    element.input().do_callback();
    element.set_value_index(0);
    element.set_callback(move |_| crate::rename());
}

fn text(tooltip: &str) {
    let mut element = TextEditor::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default();
    element.set_tooltip("Menu");
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
            app::widget_from_id::<Button>(crate::TRANSLATE)
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
    element
}

fn info(_: &mut MenuButton) {
    const INFO: &str = r#"<p style="color:gray;">
<a href="https://github.com/fltk-rs/demos/tree/master/fldialect">FlDialect</a>
 is similar to
 <a href="https://apps.gnome.org/Dialect">Dialect</a>
 written using
 <a href="https://fltk-rs.github.io/fltk-rs">FLTK-RS</a>
</p>"#;
    let (r, g, b) = Color::from_hex(0x2aa198).to_rgb();
    app::set_color(Color::Blue, r, g, b);
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
            app::widget_from_id::<TextEditor>(crate::SOURCE)
                .unwrap()
                .buffer()
                .unwrap()
                .load_file(&file)
                .ok()
                .unwrap();
        };
    };
}
fn save(_: &mut Button) {
    if let Some(mut editor) = app::widget_from_id::<TextEditor>(crate::TARGET)
        .unwrap()
        .buffer()
    {
        if !editor.text().is_empty() {
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
                    editor.save_file(&file).ok().unwrap();
                };
            };
        } else {
            alert_default("Target is empty.");
        };
    };
}

fn hide(_: &mut MenuButton) {
    let mut page = app::widget_from_id::<Flex>(crate::PAGE).unwrap();
    let mut footer = app::widget_from_id::<Flex>(crate::FOOTER).unwrap();
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
        "Translate from {} to {} - {NAME}",
        app::widget_from_id::<InputChoice>(crate::FROM)
            .unwrap()
            .value()
            .unwrap(),
        app::widget_from_id::<InputChoice>(crate::TO)
            .unwrap()
            .value()
            .unwrap(),
    ));
}

fn resize(flex: &mut Flex, event: Event) -> bool {
    if event == Event::Resize {
        flex.set_type(match flex.width() < flex.height() {
            true => FlexType::Column,
            false => FlexType::Row,
        });
        flex.fixed(&flex.child(0).unwrap(), 0);
        flex.fixed(&flex.child(1).unwrap(), SPACE);
        true
    } else {
        false
    }
}

fn switch(_: &mut Button) {
    let mut from = app::widget_from_id::<InputChoice>(crate::FROM).unwrap();
    let mut to = app::widget_from_id::<InputChoice>(crate::TO).unwrap();
    if from.value() != to.value() {
        let temp = from.value().unwrap();
        from.set_value(&to.value().unwrap());
        to.set_value(&temp);
        crate::rename();
    }
}

fn translate(button: &mut Button) {
    let from = app::widget_from_id::<InputChoice>(crate::FROM)
        .unwrap()
        .value()
        .unwrap();
    let to = app::widget_from_id::<InputChoice>(crate::TO)
        .unwrap()
        .value()
        .unwrap();
    let source = app::widget_from_id::<TextEditor>(crate::SOURCE)
        .unwrap()
        .buffer()
        .unwrap()
        .text();
    if from != to && !source.is_empty() {
        button.deactivate();
        let voice = app::widget_from_id::<Button>(crate::SPEAK).unwrap().value();
        let handler = thread::spawn(move || -> String { crate::run(voice, from, to, source) });
        while !handler.is_finished() {
            app::wait();
            app::sleep(0.02);
            app::widget_from_id::<Dial>(crate::DIAL)
                .unwrap()
                .do_callback();
        }
        if let Ok(msg) = handler.join() {
            app::widget_from_id::<TextEditor>(TARGET)
                .unwrap()
                .buffer()
                .unwrap()
                .set_text(&msg);
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
    const DEFAULT: [u8; 6] = [
        1,   // [0] window_width * U8 +
        105, // [1] window_width_fract
        2,   // [2] window_height * U8 +
        130, // [3] window_height_fract
        1,   // [4] footer_font
        14,  // [5] footer_size
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
    element.set_callback(move |window| {
        if app::event() == Event::Close {
            fs::write(
                &file,
                [
                    (window.width() / U8) as u8,
                    (window.width() % U8) as u8,
                    (window.height() / U8) as u8,
                    (window.height() % U8) as u8,
                    app::widget_from_id::<Choice>(crate::FONTS).unwrap().value() as u8,
                    app::widget_from_id::<Counter>(crate::SIZE).unwrap().value() as u8,
                ],
            )
            .unwrap();
            app::quit();
        }
    });
    (element, params)
}

fn font(font: &mut Choice) {
    for label in [crate::SOURCE, crate::TARGET] {
        if let Some(mut text) = app::widget_from_id::<TextEditor>(label) {
            text.set_text_font(Font::by_name(&font.choice().unwrap()));
            text.redraw();
        }
    }
}

fn run(voice: bool, from: String, to: String, word: String) -> String {
    let run = Command::new("trans")
        .args([
            "-join-sentence",
            "-no-ansi",
            "-show-languages",
            "n",
            "-show-original",
            "n",
            "-show-original-dictionary",
            "n",
            "-show-original-dictionary",
            "n",
            "-show-prompt-message",
            "n",
            "-show-alternatives",
            "n",
            "-show-translation-phonetics",
            "n",
            "-indent",
            "2",
            "-from",
            &from,
            "-to",
            &to,
            match word.split_whitespace().count() {
                1 => "",
                _ => "-brief",
            },
            if voice { "-speak" } else { "" },
            &word.trim().replace("\n\n", "\n"),
        ])
        .output()
        .expect("failed to execute bash");
    String::from_utf8_lossy(match run.status.success() {
        true => &run.stdout,
        false => &run.stderr,
    })
    .to_string()
}

fn list() -> Vec<String> {
    if cfg!(target_family = "unix") {
        let run = Command::new("trans")
            .arg("-list-languages-english")
            .output()
            .expect("failed to execute bash");
        match run.status.success() {
            true => String::from_utf8_lossy(&run.stdout)
                .lines()
                .map(str::to_string)
                .collect::<Vec<String>>(),
            false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        }
    } else {
        Vec::from(["no way".to_string()])
    }
}

fn once() -> bool {
    if cfg!(target_os = "linux") {
        let run = Command::new("lsof")
            .args(["-t", env::current_exe().unwrap().to_str().unwrap()])
            .output()
            .expect("failed to execute bash");
        match run.status.success() {
            true => {
                String::from_utf8_lossy(&run.stdout)
                    .split_whitespace()
                    .count()
                    == 1
            }
            false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        }
    } else {
        true
    }
}
