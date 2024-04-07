#![forbid(unsafe_code)]

mod components;

use fltk::{
    app,
    enums::{Event, Font},
};

#[derive(Clone, Copy)]
pub enum Message {
    Quit(bool),
    Open(bool),
    Prev,
    Next,
    Size,
    Remove,
}

fn main() {
    let (sender, receiver) = app::channel::<Message>();
    let mut page = components::Page::build(sender);
    sender.send(Message::Open(false));
    app::set_font(Font::Courier);
    while app::App::default().with_scheme(app::Scheme::Plastic).wait() {
        match receiver.recv() {
            Some(Message::Open(ui)) => page.open(ui),
            Some(Message::Size) => page.set(),
            Some(Message::Next) => page.next(),
            Some(Message::Prev) => page.prev(),
            Some(Message::Remove) => page.rem(),
            Some(Message::Quit(force)) => {
                if force || app::event() == Event::Close {
                    app::quit();
                }
            }
            None => {}
        };
    }
}
