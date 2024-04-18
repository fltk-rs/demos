#![forbid(unsafe_code)]

use {
    fltk::{
        app,
        app::WidgetId,
        button::Button,
        dialog::HelpDialog,
        enums::{Align, CallbackTrigger, Color, Event, Font, FrameType, Key, Shortcut},
        frame::Frame,
        group::Flex,
        image::SvgImage,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::{ButtonExt, DisplayExt, GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
        text::{TextBuffer, TextDisplay, WrapMode},
        window::Window,
    },
    std::{env, fs, path::Path},
};

const BUTTONS: [[&str; 4]; 5] = [
    ["CE", "C", "%", "/"],
    ["7", "8", "9", "x"],
    ["4", "5", "6", "-"],
    ["1", "2", "3", "+"],
    ["0", ".", "@<-", "="],
];
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

fn main() {
    // let mut temp: f64 = 0.0;
    let app = app::App::default();
    let (mut window, theme) = crate::window();
    let mut page = Flex::default_fill().column().with_id("Page");
    crate::display("Output");
    let mut row = Flex::default();
    row.fixed(&crate::output("Operation", ""), 30);
    let mut col = Flex::default().column();
    crate::output("Previous", "0");
    crate::output("Current", "0");
    col.end();
    col.set_pad(0);
    row.end();
    row.set_pad(0);
    let mut buttons = Flex::default_fill().column().with_id("Buttons");
    for line in BUTTONS {
        let mut row = Flex::default();
        for label in line {
            button(label).set_callback(crate::run);
        }
        row.end();
        row.set_pad(10);
        row.set_margin(0);
    }
    buttons.end();
    page.end();
    window.end();
    window.show();
    crate::menu("Menu", theme);
    {
        row.set_pad(PAD);
        row.set_margin(0);
        buttons.set_pad(10);
        buttons.set_margin(0);
        page.set_margin(PAD);
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.fixed(&row, 60);
        page.fixed(&buttons, 425);
        app::set_font(Font::Courier);
    }
    app.run().unwrap();
}

fn button(title: &'static str) -> Button {
    let mut element = Button::default().with_label(title).with_id(title);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::OFlatFrame);
    match title {
        "@<-" => element.set_shortcut(Shortcut::None | Key::BackSpace),
        "CE" => element.set_shortcut(Shortcut::None | Key::Delete),
        "=" => element.set_shortcut(Shortcut::None | Key::Enter),
        "x" => element.set_shortcut(Shortcut::None | '*'),
        _ => element.set_shortcut(Shortcut::None | title.chars().next().unwrap()),
    }
    element
}
pub fn display(tooltip: &str) -> TextDisplay {
    let mut element = TextDisplay::default().with_id(tooltip);
    element.set_text_size(HEIGHT - 5);
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
    element
}
pub fn output(tooltip: &str, label: &str) -> Frame {
    let mut element = Frame::default()
        .with_align(Align::Right | Align::Inside)
        .with_id(tooltip);
    element.set_label_size(HEIGHT);
    element.set_label(label);
    element.set_frame(FrameType::FlatBox);
    element
}
pub fn menu(tooltip: &str, theme: u8) -> MenuButton {
    let mut element = MenuButton::default()
        .with_id(tooltip)
        .with_type(MenuButtonType::Popup3);
    element.set_frame(FrameType::FlatBox);
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
        "@#search  &Info",
        Shortcut::Ctrl | 'i',
        MenuFlag::Normal,
        crate::info,
    );
    let ord: i32 = element.add(
        "@#1+  &Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        move |_| println!("Quit"),
    );
    element.at(ord).unwrap().set_label_color(Color::Red);
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
pub fn hide(_: &mut MenuButton) {
    let mut page = app::widget_from_id::<Flex>("Page").unwrap();
    let mut footer = app::widget_from_id::<Flex>("Buttons").unwrap();
    if footer.visible() {
        page.fixed(&footer, 0);
        footer.hide();
    } else {
        page.fixed(&footer, 425);
        footer.show();
    };
    page.redraw();
}
pub fn theme(menu: &mut MenuButton) {
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
    let theme = menu.at(1).unwrap().value();
    let mut window = app::first_window().unwrap();
    let mut operation = app::widget_from_id::<Frame>("Operation").unwrap();
    let mut prev = app::widget_from_id::<Frame>("Previous").unwrap();
    let mut output = app::widget_from_id::<Frame>("Current").unwrap();
    let mut display = app::widget_from_id::<TextDisplay>("Output").unwrap();
    window.set_color(COLORS[theme as usize][0]);
    output.set_color(COLORS[theme as usize][0]);
    output.set_label_color(COLORS[theme as usize][1]);
    operation.set_color(COLORS[theme as usize][0]);
    operation.set_label_color(COLORS[theme as usize][1]);
    prev.set_color(COLORS[theme as usize][0]);
    prev.set_label_color(COLORS[theme as usize][1]);
    menu.set_color(COLORS[theme as usize][1]);
    menu.set_text_color(COLORS[theme as usize][0]);
    display.set_color(COLORS[theme as usize][0]);
    display.set_text_color(COLORS[theme as usize][1]);
    for row in BUTTONS {
        for label in row {
            let mut button = app::widget_from_id::<Button>(label).unwrap();
            match button.label().as_str() {
                "C" | "x" | "/" | "+" | "-" | "%" => {
                    button.set_color(COLORS[theme as usize][2]);
                    button.set_label_color(COLORS[theme as usize][0]);
                }
                "CE" => {
                    button.set_color(COLORS[theme as usize][4]);
                    button.set_label_color(COLORS[theme as usize][0]);
                }
                "=" => {
                    button.set_color(COLORS[theme as usize][5]);
                    button.set_label_color(COLORS[theme as usize][0]);
                }
                _ => {
                    button.set_color(COLORS[theme as usize][3]);
                    button.set_label_color(COLORS[theme as usize][1]);
                }
            };
        }
    }
    window.redraw();
}

