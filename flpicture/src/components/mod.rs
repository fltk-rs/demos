use {
    crate::Message,
    fltk::{
        app,
        button::Button,
        dialog::{choice2_default, FileChooser, FileChooserType},
        enums::{Event, Shortcut},
        frame::Frame,
        group::Flex,
        image::SharedImage,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::{GroupExt, ImageExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt},
        valuator::{Scrollbar, ScrollbarType},
        window::Window,
    },
    std::{env, ffi::OsStr, fs, path::Path},
};

#[derive(Clone)]
pub struct Page {
    pub window: Window,
    pub open: Button,
    pub prev: Button,
    pub size: Scrollbar,
    pub next: Button,
    pub rem: Button,
    pub image: Frame,
    pos: usize,
    images: Vec<String>,
}

impl Page {
    pub fn build(sender: app::Sender<Message>) -> Self {
        let mut menu = MenuButton::default().with_type(MenuButtonType::Popup3);
        for (label, chr, msg) in [
            ("&File/@#fileopen  &Open", 'o', Message::Open(true)),
            ("&File/@#menu  &Remove", 'p', Message::Remove),
            ("@#1+  &Quit", 'q', Message::Quit(true)),
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
            .with_label("FlPicView")
            .center_screen();
        let mut margin = Flex::default_fill().column();
        let mut footer = Flex::default();
        let mut open = button("@#+2fileopen", "Open ...", &mut footer);
        open.activate();
        open.emit(sender, Message::Open(true));
        let mut prev = button("@#+2<-", "Previous", &mut footer);
        prev.emit(sender, Message::Prev);
        let mut size = Scrollbar::default().with_type(ScrollbarType::HorizontalNice);
        size.emit(sender, Message::Size);
        size.set_range(0f64, 100f64);
        size.set_precision(0);
        size.set_value(50f64);
        size.deactivate();
        let mut next = button("@#+2->", "Next", &mut footer);
        next.emit(sender, Message::Next);
        let mut rem = button("@#+2menu", "Remove", &mut footer);
        rem.emit(sender, Message::Remove);
        footer.end();
        footer.set_pad(0);
        let mut image = Frame::default_fill();
        image.set_image(None::<SharedImage>);
        image.handle(move |_, event| match event {
            Event::Push => match app::event_mouse_button() {
                app::MouseButton::Right => {
                    menu.popup();
                    true
                }
                _ => false,
            },
            _ => false,
        });
        margin.end();
        margin.fixed(&footer, 30);
        margin.set_margin(10);
        window.end();
        window.make_resizable(true);
        window.show();
        window.emit(sender, Message::Quit(false));
        Self {
            window,
            open,
            prev,
            size,
            next,
            rem,
            image,
            pos: 0,
            images: Vec::new(),
        }
    }
    pub fn set(&mut self) {
        if !self.images.is_empty() {
            if let Ok(mut image) = SharedImage::load(&self.images[self.pos]) {
                image.scale(
                    (self.image.width() as f64 * self.size.value()) as i32 / 100,
                    (self.image.height() as f64 * self.size.value()) as i32 / 100,
                    true,
                    true,
                );
                self.image.set_image(Some(image));
                self.window.set_label(&format!(
                    "{} - FlPicView ({}%)",
                    self.images[self.pos],
                    self.size.value()
                ));
            };
            self.prev.activate();
            self.next.activate();
            self.rem.activate();
            self.size.activate();
        } else {
            self.prev.deactivate();
            self.next.deactivate();
            self.rem.deactivate();
            self.size.deactivate();
        };
        self.window.redraw();
    }
    pub fn open(&mut self, ui: bool) {
        if ui {
            let mut dialog = FileChooser::new(
                std::env::var("HOME").unwrap(),
                "*.{png,svg}",
                FileChooserType::Single,
                "Choose File...",
            );
            dialog.show();
            while dialog.shown() {
                app::wait();
            }
            if dialog.count() > 0 {
                if let Some(file) = dialog.value(1) {
                    self.load(&file);
                    self.set();
                };
            };
        } else {
            let args: Vec<String> = env::args().collect();
            if args.len() > 1 {
                self.load(&args[1]);
                self.set();
            };
        };
    }
    pub fn load(&mut self, file: &str) {
        if let Some(parent) = Path::new(file).parent() {
            if let Ok(entries) = fs::read_dir(parent) {
                self.images = entries
                    .map(|entry| entry.ok().unwrap().path())
                    .filter(|path| {
                        ["png", "svg"]
                            .map(|x| Some(OsStr::new(x)))
                            .contains(&path.extension())
                    })
                    .map(|path| format!("{}", path.display()))
                    .collect::<Vec<String>>();
                self.pos = self
                    .images
                    .iter()
                    .position(|item| item == file)
                    .unwrap_or(0);
                eprintln!("{:?}\n{}", self.images, self.pos);
            }
        };
    }
    pub fn next(&mut self) {
        match self.pos < self.images.len() - 1 {
            true => self.pos += 1,
            false => self.pos = 0,
        };
        self.set();
    }
    pub fn prev(&mut self) {
        match self.pos > 0 {
            true => self.pos -= 1,
            false => self.pos = self.images.len() - 1,
        };
        self.set();
    }
    pub fn rem(&mut self) {
        if let Some(0) = choice2_default(
            &format!("Remove {}?", self.images[self.pos]),
            "Remove",
            "Cancel",
            "",
        ) {
            if fs::remove_file(&self.images[self.pos]).is_ok() {
                self.images.remove(self.pos);
                match self.images.is_empty() {
                    false => self.next(),
                    true => {
                        self.image.set_image(None::<SharedImage>);
                        self.prev.deactivate();
                        self.next.deactivate();
                        self.rem.deactivate();
                    }
                };
            };
        };
    }
}

fn button(label: &str, tooltip: &str, layout: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    layout.fixed(&element, 30);
    element.set_tooltip(tooltip);
    element.deactivate();
    element
}
