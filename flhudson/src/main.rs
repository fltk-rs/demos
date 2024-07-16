#![forbid(unsafe_code)]

mod actions;

use {
    actions::Action,
    fltk::{
        app,
        app::WidgetId,
        button::Button,
        enums::{Align, Event, FrameType},
        frame::Frame,
        group::{Flex, Wizard},
        image::SvgImage,
        menu::Choice,
        misc::HelpView,
        prelude::{DisplayExt, GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt},
        text::{TextBuffer, TextEditor, WrapMode},
        valuator::{Counter, CounterType, Dial},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    std::{env, fs, path::Path, thread},
};

pub const PAD: i32 = 10;
pub const HEIGHT: i32 = PAD * 3;
pub const WIDTH: i32 = HEIGHT * 3;

pub struct Param {
    case: String,
    path: String,
}

impl Param {
    pub fn build(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        Ok(Self {
            case: args[1].clone(),
            path: args[2].clone(),
        })
    }
}

fn main() {
    if actions::once() {
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 1 {
            app();
        } else {
            let param = Param::build(args).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {err}");
                std::process::exit(1);
            });
            if let Err(e) = actions::route(param) {
                eprintln!("Application error: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn app() {
    let app = app::App::default();
    let mut window = crate::window();
    let mut page = Flex::default_fill().column().with_id("Page");

    let mut header = Flex::default().with_id("Header");
    header.fixed(&Frame::default(), WIDTH + 5);
    let mut jobs = crate::choice("Choice", "Info|Virsh|Meter");
    jobs.set_callback(move |choice| {
        let mut wizard = app::widget_from_id::<Wizard>("Hero").unwrap();
        wizard.set_current_widget(&wizard.child(choice.value()).unwrap());
        wizard.redraw();
    });
    header.fixed(&jobs, WIDTH);
    Frame::default();
    header.fixed(&crate::dial("Spinner"), HEIGHT);
    crate::button("Run", "@#+2->", &mut header).set_callback(crate::run);
    header.end();

    let mut hero = Wizard::default_fill().with_id("Hero");
    HelpView::default_fill().set_value(INFO);
    let mut slide = Flex::default_fill().with_label("Virsh");
    slide.fixed(&Frame::default(), WIDTH);
    let mut right = Flex::default().column();
    crate::text("Nodes", "1");
    right.end();
    right.set_pad(PAD);
    slide.end();
    slide.set_margin(PAD);
    let mut slide = Flex::default_fill().with_label("Siege");
    slide.fixed(&Frame::default(), WIDTH);
    let mut col = Flex::default().column();
    col.fixed(&crate::choice("Proto", "https|http"), HEIGHT);
    col.fixed(&crate::choice("Port", "443|80"), HEIGHT);
    col.fixed(&crate::counter("Concurrent", 8f64), HEIGHT);
    crate::text("Targets", "127.0.0.1");
    crate::text("Endpoints", "/");
    right.end();
    right.set_pad(PAD);
    slide.end();
    slide.set_margin(PAD);
    hero.end();

    page.end();
    window.end();
    window.show();
    {
        header.set_pad(PAD);
        header.set_margin(0);
        hero.set_frame(FrameType::EngravedBox);
        page.fixed(&header, HEIGHT);
        page.set_margin(PAD);
        page.set_pad(PAD);
        page.set_frame(FrameType::FlatBox);
        ColorTheme::new(color_themes::DARK_THEME).apply();
    }
    app.run().unwrap();
}

const INFO: &str = r#"<p>
<a href="https://www.jenkins.io/">FlHudson</a> is similar to
 <a href="https://jenkins.io">Jenkins</a> written using
 <a href="https://fltk-rs.github.io/fltk-rs">FLTK-RS</a>.
</p>"#;

pub fn window() -> Window {
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
    const NAME: &str = "FlHudson";
    const U8: i32 = 255;
    const DEFAULT: [u8; 4] = [
        2,   // [2] window_width * U8 +
        130, // [3] window_width_fract
        1,   // [4] window_height * U8 +
        105, // [5] window_height_fract
    ];
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
                ],
            )
            .unwrap();
            app::quit();
        }
    });
    element
}

pub fn text(tooltip: &str, text: &str) {
    let mut element = TextEditor::default()
        .with_label(tooltip)
        .with_id(tooltip)
        .with_align(Align::Left);
    element.set_tooltip(tooltip);
    element.set_buffer(TextBuffer::default());
    element.set_linenumber_width(crate::HEIGHT);
    element.buffer().unwrap().set_text(text);
    element.wrap_mode(WrapMode::AtBounds, 0);
}

pub fn dial(tooltip: &str) -> Dial {
    const DIAL: u8 = 120;
    let mut element = Dial::default().with_id(tooltip);
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

pub fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    flex.fixed(&element, crate::HEIGHT);
    element
}

pub fn choice(tooltip: &str, choice: &str) -> Choice {
    let mut element = Choice::default().with_label(tooltip).with_id(tooltip);
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(0);
    element
}

pub fn counter(tooltip: &str, value: f64) -> Counter {
    let mut element = Counter::default()
        .with_type(CounterType::Simple)
        .with_align(Align::Left)
        .with_label(tooltip)
        .with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_range(1_f64, 8_f64);
    element.set_precision(0);
    element.set_value(value);
    element
}

pub fn run(button: &mut Button) {
    button.deactivate();
    let handler =
        thread::spawn(
            move || match app::widget_from_id::<Choice>("Choice").unwrap().value() {
                1 => actions::save_xlsx(
                    &(std::env::var("HOME").unwrap() + "/report.xlsx"),
                    &actions::Virsh {
                        nodes: app::widget_from_id::<TextEditor>("Nodes")
                            .unwrap()
                            .buffer()
                            .unwrap()
                            .text()
                            .split_whitespace()
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                    }
                    .run(),
                ),
                2 => actions::save_xlsx(
                    &(std::env::var("HOME").unwrap() + "/report.xlsx"),
                    &actions::Siege {
                        proto: app::widget_from_id::<Choice>("Proto")
                            .unwrap()
                            .choice()
                            .unwrap()
                            .as_str()
                            .to_string(),
                        port: app::widget_from_id::<Choice>("Port")
                            .unwrap()
                            .choice()
                            .unwrap()
                            .parse::<u16>()
                            .unwrap(),
                        concurrent: app::widget_from_id::<Counter>("Concurrent")
                            .unwrap()
                            .value() as u8,
                        targets: app::widget_from_id::<TextEditor>("Targets")
                            .unwrap()
                            .buffer()
                            .unwrap()
                            .text()
                            .split_whitespace()
                            .map(str::to_string)
                            .collect::<Vec<String>>(),
                        endpoints: app::widget_from_id::<TextEditor>("Endpoints")
                            .unwrap()
                            .buffer()
                            .unwrap()
                            .text()
                            .split_whitespace()
                            .map(str::to_string)
                            .collect::<Vec<String>>(),
                    }
                    .run(),
                ),
                _ => {}
            },
        );
    let mut dial = app::widget_from_id::<Dial>("Spinner").unwrap();
    while !handler.is_finished() {
        app::wait();
        app::sleep(0.02);
        dial.do_callback();
    }
    button.activate();
}
