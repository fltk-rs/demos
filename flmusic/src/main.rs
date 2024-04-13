#![forbid(unsafe_code)]

mod components;

use {
    crate::components::Page,
    fltk::{app, enums::Event},
    std::{env, fs, path::Path},
};

#[derive(Clone, Copy)]
pub enum Message {
    Quit(bool),
    Plus,
    Open,
    Remove,
}

fn main() {
    let file = env::var("HOME").unwrap() + "/.config/" + "FlMusic";
    let model: Vec<String> = if Path::new(&file).exists() {
        if let Ok(value) = fs::read(&file) {
            rmp_serde::from_slice(&value).unwrap()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    let (sender, receiver) = app::channel::<Message>();
    let mut page = Page::build(sender);
    page.add(model);
    while app::App::default().with_scheme(app::Scheme::Plastic).wait() {
        match receiver.recv() {
            Some(Message::Remove) => page.remove(),
            Some(Message::Plus) => page.plus(),
            Some(Message::Open) => page.open(),
            Some(Message::Quit(force)) => {
                if force || app::event() == Event::Close {
                    page.player.borrow().stop_all();
                    fs::write(&file, &rmp_serde::to_vec(&page.get()).unwrap()).unwrap();
                    app::quit();
                }
            }
            None => {}
        };
    }
}
