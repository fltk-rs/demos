#![forbid(unsafe_code)]

mod model;

use {
    fltk::{
        app,
        button::Button,
        dialog::HelpDialog,
        enums::{Align, CallbackTrigger, Color, Cursor, Event, Font, FrameType, Key, Shortcut},
        frame::Frame,
        group::Flex,
        image::SvgImage,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        window::Window,
    },
    model::Model,
    std::{cell::RefCell, rc::Rc},
};

const NAME: &str = "FlCalculator";
const HEARTBEAT: Event = Event::from_i32(404);
const THEME: Event = Event::from_i32(405);
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const EQUAL: &str = "=";
const COLORS: [[Color; 6]; 2] = [
    [
        Color::from_hex(0xfdf6e3),
        Color::from_hex(0x586e75),
        Color::from_hex(0xb58900),
        Color::from_hex(0xeee8d5),
        Color::from_hex(0xcb4b16),
        Color::from_hex(0xdc322f),
    ],
    [
        Color::from_hex(0x002b36),
        Color::from_hex(0x93a1a1),
        Color::from_hex(0x268bd2),
        Color::from_hex(0x073642),
        Color::from_hex(0x6c71c4),
        Color::from_hex(0xd33682),
    ],
];

fn main() -> Result<(), FltkError> {
    let app = app::App::default().with_scheme(app::AppScheme::Base);
    crate::window();
    app::handle_main(HEARTBEAT).unwrap();
    app::set_font(Font::Courier);
    app.run()
}

fn window() {
    let mut element = Window::default()
        .with_size(360, 640)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(false);
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    let state = Rc::from(RefCell::from(Model::default()));
    crate::view(state.clone());
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            state.borrow_mut().save();
            app::quit();
        }
    });
    element.end();
    element.show();
}

fn view(state: Rc<RefCell<Model>>) {
    let menu = crate::menu(state.clone());
    let mut page = Flex::default_fill().column();
    crate::display("Output", state.clone());
    let mut row = Flex::default();
    crate::output("Operation", state.clone());
    let mut col = Flex::default().column();
    crate::output("Previous", state.clone());
    crate::output("Current", state.clone());
    col.end();
    col.set_pad(0);
    row.end();
    row.set_pad(0);
    row.set_margin(0);
    row.fixed(&row.child(0).unwrap(), 30);
    let mut buttons = Flex::default_fill().column();
    for line in [
        ["CE", "C", "%", "/"],
        ["7", "8", "9", "x"],
        ["4", "5", "6", "-"],
        ["1", "2", "3", "+"],
        ["0", ".", "@<-", crate::EQUAL],
    ] {
        let mut row = Flex::default();
        for label in line {
            crate::button(label, state.clone());
        }
        row.end();
        row.set_pad(PAD);
        row.set_margin(0);
    }
    buttons.handle(move |flex, event| match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                menu.popup();
                true
            }
            _ => false,
        },
        Event::Enter => {
            flex.window().unwrap().set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            flex.window().unwrap().set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    });
    buttons.end();
    buttons.set_pad(PAD);
    buttons.set_margin(0);
    page.fixed(&buttons, 425);
    page.end();
    page.set_margin(PAD);
    page.set_pad(PAD);
    page.set_frame(FrameType::FlatBox);
    page.fixed(&row, 60);
    page.handle(move |flex, event| match event {
        HEARTBEAT => {
            flex.set_color(crate::COLORS[state.borrow().theme as usize][0]);
            false
        }
        THEME => {
            state.borrow_mut().theme();
            app::handle_main(HEARTBEAT).unwrap();
            app::redraw();
            true
        }
        _ => false,
    });
}

