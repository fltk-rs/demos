#![forbid(unsafe_code)]
use {
    fltk::{enums::*, prelude::*, *},
    fltk_theme::{color_themes, ColorTheme},
    json_tools::{Buffer, BufferType, Lexer, Span, TokenType},
    ureq::Error,
};

fn main() {
    let mut sbuf = text::TextBuffer::default();
    let styles: Vec<text::StyleTableEntry> = [0xdc322f, 0x268bd2, 0x859900]
        .into_iter()
        .map(|color| text::StyleTableEntry {
            color: Color::from_hex(color),
            font: Font::Courier,
            size: 16,
        })
        .collect();
    let mut window = window::Window::default()
        .with_size(640, 360)
        .with_label("flResters")
        .center_screen();
    window.set_xclass("resters");
    let mut flex = group::Flex::default_fill().column();
    let mut header = group::Flex::default();
    flex.fixed(&header, 30);
    let mut choice = menu::Choice::default();
    choice.add_choice("GET|POST");
    choice.set_value(0);
    header.fixed(&choice, 80);
    header.fixed(&frame::Frame::default().with_label("https://"), 60);
    let mut inp = input::Input::default();
    inp.set_trigger(CallbackTrigger::EnterKeyAlways);
    let mut info = button::Button::default().with_label("ℹ️");
    info.set_label_size(18);
    info.set_frame(FrameType::NoBox);
    info.set_callback(move |_| {
        dialog::message_default("Resters was created using Rust and fltk-rs. It is MIT licensed!")
    });
    header.fixed(&info, 30);
    header.end();
    header.set_pad(10);
    let mut disp = text::TextDisplay::default();
    disp.wrap_mode(text::WrapMode::AtBounds, 0);
    disp.set_buffer(text::TextBuffer::default());
    disp.set_color(Color::from_hex(0x002b36));
    disp.set_highlight_data(sbuf.clone(), styles);
    let mut footer = group::Flex::default();
    flex.fixed(&footer, 20);
    footer.fixed(&frame::Frame::default().with_label("Status: "), 80);
    let mut status = frame::Frame::default().with_align(Align::Left | Align::Inside);
    footer.end();
    flex.end();
    flex.set_pad(10);
    flex.set_margin(10);
    window.end();
    window.make_resizable(true);
    window.show();
    window.set_icon(Some(image::SvgImage::from_data(SVG).unwrap()));
    inp.set_callback(move |inp| {
        status.set_label("");
        disp.buffer().unwrap().set_text("");
        sbuf.set_text("");
        let mut path = inp.value();
        if !path.starts_with("https://") {
            path = String::from("https://") + &path;
        }
        let req = match choice.value() {
            0 => ureq::get(&path),
            1 => ureq::post(&path),
            _ => unreachable!(),
        };
        match req.call() {
            Ok(response) => {
                if let Ok(json) = response.into_json::<serde_json::Value>() {
                    let json: String = serde_json::to_string_pretty(&json).unwrap();
                    disp.buffer().unwrap().set_text(&json);
                    fill_style_buffer(&mut sbuf, &json);
                    status.set_label("200 OK");
                    status.set_label_color(enums::Color::Yellow);
                } else {
                    dialog::message_default("Error parsing json");
                }
            }
            Err(Error::Status(code, response)) => {
                status.set_label(&format!("{} {}", code, response.status_text()));
                status.set_label_color(enums::Color::Red);
            }
            Err(e) => {
                dialog::message_default(&e.to_string());
            }
        }
    });
    app::set_font(Font::Courier);
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::App::default().with_scheme(app::Scheme::Plastic).run().unwrap();
}

fn fill_style_buffer(sbuf: &mut text::TextBuffer, s: &str) {
    let mut local_buf = vec![b'A'; s.len()];
    for token in Lexer::new(s.bytes(), BufferType::Span) {
        use TokenType::*;
        let c = match token.kind {
            CurlyOpen | CurlyClose | BracketOpen | BracketClose | Colon | Comma | Invalid => 'A',
            String => 'B',
            BooleanTrue | BooleanFalse | Null => 'C',
            Number => 'D',
        };
        if let Buffer::Span(Span { first, end }) = token.buf {
            let start = first as _;
            let last = end as _;
            local_buf[start..last].copy_from_slice(c.to_string().repeat(last - start).as_bytes());
        }
    }
    sbuf.set_text(&String::from_utf8_lossy(&local_buf));
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
