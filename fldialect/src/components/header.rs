use {
    crate::{
        commands,
        constants::{Message, COLORS, DEFAULT, HEIGHT, NAME, SPACE, WIDTH},
        elements,
    },
    fltk::{
        app,
        button::{Button, ButtonType},
        enums::{Color, Shortcut},
        frame::Frame,
        group::Flex,
        menu::{Choice, MenuButton, MenuFlag},
        prelude::{DisplayExt, GroupExt, MenuExt, WidgetBase, WidgetExt},
        text::TextEditor,
    },
    fltk_theme::{color_themes, ColorTheme},
};

#[derive(Clone)]
pub struct Header {
    pub menu: MenuButton,
    pub voice: Button,
    pub from: Choice,
    pub switch: Button,
    pub to: Choice,
}

impl Header {
    pub fn build(
        sender: app::Sender<Message>,
        flex: &mut Flex,
        theme: u8,
        from: u8,
        to: u8,
    ) -> Self {
        let mut layout = Flex::default_fill();
        let mut menu = MenuButton::default();
        {
            menu.set_tooltip("Open application menu");
            let idx = menu.add_emit(
                "&View/&Night mode\t",
                Shortcut::Ctrl | 'n',
                MenuFlag::Toggle,
                sender.clone(),
                Message::Themes,
            );
            if theme == 1 {
                menu.at(idx).unwrap().set();
            };
            let idx: i32 = menu.add_emit(
                "&View/&Footer\t",
                Shortcut::None,
                MenuFlag::Toggle,
                sender.clone(),
                Message::Hide,
            );
            menu.at(idx).unwrap().set();
            for (label, chr, msg) in [
                ("&File/@#fileopen  &Open", 'o', Message::Open),
                ("&File/@#filesaveas  &Save", 's', Message::Save),
                ("@#circle  T&ranslate", 'r', Message::Request),
                ("@#refresh  Swi&tch", 't', Message::Switch),
                ("@#search  &Info", 'i', Message::Info),
                (
                    "@#reload  &Reload",
                    's',
                    Message::Reload(Vec::from(DEFAULT)),
                ),
            ] {
                menu.add_emit(
                    &format!("{label}\t"),
                    Shortcut::Ctrl | chr,
                    MenuFlag::Normal,
                    sender.clone(),
                    msg,
                );
            }
            let ord: i32 = menu.add_emit(
                "@#1+  &Quit",
                Shortcut::Ctrl | 'q',
                MenuFlag::Normal,
                sender.clone(),
                Message::Quit(true),
            );
            menu.at(ord).unwrap().set_label_color(Color::Red);
        }
        Frame::default();
        let lang = commands::list();
        let mut from = elements::choice("From...", &lang, from, &mut layout);
        from.emit(sender.clone(), Message::Rename);
        let mut switch = elements::button("@#refresh", "Switch", &mut layout);
        switch.emit(sender.clone(), Message::Switch);
        let mut to = elements::choice("To...", &lang, to, &mut layout);
        to.emit(sender.clone(), Message::Rename);
        Frame::default();
        let voice = elements::button("@#<", "Speak", &mut layout).with_type(ButtonType::Toggle);
        layout.end();
        layout.set_pad(SPACE);
        layout.fixed(&menu, HEIGHT);
        layout.fixed(&from, WIDTH);
        layout.fixed(&to, WIDTH);
        flex.fixed(&layout, HEIGHT);
        Self {
            menu,
            voice,
            from,
            switch,
            to,
        }
    }
    pub fn theme(&mut self) {
        if let Some(item) = self.menu.at(1) {
            app::set_scheme(if item.value() {
                ColorTheme::new(color_themes::DARK_THEME).apply();
                app::Scheme::Plastic
            } else {
                ColorTheme::new(color_themes::TAN_THEME).apply();
                app::Scheme::Base
            });
            for label in ["text_from", "text_to"] {
                if let Some(mut text) = app::widget_from_id::<TextEditor>(label) {
                    text.set_color(COLORS[item.value() as usize][0]);
                    text.set_text_color(COLORS[item.value() as usize][1]);
                }
            }
        }
    }
    pub fn switch(&mut self) {
        if self.from.value() != self.to.value() {
            let temp = self.from.value();
            self.from.set_value(self.to.value());
            self.to.set_value(temp);
            self.rename();
        }
    }
    pub fn rename(&mut self) {
        if let Some(mut window) = app::first_window() {
            window.set_label(&format!(
                "Translate from {} to {} - {}",
                self.from.choice().unwrap(),
                self.to.choice().unwrap(),
                NAME
            ));
        };
    }
}
