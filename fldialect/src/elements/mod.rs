use {
    crate::constants::{TEXT_SIZE, WIDGET_HEIGHT, WIDGET_WIDTH},
    fltk::{
        button::{Button, ToggleButton},
        group::Flex,
        menu::Choice,
        prelude::{DisplayExt, MenuExt, WidgetExt},
        text::{TextBuffer, TextEditor, WrapMode},
    },
};

pub fn button(label: &str, tooltip: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    element.set_label_size(TEXT_SIZE);
    flex.fixed(&element, WIDGET_HEIGHT);
    element
}

pub fn toggle_button(label: &str, tooltip: &str, flex: &mut Flex) -> ToggleButton {
    let mut element = ToggleButton::default().with_label(label);
    element.set_tooltip(tooltip);
    element.set_label_size(TEXT_SIZE);
    flex.fixed(&element, WIDGET_HEIGHT);
    element
}

pub fn choice(tooltip: &str, choice: &str, value: i32, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value);
    flex.fixed(&element, WIDGET_WIDTH);
    element
}

pub fn text(tooltip: &str) -> TextEditor {
    let mut element = TextEditor::default();
    element.set_tooltip(tooltip);
    element.set_text_size(TEXT_SIZE);
    element.set_linenumber_size(TEXT_SIZE);
    element.set_linenumber_width(WIDGET_HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element
}
