#![forbid(unsafe_code)]

mod components;

use {
    crate::components::Page,
    fltk::{
        app,
        enums::{Event, Font},
    },
};

#[derive(Clone, Copy)]
pub enum Message {
    Quit(bool),
    Plus,
    Stop,
    Next,
    Prev,
    Open,
    Play,
    Volume,
    Remove,
    Select,
}

fn main() {
    let (sender, receiver) = app::channel::<Message>();
    let mut page = Page::build(sender);
    app::set_font(Font::Courier);
    while app::App::default().with_scheme(app::Scheme::Plastic).wait() {
        match receiver.recv() {
            Some(Message::Select) => {
                page.menu.popup();
            }
            Some(Message::Remove) => page.remove(),
            Some(Message::Plus) => page.plus(),
            Some(Message::Open) => page.open(),
            Some(Message::Play) => page.play(),
            Some(Message::Stop) => page.stop(),
            Some(Message::Next) => page.next(),
            Some(Message::Prev) => page.prev(),
            Some(Message::Volume) => page.volume(),
            Some(Message::Quit(force)) => {
                if force || app::event() == Event::Close {
                    page.player.stop_all();
                    app::quit();
                }
            }
            None => {}
        };
    }
}
