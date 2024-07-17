#![forbid(unsafe_code)]

mod model;

use {
    fltk::{
        app,
        button::Button,
        dialog::HelpDialog,
        enums::{Align, CallbackTrigger, Color, Cursor, Event, Font, FrameType, Key, Shortcut},
        frame::Frame,
        group::Flex,
        image::SvgImage,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        window::Window,
    },
    model::Model,
};

const HEARTBEAT: Event = Event::from_i32(404);
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const EQUAL: &str = "=";
const COLORS: [[Color; 6]; 2] = [
    [
        Color::from_hex(0xfdf6e3),
        Color::from_hex(0x586e75),
        Color::from_hex(0xb58900),
        Color::from_hex(0xeee8d5),
        Color::from_hex(0xcb4b16),
        Color::from_hex(0xdc322f),
    ],
    [
        Color::from_hex(0x002b36),
        Color::from_hex(0x93a1a1),
        Color::from_hex(0x268bd2),
        Color::from_hex(0x073642),
        Color::from_hex(0x6c71c4),
        Color::from_hex(0xd33682),
    ],
];

fn main() -> Result<(), FltkError> {
    let app = app::App::default().with_scheme(app::AppScheme::Base);
    let mut window = crate::window();
    crate::view();
    window.end();
    window.show();
    app::handle_main(HEARTBEAT).unwrap();
    app::set_font(Font::Courier);
    app.run()
}

fn view() {
    let mut page = Flex::default_fill().column();
    crate::display("Output");
    let mut row = Flex::default();
    crate::output("Operation").handle(move |frame, event| {
        if event == HEARTBEAT {
            let value = app::GlobalState::<Model>::get().with(move |model| model.operation.clone());
            frame.set_label(&value.to_string());
        };
        false
    });
    let mut col = Flex::default().column();
    crate::output("Previous").handle(move |frame, event| {
        if event == HEARTBEAT {
            let value = app::GlobalState::<Model>::get().with(move |model| model.prev);
            frame.set_label(&value.to_string());
        };
        false
    });
    crate::output("Current").handle(move |frame, event| {
        if event == HEARTBEAT {
            let value = app::GlobalState::<Model>::get().with(move |model| model.current.clone());
            frame.set_label(&value);
        };
        false
    });
    col.end();
    col.set_pad(0);
    col.handle(crate::theme);
    row.end();
    row.set_pad(0);
    row.set_margin(0);
    row.handle(crate::theme);
    row.fixed(&row.child(0).unwrap(), 30);
    let mut buttons = Flex::default_fill().column();
    for line in [
        ["CE", "C", "%", "/"],
        ["7", "8", "9", "x"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["0", ".", "@<-", crate::EQUAL],
    ] {
        let mut row = Flex::default();
        for label in line {
            crate::button(label).handle(crate::click);
        }
        row.end();
        row.set_pad(PAD);
        row.set_margin(0);
    }
    buttons.handle(crate::popup);
    buttons.end();
    buttons.set_pad(PAD);
    buttons.set_margin(0);
    page.fixed(&buttons, 425);
    page.end();
    page.set_margin(PAD);
    page.set_pad(PAD);
    page.set_frame(FrameType::FlatBox);
    page.fixed(&row, 60);
    page.handle(move |flex, event| {
        if event == HEARTBEAT {
            let theme = app::GlobalState::<Model>::get().with(move |model| model.theme);
            flex.set_color(crate::COLORS[theme as usize][0]);
        };
        false
    });
}

fn button(title: &'static str) -> Button {
    let mut element = Button::default().with_label(title);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::OFlatFrame);
    match title {
        "@<-" => element.set_shortcut(Shortcut::None | Key::BackSpace),
        "CE" => element.set_shortcut(Shortcut::None | Key::Delete),
        crate::EQUAL => element.set_shortcut(Shortcut::None | Key::Enter),
        "x" => element.set_shortcut(Shortcut::None | '*'),
        _ => element.set_shortcut(Shortcut::None | title.chars().next().unwrap()),
    }
    element
}

fn display(tooltip: &str) {
    let mut element = TextDisplay::default();
    element.set_text_size(HEIGHT - 5);
    element.set_tooltip(tooltip);
    element.set_scrollbar_size(3);
    element.set_frame(FrameType::FlatBox);
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_buffer(TextBuffer::default());
    element.set_trigger(CallbackTrigger::Changed);
    element.set_callback(move |display| {
        display.scroll(
            display.buffer().unwrap().text().split_whitespace().count() as i32,
            0,
        )
    });
    element.handle(move |display, event| {
        if event == HEARTBEAT {
            let theme = app::GlobalState::<Model>::get().with(move |model| model.theme);
            display.set_color(crate::COLORS[theme as usize][0]);
            display.set_text_color(crate::COLORS[theme as usize][1]);
            let value = app::GlobalState::<Model>::get().with(move |model| model.output.clone());
            display.buffer().unwrap().set_text(&value);
        };
        false
    })
}

