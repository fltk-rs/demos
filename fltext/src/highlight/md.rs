use super::colors::*;
use super::HighlightData;
use tree_sitter_highlight::HighlightConfiguration;

use tree_sitter_md as ts;

pub const STYLES: &[(&str, u32)] = &[
    ("DEFAULT", WHITE),
    ("text.title", RED),
    ("text.reference", GREY),
    ("punctuation.special", RED),
    ("text.literal", GREEN),
    ("punctuation.delimiter", DARKYELLOW),
    ("text.uri", DARKYELLOW),
];

pub fn lang_data() -> HighlightData {
    let (names, styles) = super::resolve_styles(STYLES);
    let mut config =
        HighlightConfiguration::new(ts::language(), ts::HIGHLIGHT_QUERY_BLOCK, "", "").unwrap();
    config.configure(&names);
    HighlightData::new(styles, config, None)
}
