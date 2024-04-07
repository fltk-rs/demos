use super::colors::*;
use super::HighlightData;
use tree_sitter_highlight::HighlightConfiguration;

use tree_sitter_rust as ts;

pub const STYLES: &[(&str, u32)] = &[
    ("DEFAULT", WHITE),
    ("attribute", RED),
    ("constructor", DARKYELLOW),
    ("comment", GREY),
    ("constant", DARKYELLOW),
    ("constant.builtin", DARKYELLOW),
    ("function", BLUE),
    ("function.method", BLUE),
    ("keyword", PURPLE),
    ("operator", WHITE),
    ("property", RED),
    ("punctuation.bracket", DARKYELLOW),
    ("punctuation.delimiter", WHITE),
    ("string", GREEN),
    ("type", YELLOW),
    ("type.builtin", YELLOW),
    ("variable", RED),
    ("variable.builtin", RED),
    ("variable.parameter", WHITE),
    ("label", WHITE),
];

pub fn lang_data() -> HighlightData {
    let (names, styles) = super::resolve_styles(STYLES);
    let mut config =
        HighlightConfiguration::new(ts::language(), ts::HIGHLIGHT_QUERY, "", "").unwrap();
    config.configure(&names);
    HighlightData::new(styles, config, None)
}
