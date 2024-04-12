use {
    crate::Message,
    fltk::{
        app,
        browser::{Browser, BrowserType},
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{Color, Event, Shortcut},
        group::Flex,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        misc::Progress,
        prelude::{BrowserExt, GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt},
        valuator::{Slider, SliderType},
        window::Window,
    },
    soloud::{audio::Wav, AudioExt, LoadExt, Soloud},
    std::{cell::RefCell, ffi::OsStr, fs, path::Path, rc::Rc, thread, time::Duration},
};

pub struct Page {
    pub player: Rc<RefCell<Soloud>>,
    pub menu: MenuButton,
    pub browser: Browser,
    prev: Button,
    play: Button,
    next: Button,
    pub song: Progress,
    vol: Slider,
}

impl Page {
    pub fn build(sender: app::Sender<Message>) -> Self {
        let mut window = Window::default()
            .with_size(640, 360)
            .with_label("FlMusic")
            .center_screen();
        let mut margin = Flex::default_fill().column();
        let mut header = Flex::default();
        let mut menu = MenuButton::default().with_label("@#+");
        for (label, chr, msg) in [
            ("@#fileopen  &Open", 'o', Message::Open),
            ("@#filenew  &Add", 'a', Message::Plus),
        ] {
            menu.add_emit(
                &format!("{label}\t"),
                Shortcut::Ctrl | chr,
                MenuFlag::Normal,
                sender,
                msg,
            );
        }
        header.fixed(&menu, 50);
        let mut row = Flex::default();
        let mut prev = button("@#|<", "Prev ...", &mut header);
        let mut play = button("@#>", "Play", &mut header);
        let mut next = button("@#>|", "Next", &mut header);
        row.end();
        row.set_pad(0);
        header.fixed(&row, 90);
        let mut vol = Slider::default().with_type(SliderType::Horizontal);
        vol.set_maximum(6_f64);
        vol.set_value(3_f64);
        vol.deactivate();
        header.end();
        header.set_pad(10);
        let mut browser = Browser::default().with_type(BrowserType::Hold);
        let mut song = Progress::default();
        song.set_selection_color(Color::Black);
        song.deactivate();
        margin.end();
        margin.fixed(&song, 30);
        margin.fixed(&header, 30);
        margin.set_margin(10);
        margin.set_pad(10);
        window.end();
        window.make_resizable(true);
        window.show();
        window.emit(sender, Message::Quit(true));
        let mut menu = MenuButton::default().with_type(MenuButtonType::Popup3);
        menu.add_emit(
            "@#1+  &Remove",
            Shortcut::Ctrl | 'd',
            MenuFlag::Normal,
            sender,
            Message::Remove,
        );
        browser.handle({
            let menu = menu.clone();
            move |_, event| match event {
                Event::Push => match app::event_mouse_button() {
                    app::MouseButton::Right => {
                        menu.popup();
                        true
                    }
                    app::MouseButton::Left => true,
                    _ => false,
                },
                _ => false,
            }
        });
        prev.set_callback({
            let mut browser = browser.clone();
            move |_| {
                match browser.value() > 1 {
                    true => browser.select(browser.value() - 1),
                    false => browser.select(browser.size()),
                };
                browser.redraw();
            }
        });
        next.set_callback({
            let mut browser = browser.clone();
            move |_| {
                match browser.value() < browser.size() {
                    true => browser.select(browser.value() + 1),
                    false => browser.select(1),
                };
                browser.redraw();
            }
        });
        let player = Rc::from(RefCell::from(
            Soloud::default().expect("Cannot access audio backend"),
        ));
        vol.set_callback({
            let player = player.clone();
            move |vol| player.borrow_mut().set_global_volume(vol.value() as f32)
        });
        play.set_callback({
            let browser = browser.clone();
            let player = player.clone();
            let mut song = song.clone();
            let vol = vol.clone();
            move |play| {
                if browser.size() > 0 {
                    if player.borrow().active_voice_count() > 0 {
                        play.set_label("@#>");
                        play.set_tooltip("Start");
                        player.borrow().stop_all();
                    } else {
                        play.set_label("@#||");
                        play.set_tooltip("Stop");
                        let mut wav = Wav::default();
                        if wav
                            .load(Path::new(&browser.text(browser.value()).unwrap()))
                            .is_ok()
                        {
                            song.set_maximum(wav.length());
                            let handle = player.borrow().play(&wav);
                            while player.borrow().active_voice_count() > 0 {
                                app::wait();
                                thread::sleep(Duration::from_millis(100));
                                player.borrow_mut().set_volume(handle, vol.value() as f32);
                                song.set_value(player.borrow().stream_time(handle));
                                song.set_label(&format!(
                                    "{:.1}%",
                                    wav.length() / 600_f64 * player.borrow().stream_time(handle)
                                ));
                            }
                        }
                    }
                }
            }
        });
        Self {
            player,
            menu,
            browser,
            prev,
            play,
            next,
            song,
            vol,
        }
    }
    pub fn get(&self) -> Vec<String> {
        (1..=self.browser.size())
            .map(|idx| self.browser.text(idx).unwrap())
            .collect()
    }
    pub fn plus(&mut self) {
        let mut dialog = FileChooser::new(
            std::env::var("HOME").unwrap(),
            "*.{mp3}",
            FileChooserType::Single,
            "Choose File...",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        if dialog.count() > 0 {
            for item in 1..=dialog.count() {
                if let Some(file) = dialog.value(item) {
                    self.browser.add(&file);
                };
            }
            self.browser.sort();
            self.browser.select(1);
            self.play.activate();
            self.vol.activate();
            self.song.activate();
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
                    self.add(
                        entries
                            .map(|entry| entry.ok().unwrap().path())
                            .filter(|path| {
                                ["mp3"]
                                    .map(|x| Some(OsStr::new(x)))
                                    .contains(&path.extension())
                            })
                            .map(|path| format!("{}", path.display()))
                            .collect(),
                    )
                };
                self.browser.sort();
                self.browser.select(1);
                self.play.activate();
                self.vol.activate();
                self.song.activate();
                self.next.activate();
                self.prev.activate();
            };
        };
    }
    pub fn add(&mut self, model: Vec<String>) {
        for item in model {
            self.browser.add(&item);
        }
    }
    pub fn remove(&mut self) {
        match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
            Some(0) => {
                self.browser.remove(self.browser.value());
            }
            Some(2) => {
                if fs::remove_file(self.browser.text(self.browser.value()).unwrap()).is_ok() {
                    self.browser.remove(self.browser.value());
                }
            }
            _ => {}
        };
    }
}

pub fn button(label: &str, tooltip: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    element.deactivate();
    flex.fixed(&element, 30);
    element
}
