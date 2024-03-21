use {
    fltk::{
        app,
        button::Button,
        dialog::HelpDialog,
        enums::{Align, Color, Event, Font, FrameType, Key, Shortcut},
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

fn main() {
    let file = env::var("HOME").unwrap() + CFG;
    let mut theme: u8 = match Path::new(&file).exists() {
        true => fs::read(&file).unwrap()[0],
        false => 0,
    };
    let mut current_operation = '=';
    let mut current_value = String::from("0");
    let mut temp: f64 = 0.0;
    let (sender, receiver) = app::channel::<Message>();
    let mut menu = MenuButton::default().with_type(MenuButtonType::Popup3);
    for (ord, item) in THEMES.iter().enumerate() {
        let idx = menu.add_emit(
            &format!("&Theme/&{}\t", item),
            Shortcut::Ctrl | item.to_lowercase().chars().next().unwrap(),
            MenuFlag::Radio,
            sender,
            Message::Themes(ord as u8),
        );
        if ord == theme as usize {
            menu.at(idx).unwrap().set();
        };
    }
    let idx: i32 = menu.add_emit(
        "&View/&Footer\t",
        Shortcut::None,
        MenuFlag::Toggle,
        sender,
        Message::Hide,
    );
    menu.at(idx).unwrap().set();
    for (label, chr, msg) in [
        ("@#reload  &Reload", 's', Message::Reload),
        ("@#search  &Info", 'i', Message::Info),
        ("@#1+  &Quit", 'q', Message::Quit(true)),
    ] {
        menu.add_emit(
            &format!("{label}\t"),
            Shortcut::Ctrl | chr,
            MenuFlag::Normal,
            sender,
            msg,
        );
    }
    menu.set_frame(FrameType::FlatBox);
    let mut window = Window::default()
        .with_label("FlCalculator")
        .with_size(360, 640)
        .center_screen();
    let mut vbox = Flex::default_fill().column();
    let mut header = TextDisplay::default();
    header.set_text_size(SIZE);
    header.set_frame(FrameType::FlatBox);
    header.wrap_mode(WrapMode::AtBounds, 0);
    header.set_buffer(TextBuffer::default());
    let menu_clone = menu.clone();
    header.handle(move |_, event| match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                menu_clone.popup();
                true
            }
            _ => false,
        },
        _ => false,
    });
    let mut output = Frame::default().with_align(Align::Right | Align::Inside);
    output.set_label_color(COLORS[0][1]);
    output.set_color(COLORS[0][0]);
    output.set_label_size(OUTPUT);
    output.set_label("0");
    output.set_frame(FrameType::FlatBox);
    let mut footer = Flex::default_fill().column();
    let mut buttons: Vec<Button> = Vec::new();
    for row in [
        [
            ("CE", Message::CE),
            ("C", Message::C),
            ("%", Message::Ops('%')),
            ("/", Message::Ops('/')),
        ],
        [
            ("7", Message::Number('7')),
            ("8", Message::Number('8')),
            ("9", Message::Number('9')),
            ("x", Message::Ops('x')),
        ],
        [
            ("4", Message::Number('4')),
            ("5", Message::Number('5')),
            ("6", Message::Number('6')),
            ("-", Message::Ops('-')),
        ],
        [
            ("1", Message::Number('1')),
            ("2", Message::Number('2')),
            ("3", Message::Number('3')),
            ("+", Message::Ops('+')),
        ],
        [
            ("0", Message::Number('0')),
            (".", Message::Dot),
            ("@<-", Message::Back),
            ("=", Message::Ops('=')),
        ],
    ] {
        let mut hbox = Flex::default();
        for (item, msg) in row {
            let mut button = button(item);
            button.emit(sender, msg);
            buttons.push(button);
        }
        hbox.end();
        hbox.set_pad(10);
    }
    footer.end();
    footer.set_pad(10);
    vbox.end();
    vbox.fixed(&output, OUTPUT);
    vbox.fixed(&footer, 420);
    vbox.set_margin(10);
    vbox.set_pad(10);
    window.make_resizable(false);
    window.end();
    window.show();
    window.set_icon(Some(SvgImage::from_data(SVG).unwrap()));
    window.emit(sender, Message::Quit(false));
    sender.send(Message::Themes(theme));
    app::set_font(Font::Courier);
    while app::App::default().load_system_fonts().wait() {
        match receiver.recv() {
            Some(Message::Quit(force)) => {
                if force || app::event() == Event::Close {
                    fs::write(&file, [theme]).unwrap();
                    app::quit();
                }
            }
            Some(Message::Info) => {
                let mut dialog = HelpDialog::default();
                dialog.set_value(INFO);
                dialog.set_text_size(16);
                dialog.show();
                while dialog.shown() {
                    app::wait();
                }
            }
            Some(Message::Themes(ord)) => {
                theme = ord;
                window.set_color(COLORS[theme as usize][0]);
                output.set_color(COLORS[theme as usize][0]);
                menu.set_color(COLORS[theme as usize][1]);
                menu.set_text_color(COLORS[theme as usize][0]);
                output.set_label_color(COLORS[theme as usize][1]);
                header.set_color(COLORS[theme as usize][0]);
                header.set_text_color(COLORS[theme as usize][1]);
                for button in &mut buttons {
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
                app::redraw();
            }
            Some(Message::Reload) => sender.send(Message::Themes(0)),
            Some(Message::Hide) => {
                match menu.at(idx).unwrap().value() {
                    false => {
                        footer.hide();
                        vbox.fixed(&footer, 0);
                    }
                    true => {
                        vbox.fixed(&footer, 420);
                        footer.show();
                    }
                };
                app::redraw();
            }
            Some(Message::Number(num)) => {
                if output.label() == "0" {
                    current_value.clear();
                }
                current_value.push(num);
                output.set_label(&current_value);
                app::redraw();
            }
            Some(Message::Dot) => {
                if !current_value.contains('.') {
                    current_value.push('.');
                    output.set_label(&current_value);
                    app::redraw();
                }
            }
            Some(Message::Back) => {
                if current_value.len() > 1 {
                    current_value.pop();
                } else {
                    current_value = String::from("0");
                };
                output.set_label(&current_value);
                app::redraw();
            }
            Some(Message::C) => {
                current_value = String::from("0");
                output.set_label(&current_value);
                app::redraw();
            }
            Some(Message::CE) => {
                header.buffer().unwrap().set_text("");
                temp = 0.0;
                current_operation = '=';
                sender.send(Message::C);
                app::redraw();
            }
            Some(Message::Ops(ops)) => {
                match current_operation {
                    '=' => {
                        if ops != '=' {
                            temp = output.label().parse().unwrap();
                            current_value = String::from("0");
                            output.set_label(&current_value);
                            header.buffer().unwrap().append(&format!("{temp} {ops}"));
                        }
                    }
                    _ => {
                        let left: f64 = temp;
                        let right: f64 = output.label().parse().unwrap();
                        temp = match current_operation {
                            '/' => left / right,
                            'x' => left * right,
                            '+' => left + right,
                            '-' => left - right,
                            '%' => left / 100.0 * right,
                            _ => right,
                        };
                        header.buffer().unwrap().append(&format!(
                            " {right}\n{}= {temp}\n",
                            (0..=left.to_string().len())
                                .map(|_| ' ')
                                .collect::<String>()
                        ));
                        if ops != '=' {
                            header.buffer().unwrap().append(&format!("{temp} {ops}"));
                        };
                        current_value = String::from("0");
                        output.set_label(&current_value);
                        header.scroll(
                            header.buffer().unwrap().text().split_whitespace().count() as i32,
                            0,
                        );
                    }
                };
                current_operation = ops;
                app::redraw();
            }
            None => {}
        }
    }
}

fn button(title: &'static str) -> Button {
    let mut element = Button::default().with_label(title);
    element.set_label_size(SIZE);
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

const INFO: &str = "<p>
<a href=\"https://gitlab.com/kbit/kbit.gitlab.io/-/tree/master/app/front/flcalculator\">FlCalculator</a>
 is similar to
 <a href=\"https://apps.gnome.org/Calculator\">Calculator</a>
 written using
 <a href=\"https://fltk-rs.github.io/fltk-rs\">FLTK-RS</a>
</p>";

#[derive(Copy, Clone)]
enum Message {
    Info,
    Quit(bool),
    Hide,
    Reload,
    Themes(u8),
    Number(char),
    Ops(char),
    Dot,
    Back,
    CE,
    C,
}

const CFG: &str = "/.config/flcalculator";
const SIZE: i32 = 25;
const OUTPUT: i32 = 36;
const THEMES: [&str; 2] = ["Light", "Dark"];
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
