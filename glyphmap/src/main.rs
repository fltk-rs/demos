#![forbid(unsafe_code)]
use {
    fltk::{
        dialog::{FileChooser, FileChooserType},
        enums::*,
        frame::Frame,
        group::{Pack, PackType, Scroll, ScrollType},
        input::{Input, InputType},
        menu::{MenuBar, MenuFlag},
        output::Output,
        prelude::*,
        window::Window,
        *,
    },
    fltk_theme::{color_themes, ColorTheme},
    std::fs,
    ttf_parser::Face,
};

fn main() {
    let app = app::App::default();
    dialog::message_title_default(crate::NAME);
    let mut window = crate::window();
    crate::menu();
    let scroll = Scroll::new(0, 30, WIDTH, HEIGHT - 100, None)
        .with_type(ScrollType::Vertical)
        .with_id(crate::SCROLL);
    let pack = Pack::default_fill().with_id(crate::PACK);
    pack.end();
    scroll.end();
    crate::input().with_id(crate::INPUT).below_of(&scroll, 2);
    window.end();
    window.show();

    ColorTheme::new(color_themes::DARK_THEME).apply();
    app.run().unwrap();
}

fn window() -> Window {
    let mut element = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label(crate::NAME);
    element.set_xclass("glyphmap");
    element.make_resizable(false);
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            app::quit();
        }
    });
    element
}

fn menu() {
    let mut element = MenuBar::new(0, 0, WIDTH, 30, None);
    element.set_text_size(14);
    element.add(
        "&File/@#fileopen  L&oad font...",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        crate::load,
    );
    element.add(
        "&About",
        Shortcut::Ctrl | 'a',
        MenuFlag::Normal,
        move |_| dialog::alert_default(TEXT),
    );
    let ord: i32 = element.add(
        "&File/@#1+  &Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(Event::Close).unwrap();
        },
    );
    element.at(ord).unwrap().set_label_color(Color::Red);
}

fn load(menu: &mut MenuBar) {
    let mut dialog = FileChooser::new(
        std::env::var("HOME").unwrap(),
        "*.{ttf,otf}",
        FileChooserType::Single,
        "Open ...",
    );
    dialog.show();
    while dialog.shown() {
        app::wait();
    }
    if dialog.count() > 0 {
        if let Some(file) = dialog.value(1) {
            let font = Font::load_font(&file).unwrap();
            menu.window()
                .unwrap()
                .set_label(&format!("{}-{NAME}", font));
            let mut input = app::widget_from_id::<Input>(crate::INPUT).unwrap();
            input.set_text_font(Font::by_name(&font));
            input.redraw();
            let mut scroll = app::widget_from_id::<Scroll>(crate::SCROLL).unwrap();
            scroll.scroll_to(0, 0);
            let mut pack = app::widget_from_id::<Pack>(crate::PACK).unwrap();
            pack.clear();
            pack.begin();
            let data = fs::read(&file).unwrap();
            let face = Face::from_slice(&data, 0).unwrap();
            let cmap = face.tables().cmap.unwrap();
            if let Some(table) = cmap.subtables.into_iter().last() {
                table.codepoints(|codepoint| {
                    let c = char::from_u32(codepoint).unwrap();
                    let txt = String::from(c);
                    let (w, h) = draw::measure(&txt, true);
                    if w != 0 && h != 0 && face.glyph_index(c).is_some() {
                        let hpack = Pack::default()
                            .with_type(PackType::Horizontal)
                            .with_size(0, 30);
                        Frame::default()
                            .with_size(100, 0)
                            .with_label(&codepoint.to_string());
                        crate::frame().with_size(100, 0);
                        let mut out = Output::default().with_size(WIDTH - 200, 0);
                        out.set_text_font(Font::by_name(&font));
                        out.set_frame(FrameType::FlatBox);
                        out.set_text_size(22);
                        out.set_value(&txt);
                        hpack.end();
                    }
                });
            }
            pack.end();
            scroll.redraw();
        }
    }
}

fn input() -> Input {
    let mut element = Input::default()
        .with_type(InputType::Multiline)
        .with_size(WIDTH, 67);
    element.set_text_size(22);
    element
}

fn frame() -> Frame {
    let mut element = Frame::default();
    element.set_frame(FrameType::FlatBox);
    element.set_color(Color::White);
    element
}

const HEIGHT: i32 = 360;
const WIDTH: i32 = 640;
const INPUT: &str = "INPUT";
const PACK: &str = "PACK";
const SCROLL: &str = "SCROLL";
const NAME: &str = "Glyph Map";
const TEXT: &str = r#"This app can be used to visualize fonts and their corresponding codepoints,
which can then be used in fltk-rs apps using char::from_u32(codepoint).
It was created using fltk-rs"#;
