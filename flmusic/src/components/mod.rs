use {
    crate::Message,
    fltk::{
        app,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{Event, Shortcut},
        group::Flex,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::{BrowserExt, GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt},
        valuator::{Scrollbar, ScrollbarType},
        window::Window,
    },
    soloud::{audio::Wav, AudioExt, LoadExt, Soloud},
    std::{ffi::OsStr, fs, path::Path},
};

pub struct Page {
    model: Vec<String>,
    pub player: Soloud,
    pub menu: MenuButton,
    browser: Browser,
    prev: Button,
    play: Button,
    stop: Button,
    next: Button,
    song: Scrollbar,
    vol: Scrollbar,
    rem: Button,
}

impl Page {
    pub fn build(sender: app::Sender<Message>) -> Self {
        let mut menu = MenuButton::default().with_type(MenuButtonType::Popup3);
        for (label, chr, msg) in [
            ("@#+2menu  &Remove", 'd', Message::Remove),
            ("@#+2+  &Add", 'n', Message::Plus),
        ] {
            menu.add_emit(
                &format!("{label}\t"),
                Shortcut::Ctrl | chr,
                MenuFlag::Normal,
                sender,
                msg,
            );
        }
        let mut window = Window::default()
            .with_size(640, 360)
            .with_label("FlMusic")
            .center_screen();
        let mut margin = Flex::default_fill().column();
        let mut header = Flex::default();
        let mut open = button("@#+2fileopen", "Open", &mut header);
        open.emit(sender, Message::Open);
        open.activate();
        let mut prev = button("@#+2|<", "Prev ...", &mut header);
        prev.emit(sender, Message::Prev);
        let mut play = button("@#+2>", "Play", &mut header);
        play.emit(sender, Message::Play);
        let group = Flex::default().column();
        let mut song = Scrollbar::default().with_type(ScrollbarType::HorizontalNice);
        song.deactivate();
        let mut vol = Scrollbar::default().with_type(ScrollbarType::HorizontalNice);
        vol.emit(sender, Message::Volume);
        vol.set_range(0.0, 100.0);
        vol.set_value(50.0);
        vol.set_precision(0);
        vol.deactivate();
        group.end();
        let mut stop = button("@#+1square", "Stop", &mut header);
        stop.emit(sender, Message::Stop);
        let mut next = button("@#+2>|", "Next", &mut header);
        next.emit(sender, Message::Next);
        let mut rem = button("@#+2menu", "Remove", &mut header);
        rem.emit(sender, Message::Remove);
        header.end();
        header.set_pad(0);
        let mut browser = Browser::default().with_type(BrowserType::Hold);
        margin.end();
        margin.fixed(&header, 30);
        margin.set_margin(10);
        margin.set_pad(10);
        window.end();
        window.make_resizable(true);
        window.show();
        window.emit(sender, Message::Quit(false));
        browser.handle(move |_, event| match event {
            Event::Push => match app::event_mouse_button() {
                app::MouseButton::Right => {
                    sender.send(Message::Select);
                    true
                }
                app::MouseButton::Left => true,
                _ => false,
            },
            _ => false,
        });
        Self {
            model: Vec::new(),
            player: Soloud::default().expect("Cannot access audio backend"),
            menu,
            browser,
            prev,
            play,
            stop,
            next,
            song,
            vol,
            rem,
        }
    }
    pub fn update(&mut self) {
        self.browser.clear();
        for item in &self.model {
            self.browser.add(item);
        }
        self.browser.sort();
        self.browser.select(1);
        app::redraw();
    }
    pub fn plus(&mut self) {
        let mut dialog = FileChooser::new(
            std::env::var("HOME").unwrap(),
            "*.{mp3}",
            FileChooserType::Multi,
            "Choose File...",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        if dialog.count() > 0 {
            for item in 1..=dialog.count() {
                if let Some(file) = dialog.value(item) {
                    self.model.push(file);
                    self.update();
                };
            }
            self.play.activate();
            self.vol.activate();
        };
    }
    pub fn open(&mut self) {
        let mut dialog = FileChooser::new(
            std::env::var("HOME").unwrap(),
            "",
            FileChooserType::Directory,
            "Choose File...",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        if dialog.count() > 0 {
            if let Some(file) = dialog.value(1) {
                if let Ok(entries) = fs::read_dir(file) {
                    self.model = entries
                        .map(|entry| entry.ok().unwrap().path())
                        .filter(|path| {
                            ["mp3"]
                                .map(|x| Some(OsStr::new(x)))
                                .contains(&path.extension())
                        })
                        .map(|path| format!("{}", path.display()))
                        .collect::<Vec<String>>();
                };
                self.update();
                self.play.activate();
                self.vol.activate();
                self.next.activate();
                self.prev.activate();
                self.rem.activate();
            };
        };
    }
    pub fn stop(&mut self) {
        if self.player.active_voice_count() > 0 {
            self.play.set_label("@#+2>");
            self.play.set_tooltip("Start");
            self.stop.deactivate();
            self.player.stop_all();
        }
    }
    pub fn play(&mut self) {
        if !self.model.is_empty() {
            if self.player.active_voice_count() > 0 {
                self.play.set_label("@#+2>");
                self.play.set_tooltip("Start");
                self.stop.deactivate();
                self.player.stop_all();
            } else {
                self.play.set_label("@#+2||");
                self.play.set_tooltip("Stop");
                self.stop.activate();
                let mut wav = Wav::default();
                if wav
                    .load(Path::new(&self.browser.text(self.browser.value()).unwrap()))
                    .is_ok()
                {
                    let handle = self.player.play(&wav);
                    self.player.set_pause(handle, true);
                    self.player.set_volume(handle, self.vol.value() as f32);
                    self.song.set_range(0.0, wav.length());
                    self.song.set_step(wav.length(), 20);
                    self.player.play(&wav);
                    while self.player.active_voice_count() > 0 {
                        app::wait();
                    }
                };
            }
        }
    }
    pub fn volume(&mut self) {
        eprintln!("{}", self.vol.value());
        self.player.set_global_volume(self.vol.value() as f32);
    }
    pub fn remove(&mut self) {
        match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
            Some(0) => {
                if fs::remove_file(&self.model[self.browser.value() as usize - 1]).is_ok() {
                    self.model.remove(self.browser.value() as usize - 1);
                    self.update();
                }
            }
            Some(1) => {}
            _ => {}
        };
    }
    pub fn next(&mut self) {
        match self.browser.value() < self.browser.size() {
            true => self.browser.select(self.browser.value() + 1),
            false => self.browser.select(1),
        };
        app::redraw();
    }
    pub fn prev(&mut self) {
        match self.browser.value() > 1 {
            true => self.browser.select(self.browser.value() - 1),
            false => self.browser.select(self.browser.size()),
        };
        app::redraw();
    }
}

pub fn button(label: &str, tooltip: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    element.deactivate();
    flex.fixed(&element, 30);
    element
}
