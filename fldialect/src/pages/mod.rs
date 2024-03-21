use {
    crate::{
        commands,
        components::{Footer, Header, Hero},
        constants::{Message, APPNAME, CFG, PARAMS, SVG, THEMES, U8, WIDGET_HEIGHT, WIDGET_SPACE},
    },
    fltk::{
        app,
        enums::{Event, Shortcut},
        group::Flex,
        image::SvgImage,
        menu::MenuFlag,
        prelude::{DisplayExt, GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt},
        window::Window,
    },
    std::{env, fs, path::Path, thread, time::Duration},
};

pub fn main() {
    let file = env::var("HOME").unwrap() + CFG;
    let params: Vec<u8> = match Path::new(&file).exists() {
        true => fs::read(&file).unwrap(),
        false => Vec::from(PARAMS),
    };
    let mut window = Window::default()
        .with_size(
            params[1] as i32 * U8 + params[2] as i32,
            params[3] as i32 * U8 + params[4] as i32,
        )
        .with_label(APPNAME)
        .center_screen();
    let mut flex = Flex::default_fill().column();
    let mut header = Header::build(&mut flex, params[5] as i32, params[6] as i32);
    let mut hero = Hero::build(params[0]);
    let mut footer = Footer::build(&mut flex, params[7] as i32, params[8] as i32);
    flex.end();
    window.end();
    flex.set_margin(WIDGET_SPACE);
    flex.set_pad(WIDGET_SPACE);
    window.make_resizable(true);
    window.show();
    window.set_icon(Some(SvgImage::from_data(SVG).unwrap()));
    run(
        &mut window,
        &mut flex,
        &mut header,
        &mut hero,
        &mut footer,
        &file,
    );
}

fn run(
    window: &mut Window,
    flex: &mut Flex,
    header: &mut Header,
    hero: &mut Hero,
    footer: &mut Footer,
    file: &str,
) {
    let (sender, receiver) = app::channel::<Message>();
    for (ord, item) in THEMES.iter().enumerate() {
        let idx = header.menu.add_emit(
            &format!("&Theme/&{}\t", item),
            Shortcut::Ctrl | item.to_lowercase().chars().next().unwrap(),
            MenuFlag::Radio,
            sender.clone(),
            Message::Themes(ord as u8),
        );
        if ord == hero.theme as usize {
            header.menu.at(idx).unwrap().set();
        };
    }
    let idx: i32 = header.menu.add_emit(
        "&View/&Footer\t",
        Shortcut::None,
        MenuFlag::Toggle,
        sender.clone(),
        Message::Hide,
    );
    header.menu.at(idx).unwrap().set();
    for (label, chr, msg) in [
        ("&File/@#fileopen  &Open", 'o', Message::Open),
        ("&File/@#filesaveas  &Save", 's', Message::Save),
        ("@#circle  T&ranslate", 'r', Message::Request),
        ("@#refresh  Swi&tch", 't', Message::Switch),
        ("@#reload  &Reload", 's', Message::Reload(PARAMS)),
        ("@#search  &Info", 'i', Message::Info),
        ("@#1+  &Quit", 'q', Message::Quit(true)),
    ] {
        header.menu.add_emit(
            &format!("{label}\t"),
            Shortcut::Ctrl | chr,
            MenuFlag::Normal,
            sender.clone(),
            msg,
        );
    }
    footer.open.emit(sender.clone(), Message::Open);
    footer.save.emit(sender.clone(), Message::Save);
    footer.trans.emit(sender.clone(), Message::Request);
    footer.size.emit(sender.clone(), Message::Size);
    footer.font.emit(sender.clone(), Message::Font);
    header.switch.emit(sender.clone(), Message::Switch);
    header.from.emit(sender.clone(), Message::Request);
    header.to.emit(sender.clone(), Message::Request);
    window.emit(sender.clone(), Message::Quit(false));
    let sender_clone = sender.clone();
    window.handle(move |_window, event| match event {
        Event::Resize => {
            sender_clone.send(Message::Resize);
            true
        }
        _ => false,
    });
    while app::App::default().wait() {
        match receiver.recv() {
            Some(Message::Open) => hero.open(),
            Some(Message::Save) => hero.save(),
            Some(Message::Font) => footer.fonts(),
            Some(Message::Hide) => {
                match header.menu.at(idx).unwrap().value() {
                    true => {
                        flex.fixed(&footer.layout, WIDGET_HEIGHT);
                        footer.layout.show();
                    }
                    false => {
                        footer.layout.hide();
                        flex.fixed(&footer.layout, 0);
                    }
                };
                app::redraw();
            }
            Some(Message::Reload(params)) => {
                sender.send(Message::Themes(params[0]));
                window.un_maximize();
                window.set_size(
                    params[1] as i32 * U8 + params[2] as i32,
                    params[3] as i32 * U8 + params[4] as i32,
                );
                header.from.set_value(params[5] as i32);
                header.to.set_value(params[6] as i32);
                footer.font.set_value(params[7] as i32);
                footer.size.set_value(params[8] as f64);
            }
            Some(Message::Request) => {
                let source = hero.from.buffer().unwrap().text();
                let from = header.from.choice().unwrap();
                let to = header.to.choice().unwrap();
                if !source.is_empty() && from != to {
                    let close = header.close.is_toggled();
                    let sender_clone = sender.clone();
                    let handler = thread::spawn(move || {
                        let answer = commands::run(close, from, to, source);
                        sender_clone.send(Message::Responce(answer));
                    });
                    let sender_clone = sender.clone();
                    thread::spawn(move || {
                        while !handler.is_finished() {
                            let sender_clone = sender_clone.clone();
                            thread::sleep(Duration::from_millis(60));
                            sender_clone.send(Message::Tick);
                        }
                    });
                };
            }
            Some(Message::Responce(msg)) => hero.to.buffer().unwrap().set_text(&msg),
            Some(Message::Switch) => header.switch(),
            Some(Message::Resize) => hero.resize(),
            Some(Message::Tick) => header.tick(),
            Some(Message::Size) => {
                hero.from.set_text_size(footer.size.value() as i32);
                hero.to.set_text_size(footer.size.value() as i32);
                app::redraw();
            }
            Some(Message::Info) => hero.info(),
            Some(Message::Themes(ord)) => hero.theme(ord),
            Some(Message::Quit(force)) => {
                if force || app::event() == Event::Close {
                    let width = window.width();
                    let height = window.height();
                    fs::write(
                        file,
                        [
                            hero.theme,
                            (width / U8) as u8,
                            (width % U8) as u8,
                            (height / U8) as u8,
                            (height % U8) as u8,
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
