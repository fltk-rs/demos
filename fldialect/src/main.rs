#![forbid(unsafe_code)]

mod commands;
mod components;
mod constants;
mod elements;

use {
    components::{Footer, Header, Hero},
    constants::{Message, CONFIG, DEFAULT, NAME, SPACE, SVG, U8},
    fltk::{
        app,
        app::WidgetId,
        enums::Event,
        group::Flex,
        image::SvgImage,
        prelude::{
            ButtonExt, DisplayExt, GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt,
        },
        window::Window,
    },
    std::{env, fs, path::Path, thread, time::Duration},
};

fn main() {
    if commands::once() {
        app();
    }
}

pub fn app() {
    let file = env::var("HOME").unwrap() + CONFIG + NAME;
    let params: Vec<u8> = if Path::new(&file).exists() {
        if let Ok(value) = fs::read(&file) {
            if value.len() == DEFAULT.len() {
                value
            } else {
                fs::remove_file(&file).unwrap();
                Vec::from(DEFAULT)
            }
        } else {
            Vec::from(DEFAULT)
        }
    } else {
        Vec::from(DEFAULT)
    };
    let (sender, receiver) = app::channel::<Message>();
    let mut window = Window::default()
        .with_size(
            params[0] as i32 * U8 + params[1] as i32,
            params[2] as i32 * U8 + params[3] as i32,
        )
        .center_screen();
    let mut flex = Flex::default_fill().column().with_id("flex");
    let mut header = Header::build(sender.clone(), &mut flex, params[4], params[5], params[6]);
    let mut hero = Hero::build();
    let mut footer = Footer::build(sender.clone(), &mut flex, params[7], params[8]);
    flex.end();
    flex.set_margin(SPACE);
    flex.set_pad(SPACE);
    window.end();
    window.set_xclass(NAME);
    window.make_resizable(true);
    window.show();
    window.set_icon(Some(SvgImage::from_data(SVG).unwrap()));
    window.emit(sender.clone(), Message::Quit(false));
    let sender_clone = sender.clone();
    window.handle(move |_window, event| match event {
        Event::Resize => {
            sender_clone.send(Message::Resize);
            true
        }
        _ => false,
    });
    header.theme();
    header.rename();
    while app::App::default().wait() {
        match receiver.recv() {
            Some(Message::Switch) => header.switch(),
            Some(Message::Themes) => header.theme(),
            Some(Message::Rename) => header.rename(),
            Some(Message::Info) => hero.info(),
            Some(Message::Open) => hero.open(),
            Some(Message::Save) => hero.save(),
            Some(Message::Resize) => hero.resize(),
            Some(Message::Hide) => footer.hide(),
            Some(Message::Tick) => footer.dial.do_callback(),
            Some(Message::Request) => {
                let from = header.from.choice().unwrap();
                let to = header.to.choice().unwrap();
                let source = hero.from.buffer().unwrap().text();
                if from != to && !source.is_empty() {
                    footer.trans.deactivate();
                    let voice = header.voice.value();
                    let handler =
                        thread::spawn(move || -> String { commands::run(voice, from, to, source) });
                    thread::spawn({
                        let sender = sender.clone();
                        move || {
                            while !handler.is_finished() {
                                thread::sleep(Duration::from_millis(2));
                                sender.send(Message::Tick);
                            }
                            if let Ok(msg) = handler.join() {
                                sender.send(Message::Responce(msg));
                            };
                        }
                    });
                };
            }
            Some(Message::Responce(msg)) => {
                hero.to.buffer().unwrap().set_text(&msg);
                footer.trans.activate();
            }
            Some(Message::Reload(params)) => {
                window.un_maximize();
                window.set_size(
                    params[0] as i32 * U8 + params[1] as i32,
                    params[2] as i32 * U8 + params[3] as i32,
                );
                header.from.set_value(params[5] as i32);
                header.to.set_value(params[6] as i32);
                footer.font.set_value(params[7] as i32);
                footer.size.set_value(params[8] as f64);
                flex.redraw();
            }
            Some(Message::Quit(force)) => {
                if force || app::event() == Event::Close {
                    fs::write(
                        &file,
                        [
                            (window.width() / U8) as u8,
                            (window.width() % U8) as u8,
                            (window.height() / U8) as u8,
                            (window.height() % U8) as u8,
                            header.menu.at(1).unwrap().value() as u8,
                            header.from.value() as u8,
                            header.to.value() as u8,
                            footer.font.value() as u8,
                            footer.size.value() as u8,
                        ],
                    )
                    .unwrap();
                    app::quit();
                }
            }
            None => {}
        };
    }
}