fn button(title: &'static str, state: Rc<RefCell<Model>>) {
    let mut element = Button::default().with_label(title);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::OFlatFrame);
    match title {
        "@<-" => element.set_shortcut(Shortcut::None | Key::BackSpace),
        "CE" => element.set_shortcut(Shortcut::None | Key::Delete),
        crate::EQUAL => element.set_shortcut(Shortcut::None | Key::Enter),
        "x" => element.set_shortcut(Shortcut::None | '*'),
        _ => element.set_shortcut(Shortcut::None | title.chars().next().unwrap()),
    }
    element.handle(move |button, event| match event {
        Event::Push => {
            let value = button.label();
            state.borrow_mut().click(&value);
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        HEARTBEAT => {
            let theme = state.borrow().theme;
            match button.label().as_str() {
                "C" | "x" | "/" | "+" | "-" | "%" => {
                    button.set_color(crate::COLORS[theme as usize][2]);
                    button.set_label_color(crate::COLORS[theme as usize][0]);
                }
                "CE" => {
                    button.set_color(crate::COLORS[theme as usize][4]);
                    button.set_label_color(crate::COLORS[theme as usize][0]);
                }
                crate::EQUAL => {
                    button.set_color(crate::COLORS[theme as usize][5]);
                    button.set_label_color(crate::COLORS[theme as usize][0]);
                }
                _ => {
                    button.set_color(crate::COLORS[theme as usize][3]);
                    button.set_label_color(crate::COLORS[theme as usize][1]);
                }
            };
            false
        }
        _ => false,
    });
}

fn display(tooltip: &str, state: Rc<RefCell<Model>>) {
    let mut element = TextDisplay::default();
    element.set_text_size(HEIGHT - 5);
    element.set_tooltip(tooltip);
    element.set_scrollbar_size(3);
    element.set_frame(FrameType::FlatBox);
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_buffer(TextBuffer::default());
    element.set_trigger(CallbackTrigger::Changed);
    element.handle(move |display, event| {
        if event == HEARTBEAT {
            let (theme, value) = (state.borrow().theme, state.borrow().output.clone());
            display.set_color(crate::COLORS[theme as usize][0]);
            display.set_text_color(crate::COLORS[theme as usize][1]);
            display.buffer().unwrap().set_text(&value);
            display.scroll(
                display.buffer().unwrap().text().split_whitespace().count() as i32,
                0,
            );
        };
        false
    });
}

fn output(tooltip: &str, state: Rc<RefCell<Model>>) {
    let mut element = Frame::default().with_align(Align::Right | Align::Inside);
    element.set_label_size(HEIGHT);
    element.set_tooltip(tooltip);
    element.set_frame(FrameType::FlatBox);
    element.handle(move |frame, event| {
        if event == HEARTBEAT {
            let (theme, value) = (
                state.borrow().theme,
                match frame.tooltip().unwrap().as_str() {
                    "Operation" => &state.borrow().operation,
                    "Previous" => &state.borrow().prev.to_string(),
                    _ => &state.borrow().current.to_string(),
                },
            );
            frame.set_color(crate::COLORS[theme as usize][0]);
            frame.set_label_color(crate::COLORS[theme as usize][1]);
            frame.set_label(value);
        };
        false
    });
}

fn menu(state: Rc<RefCell<Model>>) -> MenuButton {
    let mut element = MenuButton::default().with_type(MenuButtonType::Popup3);
    element.set_frame(FrameType::FlatBox);
    let mut item: i32 = element.add(
        "&Night mode\t",
        Shortcut::Ctrl | 'n',
        MenuFlag::Toggle,
        move |_| {
            app::handle_main(THEME).unwrap();
        },
    );
    match state.borrow().theme {
        true => element.at(item).unwrap().set(),
        false => element.at(item).unwrap().clear(),
    };
    element.add(
        "@#search  &Info",
        Shortcut::Ctrl | 'i',
        MenuFlag::Normal,
        crate::info,
    );
    item = element.add(
        "@#1+  &Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(Event::Close).unwrap();
        },
    );
    element.at(item).unwrap().set_label_color(Color::Red);
    element
}

fn info(_: &mut MenuButton) {
    let mut dialog = HelpDialog::default();
    dialog.set_value(include_str!("../README.md"));
    dialog.set_text_size(16);
    dialog.show();
    while dialog.shown() {
        app::wait();
    }
}
