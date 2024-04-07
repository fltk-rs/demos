#![forbid(unsafe_code)]
extern crate ttf_parser;
use fltk::{enums::*, prelude::*, *};

fn main() {
    let app = app::App::default();
    dialog::message_title_default("Glyph Map");
    let mut wind = window::Window::default()
        .with_size(250, 300)
        .with_label("Glyph Map");
    wind.set_xclass("glyphmap");
    wind.make_resizable(true);
    let mut menu = menu::MenuBar::new(0, 0, 250, 30, None);
    menu.add_choice("File/Load font...|File/Quit|Help/About");
    menu.set_color(Color::Background.lighter());
    menu.set_frame(FrameType::FlatBox);
    menu.set_text_size(14);
    let mut scroll =
        group::Scroll::new(0, 30, 250, 200, None).with_type(group::ScrollType::Vertical);
    let mut scrollbar = scroll.scrollbar();
    scrollbar.set_type(valuator::ScrollbarType::VerticalNice);
    let mut pack = group::Pack::default_fill();
    pack.end();
    scroll.end();
    let mut inp = input::MultilineInput::default()
        .with_size(250, 67)
        .below_of(&scroll, 2);
    inp.set_text_size(22);
    inp.set_frame(FrameType::FlatBox);
    wind.end();
    wind.show();

    menu.set_callback(move |m| {
        if let Some(choice) = m.choice() {
            match choice.as_str() {
                "Load font..." => {
                    let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
                    dlg.set_option(dialog::FileDialogOptions::NoOptions);
                    dlg.set_filter("*.{ttf,otf}");
                    dlg.show();
                    let filename = dlg.filename();
                    if !filename.to_string_lossy().to_string().is_empty() && filename.exists() {
                        let font_data = std::fs::read(&filename).unwrap();
                        let face = ttf_parser::Face::from_slice(&font_data, 0).unwrap();
                        let f = Font::load_font(&filename).unwrap();
                        Font::set_font(Font::Zapfdingbats, &f);
                        wind.set_label(&format!("{}-Glyph Map", f));
                        scroll.scroll_to(0, 0);
                        pack.clear();
                        pack.begin();
                        let cmap = face.tables().cmap.unwrap();
                        if let Some(table) = cmap.subtables.into_iter().last() {
                            inp.set_text_font(Font::Zapfdingbats);
                            inp.redraw();
                            table.codepoints(|codepoint| {
                                let c = char::from_u32(codepoint).unwrap();
                                let txt = String::from(c);
                                let (w, h) = draw::measure(&txt, true);
                                if w != 0 && h != 0 && face.glyph_index(c).is_some() {
                                        let hpack = group::Pack::default()
                                            .with_type(group::PackType::Horizontal)
                                            .with_size(0, 50);
                                        let mut out = output::Output::default()
                                            .with_size(100, 0);
                                        out.set_value(&codepoint.to_string());
                                        out.set_frame(FrameType::FlatBox);
                                        out.set_color(Color::Background);
                                        let mut space = frame::Frame::default().with_size(50, 0);
                                        space.set_frame(FrameType::FlatBox);
                                        space.set_color(Color::White);
                                        let mut out = output::Output::default().with_size(150, 0);
                                        out.set_frame(FrameType::FlatBox);
                                        out.set_text_font(Font::Zapfdingbats);
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
                "Quit" => app::quit(),
                "About" => dialog::alert_default("This app can be used to visualize fonts and their corresponding codepoints,\nwhich can then be used in fltk-rs apps using char::from_u32(codepoint).\nIt was created using fltk-rs"),
                _ => (),
            }
        }
    });

    app::set_background_color(170, 189, 206);
    app.run().unwrap();
}