fn window() -> (Window, u8) {
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
    let file = env::var("HOME").unwrap() + "/.config/" + NAME;
    let theme: u8 = match Path::new(&file).exists() {
        true => fs::read(&file).unwrap()[0],
        false => 0,
    };
    let mut element = Window::default()
        .with_label(NAME)
        .with_size(360, 640)
        .center_screen();
    element.make_resizable(false);
    element.set_xclass(NAME);
    element.set_icon(Some(SvgImage::from_data(SVG).unwrap()));
    element.handle(move |_, event| match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                app::widget_from_id::<MenuButton>("Menu").unwrap().popup();
                true
            }
            _ => false,
        },
        _ => false,
    });
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            fs::write(
                &file,
                [app::widget_from_id::<MenuButton>("Menu")
                    .unwrap()
                    .at(1)
                    .unwrap()
                    .value() as u8],
            )
            .unwrap();
            app::quit();
        }
    });
    (element, theme)
}

fn run(button: &mut Button) {
    let mut prev = app::widget_from_id::<Frame>("Previous").unwrap();
    let mut current = app::widget_from_id::<Frame>("Current").unwrap();
    let mut operation = app::widget_from_id::<Frame>("Operation").unwrap();
    let mut output = app::widget_from_id::<TextDisplay>("Output").unwrap();
    match button.label().as_str() {
        "/" | "x" | "+" | "-" | "%" => {
            if operation.label().is_empty() {
                operation.set_label(&button.label());
                prev.set_label(&current.label());
            } else {
                app::widget_from_id::<Button>("=").unwrap().do_callback();
                operation.set_label(&button.label());
            }
            output
                .buffer()
                .unwrap()
                .append(&format!("{} {}", prev.label(), operation.label()));
            output.do_callback();
            current.set_label("0");
        }
        "=" => {
            if !operation.label().is_empty() {
                let left: f64 = prev.label().parse().unwrap();
                let right: f64 = current.label().parse().unwrap();
                let temp = match operation.label().as_str() {
                    "/" => left / right,
                    "x" => left * right,
                    "+" => left + right,
                    "-" => left - right,
                    _ => left / 100.0 * right,
                };
                output.buffer().unwrap().append(&format!(
                    " {right}\n{} = {temp}\n",
                    (0..=left.to_string().len())
                        .map(|_| ' ')
                        .collect::<String>(),
                ));
                output.do_callback();
                prev.set_label(&temp.to_string());
            } else {
                prev.set_label(&current.label());
            }
            operation.set_label("");
            current.set_label("0");
        }
        "CE" => {
            output.buffer().unwrap().set_text("");
            operation.set_label("");
            current.set_label("0");
            prev.set_label("0");
        }
        "@<-" => {
            let label = current.label();
            current.set_label(if label.len() > 1 {
                &label[..label.len() - 1]
            } else {
                "0"
            });
        }
        "C" => current.set_label("0"),
        "." => {
            if !current.label().contains('.') {
                current.set_label(&(output.label() + "."));
            }
        }
        _ => {
            if current.label() == "0" {
                current.set_label("");
            }
            current.set_label(&(current.label() + &button.label()));
        }
    }
    app::redraw();
}
