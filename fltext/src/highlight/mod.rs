use fltk::{
    app,
    enums::{Color, Font},
    prelude::DisplayExt,
    text::{StyleTableEntry, TextBuffer, TextEditor},
};
use std::path::Path;
use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::HighlightEvent;
use tree_sitter_highlight::Highlighter;

mod colors;
mod md;
mod rust;
mod toml;

fn translate_style(idx: usize) -> char {
    char::from_u32(65 + idx as u32).unwrap()
}

fn resolve_styles(v: &[(&'static str, u32)]) -> (Vec<&'static str>, Vec<StyleTableEntry>) {
    let mut names = Vec::new();
    let mut styles = Vec::new();
    for elem in v {
        names.push(elem.0);
        styles.push(StyleTableEntry {
            color: Color::from_hex(elem.1),
            font: Font::Courier,
            size: app::font_size(),
        });
    }
    (names, styles)
}

pub struct HighlightData {
    styles: Vec<StyleTableEntry>,
    config: HighlightConfiguration,
    exeption_fn: Option<fn(usize, &str) -> char>,
}

impl HighlightData {
    pub fn new(
        styles: Vec<StyleTableEntry>,
        config: HighlightConfiguration,
        exeption_fn: Option<fn(usize, &str) -> char>,
    ) -> Self {
        Self {
            styles,
            config,
            exeption_fn,
        }
    }
}

fn get_highlight(p: &Path) -> Option<HighlightData> {
    if let Some(ext) = p.extension() {
        match ext.to_str().unwrap() {
            "rs" => Some(rust::lang_data()),
            "toml" => Some(toml::lang_data()),
            "md" => Some(md::lang_data()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn highlight(p: &Path, ed: &mut TextEditor, buf: &mut TextBuffer) {
    if let Some(HighlightData {
        styles,
        config,
        exeption_fn,
    }) = get_highlight(p)
    {
        let mut highlighter = Highlighter::new();
        let mut sbuf = TextBuffer::default();
        ed.set_highlight_data(sbuf.clone(), styles);
        apply(
            &mut highlighter,
            &config,
            &buf.text(),
            &mut sbuf,
            &exeption_fn,
        );
        buf.add_modify_callback({
            let buf = buf.clone();
            move |_, _, _, _, _| {
                apply(
                    &mut highlighter,
                    &config,
                    &buf.text(),
                    &mut sbuf,
                    &exeption_fn,
                );
            }
        });
    }
}

fn apply(
    highlighter: &mut Highlighter,
    config: &HighlightConfiguration,
    s: &str,
    sbuf: &mut TextBuffer,
    exeption_fn: &Option<fn(usize, &str) -> char>,
) {
    let highlights = highlighter
        .highlight(config, s.as_bytes(), None, |_| None)
        .unwrap();

    let mut local_buf = "A".repeat(s.len());
    let mut curr = 0;
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::HighlightStart(s) => {
                curr = s.0;
            }
            HighlightEvent::Source { start, end } => {
                let c = if let Some(f) = exeption_fn {
                    f(curr, &s[start..end])
                } else {
                    translate_style(curr)
                };
                local_buf.replace_range(start..end, &c.to_string().repeat(end - start));
            }
            HighlightEvent::HighlightEnd => curr = 0,
        }
    }
    sbuf.set_text(&local_buf);
}
