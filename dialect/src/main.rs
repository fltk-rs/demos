#![forbid(unsafe_code)]

mod model;

use {
    fltk::{
        app,
        button::Button,
        dialog::{alert_default, FileChooser, FileChooserType},
        enums::{Color, Cursor, Event, Font, FrameType, Shortcut},
        frame::Frame,
        group::{Flex, FlexType, Wizard},
        image::SvgImage,
        menu::{Choice, MenuButton, MenuButtonType, MenuFlag},
        misc::HelpView,
        prelude::*,
        text::{TextBuffer, TextDisplay, TextEditor, WrapMode},
        valuator::{Counter, CounterType},
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
    model::Model,
    std::{cell::RefCell, env, rc::Rc, thread},
};

const HEARTBEAT: Event = Event::from_i32(404);
const SPINNER: Event = Event::from_i32(405);
const SWITCH: Event = Event::from_i32(406);
const CLICK: Event = Event::from_i32(408);
const HOME: Event = Event::from_i32(409);
const OPEN: Event = Event::from_i32(410);
const SAVE: Event = Event::from_i32(411);
const INFO: Event = Event::from_i32(412);
const SETT: Event = Event::from_i32(413);
const NAME: &str = "FlDialect";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = 105;

fn main() -> Result<(), FltkError> {
    let app = app::App::default().with_scheme(app::AppScheme::Base);
    crate::window();
    ColorTheme::new(color_themes::DARK_THEME).apply();
    app::handle_main(HEARTBEAT).unwrap();
    app::set_font(Font::Courier);
    app.run()
}

fn window() {
    const WIDTH: i32 = 360;
    const HEIGHT: i32 = 640;
    let mut element = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label(NAME)
        .center_screen();
    element.make_resizable(true);
    element.set_xclass(NAME);
    element.set_icon(Some(
        SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap(),
    ));
    let state = Rc::from(RefCell::from(Model::default()));
    crate::view(state.clone());
    element.handle(move |window, event| {
        if event == HEARTBEAT {
            window.set_label(&format!(
                "Translate from {} to {} - {NAME}",
                state.borrow().lang[state.borrow().from as usize]
                    .get("name")
                    .unwrap(),
                state.borrow().lang[state.borrow().to as usize]
                    .get("name")
                    .unwrap()
            ));
            false
        } else if app::event() == Event::Close {
            state.borrow().save();
            app::quit();
            true
        } else {
            false
        }
    });
    element.end();
    element.show();
}

fn view(state: Rc<RefCell<Model>>) {
    let mut wizard = Wizard::default_fill();
    {
        let mut page = Flex::default_fill().column();
        {
            let mut header = Flex::default();
            {
                header.fixed(&crate::menu(), HEIGHT);
                Frame::default();
                let lang = state
                    .borrow()
                    .lang
                    .iter()
                    .map(|x| x["name"].clone())
                    .collect::<Vec<String>>()
                    .join("|");
                header.fixed(&crate::choice("From", &lang, state.clone()), WIDTH);
                crate::button("Switch", &mut header)
                    .with_label("@#refresh")
                    .set_callback(move |_| {
                        app::handle_main(crate::SWITCH).unwrap();
                    });
                header.fixed(&crate::choice("To", &lang, state.clone()), WIDTH);
                Frame::default();
                crate::button("Translate", &mut header)
                    .with_label("@#circle")
                    .set_callback(move |_| {
                        app::handle_main(crate::CLICK).unwrap();
                    });
            }
            header.end();
            header.set_pad(PAD);
            page.fixed(&header, HEIGHT);
            let mut hero = Flex::default_fill();
            {
                let mut flex = hero.clone();
                crate::texteditor("Source", state.clone());
                Frame::default().handle(move |frame, event| match event {
                    Event::Push => true,
                    Event::Drag => {
                        let child = flex.child(0).unwrap();
                        match flex.get_type() {
                            FlexType::Column => {
                                if (flex.y()..=flex.height() + flex.y() - frame.height())
                                    .contains(&app::event_y())
                                {
                                    flex.fixed(&child, app::event_y() - flex.y());
                                }
                            }
                            FlexType::Row => {
                                if (flex.x()..=flex.width() + flex.x() - frame.width())
                                    .contains(&app::event_x())
                                {
                                    flex.fixed(&child, app::event_x() - flex.x());
                                }
                            }
                        }
                        app::redraw();
                        true
                    }
                    Event::Enter => {
                        frame.window().unwrap().set_cursor(match flex.get_type() {
                            FlexType::Column => Cursor::NS,
                            FlexType::Row => Cursor::WE,
                        });
                        true
                    }
                    Event::Leave => {
                        frame.window().unwrap().set_cursor(Cursor::Arrow);
                        true
                    }
                    _ => false,
                });
                crate::textdisplay("Target", state.clone());
            }
            hero.end();
            hero.set_pad(0);
            hero.handle(crate::resize);
        }
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.set_frame(FrameType::FlatBox);
        let mut page = Flex::default_fill();
        {
            crate::info();
        }
        page.end();
        page.set_margin(PAD);
        page.handle(crate::back);
        let mut page = Flex::default_fill();
        {
            Frame::default();
            let mut right = Flex::default_fill().column();
            {
                right.fixed(
                    &crate::choice("Font", &app::fonts().join("|"), state.clone())
                        .with_label("Font"),
                    HEIGHT,
                );
                right.fixed(
                    &crate::counter("Size", state.clone()).with_label("Size"),
                    HEIGHT,
                );
            }
            right.end();
            right.set_pad(PAD);
        }
        page.end();
        page.set_margin(PAD);
        page.handle(crate::back);
    }
    wizard.end();
    wizard.handle(move |wizard, event| match event {
        HOME => {
            wizard.set_current_widget(&wizard.child(0).unwrap());
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        INFO => {
            wizard.set_current_widget(&wizard.child(1).unwrap());
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        SETT => {
            wizard.set_current_widget(&wizard.child(2).unwrap());
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        SWITCH => {
            state.borrow_mut().switch();
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        CLICK => {
            let clone = state.borrow().clone();
            if clone.from != clone.to && !clone.source.is_empty() {
                let handler = thread::spawn(move || -> String { clone.click() });
                while !handler.is_finished() {
                    app::wait();
                    app::handle_main(SPINNER).unwrap();
                    app::sleep(0.02);
                }
                if let Ok(text) = handler.join() {
                    state.borrow_mut().target = text;
                };
            };
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        OPEN => {
            let mut dialog = FileChooser::new(
                env::var("HOME").unwrap(),
                "*.{txt,md}",
                FileChooserType::Single,
                "Open ...",
            );
            dialog.show();
            while dialog.shown() {
                app::wait();
            }
            if dialog.count() > 0 {
                if let Some(file) = dialog.value(1) {
                    state.borrow_mut().open(&file);
                };
            };
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        SAVE => {
            if !state.borrow().target.is_empty() {
                let mut dialog = FileChooser::new(
                    std::env::var("HOME").unwrap(),
                    "*.{txt,md}",
                    FileChooserType::Create,
                    "Save ...",
                );
                dialog.show();
                while dialog.shown() {
                    app::wait();
                }
                if dialog.count() > 0 {
                    if let Some(file) = dialog.value(1) {
                        state.borrow_mut().target(&file)
                    };
                };
            } else {
                alert_default("Target is empty.");
            };
            app::handle_main(HEARTBEAT).unwrap();
            true
        }
        _ => false,
    });
}

fn back(flex: &mut Flex, event: Event) -> bool {
    match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                let mut menu = MenuButton::default().with_type(MenuButtonType::Popup3);
                menu.add(
                    "@#<-  &Back",
                    Shortcut::Ctrl | 'b',
                    MenuFlag::Normal,
                    move |_| {
                        app::handle_main(crate::HOME).unwrap();
                    },
                );
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
    }
}

fn button(tooltip: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default();
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT / 2);
    flex.fixed(&element, HEIGHT);
    element
}

fn counter(tooltip: &str, state: Rc<RefCell<Model>>) -> Counter {
    let mut element = Counter::default().with_type(CounterType::Simple);
    element.set_tooltip(tooltip);
    element.set_range(14_f64, 22_f64);
    element.set_precision(0);
    element.handle(move |counter, event| match event {
        HEARTBEAT => {
            counter.set_value(state.borrow().size as f64);
            false
        }
        Event::Push => {
            state.borrow_mut().size = counter.value() as i32;
            app::handle_main(crate::HEARTBEAT).unwrap();
            true
        }
        _ => false,
    });
    element
}

fn choice(tooltip: &str, choice: &str, state: Rc<RefCell<Model>>) -> Choice {
    let mut element = Choice::default();
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.handle(move |choice, event| match event {
        HEARTBEAT => {
            match choice.tooltip().unwrap().as_str() {
                "From" => choice.set_value(state.borrow_mut().from),
                "To" => choice.set_value(state.borrow_mut().to),
                _ => choice.set_value(state.borrow_mut().font),
            };
            false
        }
        Event::Push => {
            match choice.tooltip().unwrap().as_str() {
                "From" => state.borrow_mut().from = choice.value(),
                "To" => state.borrow_mut().to = choice.value(),
                _ => state.borrow_mut().font = choice.value(),
            }
            app::handle_main(crate::HEARTBEAT).unwrap();
            false
        }
        _ => false,
    });
    element
}

fn texteditor(tooltip: &str, state: Rc<RefCell<Model>>) -> TextEditor {
    let mut element = TextEditor::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element.handle(move |editor, event| match event {
        HEARTBEAT => {
            editor.set_text_font(Font::by_index(state.borrow().font as usize));
            editor.buffer().unwrap().set_text(&state.borrow().source);
            editor.set_text_size(state.borrow().size);
            editor.set_linenumber_size(state.borrow().size);
            false
        }
        _ => false,
    });
    element
}

fn textdisplay(tooltip: &str, state: Rc<RefCell<Model>>) {
    let mut element = TextDisplay::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element.handle(move |display, event| match event {
        HEARTBEAT => {
            display.set_text_font(Font::by_index(state.borrow().font as usize));
            display.buffer().unwrap().set_text(&state.borrow().target);
            display.set_text_size(state.borrow().size);
            display.set_linenumber_size(state.borrow().size);
            false
        }
        SPINNER => {
            display.insert("#");
            true
        }
        _ => false,
    });
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default();
    element.set_tooltip("Menu");
    element.add(
        "@#fileopen  &Open...",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::OPEN).unwrap();
        },
    );
    element.add(
        "@#filesaveas  &Save as...",
        Shortcut::Ctrl | 's',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::SAVE).unwrap();
        },
    );
    element.add(
        "@#circle  T&ranslate",
        Shortcut::Ctrl | 'r',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::CLICK).unwrap();
        },
    );
    element.add(
        "@#search  &Info",
        Shortcut::Ctrl | 'i',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::INFO).unwrap();
        },
    );
    element.add(
        "@#menu  Se&ttings",
        Shortcut::Ctrl | 't',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(crate::SETT).unwrap();
        },
    );
    let ord: i32 = element.add(
        "@#1+  &Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        move |_| {
            app::handle_main(Event::Close).unwrap();
        },
    );
    element.at(ord).unwrap().set_label_color(Color::Red);
    element
}

fn info() {
    let (r, g, b) = Color::from_hex(0x2aa198).to_rgb();
    app::set_color(Color::Blue, r, g, b);
    let mut help = HelpView::default();
    help.set_value(include_str!("../README.md"));
    help.set_text_size(16);
}

fn resize(flex: &mut Flex, event: Event) -> bool {
    if event == Event::Resize {
        flex.set_type(match flex.width() < flex.height() {
            true => FlexType::Column,
            false => FlexType::Row,
        });
        flex.fixed(&flex.child(0).unwrap(), 0);
        flex.fixed(&flex.child(1).unwrap(), PAD);
        true
    } else {
        false
    }
}
