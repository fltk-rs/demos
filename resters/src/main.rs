#![forbid(unsafe_code)]
use {
    fltk::{
        button::Button,
        enums::*,
        frame::Frame,
        group::Flex,
        image::SvgImage,
        menu::Choice,
        misc::InputChoice,
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
        valuator::Dial,
        window::Window,
        *,
    },
    fltk_theme::{color_themes, ColorTheme},
    json_tools::{Buffer, BufferType, Lexer, Span, TokenType},
    std::thread,
    ureq::{Error, Response},
};

#[derive(Clone)]
struct Widget {
    buffer: TextBuffer,
    choice: Choice,
    input: InputChoice,
    text: TextDisplay,
    status: Frame,
    dial: Dial,
}

impl Widget {
    fn view() -> Self {
        let mut window = crate::window();
        let mut page = Flex::default_fill().column(); //PAGE

        let mut header = Flex::default(); //HEADER
        header.fixed(&Frame::default(), WIDTH);
        let choice = crate::choice();
        header.fixed(&Frame::default(), WIDTH);
        let input = crate::input();
        header.fixed(&crate::info(), HEIGHT);
        header.end();

        let hero = Flex::default(); //HERO
        let buffer = TextBuffer::default();
        let text = crate::text(buffer.clone());
        hero.end();

        let mut footer = Flex::default(); //FOOTER
        footer.fixed(&Frame::default().with_label("Status: "), WIDTH);
        let status = Frame::default().with_align(Align::Left | Align::Inside);
        let dial = crate::dial();
        footer.fixed(&dial, HEIGHT);
        footer.end();

        page.end();
        window.end();
        window.show();
        {
            header.set_pad(PAD);
            header.fixed(&choice, WIDTH);
            page.fixed(&header, HEIGHT);
            page.fixed(&footer, HEIGHT);
            page.set_pad(PAD);
            page.set_margin(PAD);
            page.set_frame(FrameType::FlatBox);
        }
        let mut component = Self {
            buffer,
            choice,
            input,
            text,
            status,
            dial,
        };
        let mut clone = component.clone();
        component.input.set_callback(move |_| clone.update());
        let mut clone = component.clone();
        component
            .input
            .input()
            .set_callback(move |_| clone.update());
        component
    }
    fn update(&mut self) {
        self.status.set_label("");
        self.text.buffer().unwrap().set_text("");
        self.buffer.set_text("");
        let proto = "https://";
        let path = match self.input.value().unwrap().starts_with(proto) {
            true => self.input.value().unwrap(),
            false => String::from(proto) + &self.input.value().unwrap(),
        };
        let req = match self.choice.value() {
            0 => ureq::get(&path),
            1 => ureq::post(&path),
            _ => unreachable!(),
        };
        let handler = thread::spawn(move || -> Result<Response, Error> { req.call() });
        while !handler.is_finished() {
            app::wait();
            app::sleep(0.02);
            self.dial.do_callback();
        }
        if let Ok(req) = handler.join() {
            match req {
                Ok(response) => {
                    if let Ok(json) = response.into_json::<serde_json::Value>() {
                        let json: String = serde_json::to_string_pretty(&json).unwrap();
                        self.text.buffer().unwrap().set_text(&json);
                        self.fill_style_buffer(&json);
                        self.status.set_label("200 OK");
                        self.status.set_label_color(enums::Color::Yellow);
                    } else {
                        dialog::message_default("Error parsing json");
                    }
                }
                Err(Error::Status(code, response)) => {
                    self.status
                        .set_label(&format!("{} {}", code, response.status_text()));
                    self.status.set_label_color(enums::Color::Red);
                }
                Err(e) => {
                    dialog::message_default(&e.to_string());
                }
            }
        };
    }
    fn fill_style_buffer(&mut self, s: &str) {
        let mut buffer = vec![b'A'; s.len()];
        for token in Lexer::new(s.bytes(), BufferType::Span) {
            use TokenType::*;
            let c = match token.kind {
                CurlyOpen | CurlyClose | BracketOpen | BracketClose | Colon | Comma | Invalid => {
                    'A'
                }
                String => 'B',
                BooleanTrue | BooleanFalse | Null => 'C',
                Number => 'D',
            };
            if let Buffer::Span(Span { first, end }) = token.buf {
                let start = first as _;
                let last = end as _;
                buffer[start..last].copy_from_slice(c.to_string().repeat(last - start).as_bytes());
            }
        }
        self.buffer.set_text(&String::from_utf8_lossy(&buffer));
    }
}

fn main() -> Result<(), FltkError> {
    Widget::view();
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::set_font(Font::Courier);
    app::App::default().run()
}

fn window() -> Window {
    const NAME: &str = "FlResters";
    let mut element = Window::default()
        .with_size(960, 540)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(false);
    element.set_xclass(NAME);
    element.set_icon(Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()));
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            app::quit();
        }
    });
    element
}

fn text(buffer: TextBuffer) -> TextDisplay {
    let styles: Vec<StyleTableEntry> = [0xdc322f, 0x268bd2, 0x859900]
        .into_iter()
        .map(|color| StyleTableEntry {
            color: Color::from_hex(color),
            font: Font::Courier,
            size: 16,
        })
        .collect();
    let mut element = TextDisplay::default();
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_buffer(TextBuffer::default());
    element.set_color(Color::from_hex(0x002b36));
    element.set_highlight_data(buffer, styles);
    element
}

fn info() -> Button {
    let mut element = Button::default().with_label("ℹ️");
    element.set_label_size(18);
    element.set_frame(FrameType::NoBox);
    element.set_callback(move |_| {
        dialog::message_default("Resters was created using Rust and fltk-rs. It is MIT licensed!")
    });
    element
}

fn choice() -> Choice {
    let mut element = Choice::default().with_label("Method: ");
    element.add_choice("GET|POST");
    element.set_value(0);
    element
}

fn input() -> InputChoice {
    let mut element = InputChoice::default().with_label("URL: ");
    for item in ["users", "posts", "albums", "todos", "comments", "posts"] {
        element.add(&(format!(r#"https:\/\/jsonplaceholder.typicode.com\/{item}"#)));
    }
    element.add(r#"https:\/\/lingva.thedaviddelta.com\/api\/v1\/languages"#);
    element.add(r#"https:\/\/lingva.thedaviddelta.com\/api\/v1\/en\/de\/mother"#);
    element.add(r#"https:\/\/ipinfo.io\/json"#);
    element.input().set_trigger(CallbackTrigger::EnterKeyAlways);
    element.set_value_index(0);
    element
}

fn dial() -> Dial {
    const DIAL: u8 = 120;
    let mut element = Dial::default();
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

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;