fn output(tooltip: &str) -> Frame {
    let mut element = Frame::default().with_align(Align::Right | Align::Inside);
    element.set_label_size(HEIGHT);
    element.set_tooltip(tooltip);
    element.set_frame(FrameType::FlatBox);
    element
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default().with_type(MenuButtonType::Popup3);
    element.set_frame(FrameType::FlatBox);
    element.set_tooltip("Menu");
    let mut item: i32 = element.add(
        "&Night mode\t",
        Shortcut::Ctrl | 'n',
        MenuFlag::Toggle,
        move |_| {
            app::GlobalState::<Model>::get().with(move |model| model.theme = !model.theme);
            app::handle_main(HEARTBEAT).unwrap();
            app::redraw();
        },
    );
    match app::GlobalState::<Model>::get().with(move |model| model.theme) {
        true => element.at(item).unwrap().set(),
        false => element.at(item).unwrap().clear(),
    };
    element.add(
        "@#search  &Info",
        Shortcut::Ctrl | 'i',
        MenuFlag::Normal,
        crate::info,
    );
    item = element.add(
        "@#1+  &Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(Event::Close).unwrap();
        },
    );
    element.at(item).unwrap().set_label_color(Color::Red);
    element.handle(move |menu, event| {
        if event == HEARTBEAT {
            let theme = app::GlobalState::<Model>::get().with(move |model| model.theme);
            menu.set_color(crate::COLORS[theme as usize][0]);
            menu.set_text_color(crate::COLORS[theme as usize][1]);
        };
        false
    });
    element
}

fn info(_: &mut MenuButton) {
    const INFO: &str = "<p>
<a href=\"https://gitlab.com/kbit/kbit.gitlab.io/-/tree/master/app/front/flcalculator\">FlCalculator</a>
 is similar to
 <a href=\"https://apps.gnome.org/Calculator\">Calculator</a>
 written using
 <a href=\"https://fltk-rs.github.io/fltk-rs\">FLTK-RS</a>
</p>";
    let mut dialog = HelpDialog::default();
    dialog.set_value(INFO);
    dialog.set_text_size(16);
    dialog.show();
    while dialog.shown() {
        app::wait();
    }
}

fn theme(flex: &mut Flex, event: Event) -> bool {
    if event == HEARTBEAT {
        let theme = app::GlobalState::<Model>::get().with(move |model| model.theme);
        for idx in 0..flex.children() {
            let mut frame = flex.child(idx).unwrap();
            frame.set_color(crate::COLORS[theme as usize][0]);
            frame.set_label_color(crate::COLORS[theme as usize][1]);
        }
    };
    false
}

fn window() -> Window {
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
    const NAME: &str = "FlCalculator";
    let file = std::env::var("HOME").unwrap() + "/.config/" + NAME;
    app::GlobalState::<Model>::new(Model::default(&file));
    let mut element = Window::default()
        .with_size(360, 640)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(false);
    element.set_xclass(NAME);
    element.set_icon(Some(SvgImage::from_data(SVG).unwrap()));
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            let value = file.clone();
            app::GlobalState::<Model>::get().with(move |model| model.save(&value));
            app::quit();
        }
    });
    element
}

fn popup(flex: &mut Flex, event: Event) -> bool {
    match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                crate::menu().popup();
                true
            }
            _ => false,
        },
        Event::Enter => {
            flex.window().unwrap().set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            flex.window().unwrap().set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    }
}

fn click(button: &mut Button, event: Event) -> bool {
    match event {
        Event::Push => {
            let value = button.label();
            app::GlobalState::<Model>::get().with(move |model| model.click(&value));
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        HEARTBEAT => {
            let theme = app::GlobalState::<Model>::get().with(move |model| model.theme);
            match button.label().as_str() {
                "C" | "x" | "/" | "+" | "-" | "%" => {
                    button.set_color(crate::COLORS[theme as usize][2]);
                    button.set_label_color(crate::COLORS[theme as usize][0]);
                }
                "CE" => {
                    button.set_color(crate::COLORS[theme as usize][4]);
                    button.set_label_color(crate::COLORS[theme as usize][0]);
                }
                crate::EQUAL => {
                    button.set_color(crate::COLORS[theme as usize][5]);
                    button.set_label_color(crate::COLORS[theme as usize][0]);
                }
                _ => {
                    button.set_color(crate::COLORS[theme as usize][3]);
                    button.set_label_color(crate::COLORS[theme as usize][1]);
                }
            };
            false
        }
        _ => false,
    }
}
