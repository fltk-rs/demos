use fltk::enums::Color;

pub const U8: i32 = 255;
pub const WIDGET_WIDTH: i32 = 112;
pub const WIDGET_SPACE: i32 = 10;
pub const WIDGET_HEIGHT: i32 = 25;
pub const TEXT_SIZE: i32 = 14;
pub const THEMES: [&str; 2] = ["Light", "Dark"];
pub const COLORS: [[Color; 2]; 2] = [
    [Color::from_hex(0xfdf6e3), Color::from_hex(0x586e75)],
    [Color::from_hex(0x002b36), Color::from_hex(0x93a1a1)],
];
pub const PARAMS: [u8; 9] = [
    0,   // [0] app_theme
    1,   // [1] window_width * U8 +
    105, // [2] window_width_fract
    2,   // [3] window_height * U8 +
    130, // [4] window_height_fract
    119, // [5] header_from
    35,  // [6] header_to
    0,   // [7] footer_font
    14,  // [8] footer_size
];
pub const CFG: &str = "/.config/fldialect";
pub const APPNAME: &str = "FlDialect";
pub const INFO: &str = r#"<p>
<a href="https://gitlab.com/kbit/kbit.gitlab.io/-/tree/master/app/front/fltk-dialect">FlDialect</a>
 is similar to
 <a href="https://apps.gnome.org/Dialect">Dialect</a>
 written using
 <a href="https://fltk-rs.github.io/fltk-rs">FLTK-RS</a>
 and <a href="https://github.com/soimort/translate-shell">translate-shell</a>.
</p>"#;
pub const SVG: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
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

#[derive(Clone)]
pub enum Message {
    Open,
    Save,
    Tick,
    Size,
    Font,
    Hide,
    Resize,
    Reload([u8; 9]),
    Request,
    Responce(String),
    Themes(u8),
    Switch,
    Info,
    Quit(bool),
}
