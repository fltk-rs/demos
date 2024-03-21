use {
    crate::{
        constants::{COLORS, INFO, TEXT_SIZE, WIDGET_SPACE},
        elements,
    },
    fltk::{
        app,
        dialog::{alert_default, FileChooser, FileChooserType, HelpDialog},
        enums::{Cursor, Event},
        frame::Frame,
        group::{Flex, FlexType},
        menu::MenuItem,
        prelude::GroupExt,
        prelude::{DisplayExt, WidgetBase, WidgetExt},
        text::TextEditor,
    },
    fltk_theme::{color_themes, ColorTheme},
};

#[derive(Clone)]
pub struct Hero {
    pub theme: u8,
    pub layout: Flex,
    pub from: TextEditor,
    pub frame: Frame,
    pub to: TextEditor,
}

impl Hero {
    pub fn build(theme: u8) -> Self {
        let mut component = Self {
            theme,
            layout: Flex::default().column(),
            from: elements::text("Source"),
            frame: Frame::default(),
            to: elements::text("Target"),
        };
        component.theme(component.theme);
        component.layout.end();
        component.layout.set_pad(0);
        component.layout.fixed(&component.frame, WIDGET_SPACE);
        let popup = MenuItem::new(&["Paste", "Copy", "Cut"]);
        let popup_to = popup.clone();
        component.to.handle(move |to, event| match event {
            Event::Push => text_cb(to, &popup_to),
            _ => false,
        });
        let mut layout = component.layout.clone();
        let from = component.from.clone();
        component.frame.handle(move |frame, event| match event {
            Event::Push => true,
            Event::Drag => {
                match layout.get_type() {
                    FlexType::Column => {
                        if (layout.y()..=layout.height() + layout.y() - frame.height())
                            .contains(&app::event_y())
                        {
                            layout.fixed(&from, app::event_y() - layout.y());
                        }
                    }
                    FlexType::Row => {
                        if (layout.x()..=layout.width() + layout.x() - frame.width())
                            .contains(&app::event_x())
                        {
                            layout.fixed(&from, app::event_x() - layout.x());
                        }
                    }
                }
                app::redraw();
                true
            }
            Event::Enter => {
                frame.window().unwrap().set_cursor(match layout.get_type() {
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
        component.from.handle({
            let mut dnd = false;
            let mut released = false;
            move |from, event| match event {
                Event::DndEnter => {
                    dnd = true;
                    true
                }
                Event::DndDrag => true,
                Event::DndRelease => {
                    released = true;
                    true
                }
                Event::Paste => {
                    if dnd && released {
                        let path = app::event_text();
                        let path = path.trim();
                        let path = path.replace("file://", "");
                        let path = std::path::PathBuf::from(&path);
                        if path.exists() {
                            app::add_timeout3(0.0, {
                                let mut buf = from.buffer().unwrap();
                                move |_| {
                                    buf.load_file(&path).unwrap();
                                }
                            });
                        }
                        dnd = false;
                        released = false;
                        true
                    } else {
                        false
                    }
                }
                Event::DndLeave => {
                    dnd = false;
                    released = false;
                    true
                }
                Event::Push => text_cb(from, &popup),
                _ => false,
            }
        });
        component
    }
    pub fn open(&mut self) {
        let mut dialog = FileChooser::new(
            std::env::var("HOME").unwrap(),
            "*.{txt,md}",
            FileChooserType::Create,
            "Choose File...",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        if dialog.count() > 0 {
            if let Some(file) = dialog.value(1) {
                self.from
                    .buffer()
                    .unwrap()
                    .load_file(std::path::Path::new(&file))
                    .ok()
                    .unwrap();
            };
        };
    }
    pub fn save(&self) {
        if let Some(mut source) = self.to.buffer() {
            if !source.text().is_empty() {
                let mut dialog = FileChooser::new(
                    std::env::var("HOME").unwrap(),
                    "*.{txt,md}",
                    FileChooserType::Create,
                    "Choose File...",
                );
                dialog.show();
                while dialog.shown() {
                    app::wait();
                }
                if dialog.count() > 0 {
                    if let Some(file) = dialog.value(1) {
                        source.load_file(std::path::Path::new(&file)).ok().unwrap();
                    };
                };
            } else {
                alert_default("Target is empty.");
            };
        };
    }
    pub fn resize(&mut self) {
        let window = app::first_window().unwrap();
        if window.width() < window.height() {
            self.layout.set_type(FlexType::Column);
            self.layout.fixed(&self.from, 0);
        } else {
            self.layout.set_type(FlexType::Row);
            self.layout.fixed(&self.from, 0);
        };
        self.layout.fixed(&self.frame, WIDGET_SPACE);
        app::redraw();
    }
    pub fn theme(&mut self, ord: u8) {
        if ord == 0 {
            ColorTheme::new(color_themes::TAN_THEME).apply();
            app::set_scheme(app::Scheme::Base);
        } else {
            ColorTheme::new(color_themes::DARK_THEME).apply();
            app::set_scheme(app::Scheme::Plastic);
        };
        self.from.set_color(COLORS[ord as usize][0]);
        self.from.set_text_color(COLORS[ord as usize][1]);
        self.to.set_color(COLORS[ord as usize][0]);
        self.to.set_text_color(COLORS[ord as usize][1]);
        self.theme = ord;
    }
    pub fn info(&self) {
        let mut dialog = HelpDialog::default();
        dialog.set_value(INFO);
        dialog.set_text_size(TEXT_SIZE);
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
    }
}

fn text_cb(text: &TextEditor, popup: &MenuItem) -> bool {
    if app::event_mouse_button() == app::MouseButton::Right {
        let coords = app::event_coords();
        if let Some(val) = popup.popup(coords.0, coords.1) {
            match val.label().unwrap().as_str() {
                "Paste" => text.paste(),
                "Copy" => text.copy(),
                "Cut" => text.cut(),
                _ => {}
            };
        };
        true
    } else {
        false
    }
}
