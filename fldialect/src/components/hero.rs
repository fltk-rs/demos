use {
    crate::{
        constants::{INFO, SPACE},
        elements,
    },
    fltk::{
        app,
        app::WidgetId,
        dialog::{alert_default, FileChooser, FileChooserType, HelpDialog},
        enums::{Cursor, Event},
        frame::Frame,
        group::{Flex, FlexType},
        menu::MenuItem,
        prelude::GroupExt,
        prelude::{DisplayExt, WidgetBase, WidgetExt},
        text::TextEditor,
    },
};

#[derive(Clone)]
pub struct Hero {
    pub layout: Flex,
    pub from: TextEditor,
    pub frame: Frame,
    pub to: TextEditor,
}

impl Hero {
    pub fn build() -> Self {
        let mut layout = Flex::default().column();
        let mut from = elements::text("Source").with_id("text_from");
        let mut frame = Frame::default();
        let mut to = elements::text("Target").with_id("text_to");
        layout.end();
        layout.set_pad(0);
        layout.fixed(&frame, SPACE);
        let popup = MenuItem::new(&["Paste", "Copy", "Cut"]);
        to.handle({
            let popup = popup.clone();
            move |to, event| match event {
                Event::Push => text_cb(to, &popup),
                _ => false,
            }
        });
        frame.handle({
            let mut layout = layout.clone();
            let from = from.clone();
            move |frame, event| match event {
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
            }
        });
        from.handle({
            let mut dnd = false;
            let mut released = false;
            move |from, event| match event {
                Event::DndEnter => {
                    dnd = true;
                    dnd
                }
                Event::DndDrag => true,
                Event::DndRelease => {
                    released = true;
                    released
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
        Self {
            layout,
            from,
            frame,
            to,
        }
    }
    pub fn open(&mut self) {
        let mut dialog = FileChooser::new(
            std::env::var("HOME").unwrap(),
            "*.{txt,md}",
            FileChooserType::Single,
            "Open ...",
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
                    "Save ...",
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
        if self.layout.width() < self.layout.height() {
            self.layout.set_type(FlexType::Column);
            self.layout.fixed(&self.from, 0);
        } else {
            self.layout.set_type(FlexType::Row);
            self.layout.fixed(&self.from, 0);
        };
        self.layout.fixed(&self.frame, SPACE);
        app::redraw();
    }
    pub fn info(&self) {
        let mut dialog = HelpDialog::default();
        dialog.set_value(INFO);
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
