use {
    crate::constants::{HEIGHT, WIDTH},
    fltk::{
        button::Button,
        group::Flex,
        menu::Choice,
        prelude::{DisplayExt, MenuExt, WidgetExt},
        text::{TextBuffer, TextEditor, WrapMode},
    },
};

pub fn button(label: &str, tooltip: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT / 2);
    flex.fixed(&element, HEIGHT);
    element
}

pub fn choice(tooltip: &str, choice: &str, value: u8, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value as i32);
    flex.fixed(&element, WIDTH);
    element
}

pub fn text(tooltip: &str) -> TextEditor {
    let mut element = TextEditor::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element
}
