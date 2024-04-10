use {
    crate::{
        constants::{Message, HEIGHT, SPACE, WIDTH},
        elements,
    },
    fltk::{
        app,
        button::Button,
        enums::Font,
        frame::Frame,
        group::Flex,
        menu::Choice,
        prelude::{DisplayExt, GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt},
        text::TextEditor,
        valuator::{Counter, CounterType, Dial},
    },
};

#[derive(Clone)]
pub struct Footer {
    pub dial: Dial,
    pub layout: Flex,
    pub open: Button,
    pub font: Choice,
    pub trans: Button,
    pub size: Counter,
    pub save: Button,
}

impl Footer {
    pub fn build(sender: app::Sender<Message>, flex: &mut Flex, font: u8, value: u8) -> Self {
        let mut layout = Flex::default_fill();
        let mut open = elements::button("@#fileopen", "Open...", &mut layout);
        open.emit(sender.clone(), Message::Open);
        Frame::default();
        let mut font = elements::choice("Fonts", "Courier|Helvetica|Times", font, &mut layout);
        {
            font.set_callback(move |font| {
                for label in ["text_from", "text_to"] {
                    if let Some(mut text) = app::widget_from_id::<TextEditor>(label) {
                        text.set_text_font(Font::by_name(&font.choice().unwrap()));
                        text.redraw();
                    }
                }
            });
            font.do_callback();
        }
        let mut trans = elements::button("@#circle", "Translate", &mut layout);
        trans.emit(sender.clone(), Message::Request);
        let mut size = Counter::default().with_type(CounterType::Simple);
        {
            size.set_range(14_f64, 22_f64);
            size.set_precision(0);
            size.set_value(value as f64);
            size.set_tooltip("Size");
            size.set_callback(move |size| {
                for label in ["text_from", "text_to"] {
                    if let Some(mut text) = app::widget_from_id::<TextEditor>(label) {
                        text.set_text_size(size.value() as i32);
                        text.redraw();
                    }
                }
            });
            size.do_callback();
        };
        let mut dial = Dial::default();
        dial.set_range(0_f64, 39_f64);
        dial.set_precision(0);
        dial.set_callback(move |dial| dial.set_value(dial.value() + 1_f64));
        dial.deactivate();
        Frame::default();
        let mut save = elements::button("@#filesaveas", "Save as...", &mut layout);
        save.emit(sender.clone(), Message::Save);
        layout.end();
        layout.set_pad(SPACE);
        layout.fixed(&size, WIDTH - HEIGHT - SPACE);
        layout.fixed(&font, WIDTH);
        layout.fixed(&dial, HEIGHT);
        flex.fixed(&layout, HEIGHT);
        Self {
            layout,
            dial,
            open,
            font,
            trans,
            size,
            save,
        }
    }
    pub fn hide(&mut self) {
        if let Some(mut flex) = app::widget_from_id::<Flex>("flex") {
            if self.layout.visible() {
                flex.fixed(&self.layout, 0);
                self.layout.hide();
            } else {
                flex.fixed(&self.layout, HEIGHT);
                self.layout.show();
            };
            flex.redraw();
        }
    }
}
