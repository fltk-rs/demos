use crate::Message;
use fltk::{app::Sender, button::Button, frame::Frame, group::Flex, output::Output, prelude::*};

pub struct Counter {
    model: u8,
    out: Output,
    pub inc: Button,
    pub dec: Button,
}

impl Counter {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    Counter    ");
        Frame::default();
        let col = Flex::default().column();
        Frame::default();
        let mut out = Output::default();
        let row = Flex::default();
        let mut dec = Button::default().with_label("-");
        let mut inc = Button::default().with_label("+");
        row.end();
        Frame::default();
        col.end();
        Frame::default();
        out.set_value("0");
        out.set_text_size(90);
        inc.emit(sender, Message::CounterInc);
        inc.set_label_size(90);
        dec.emit(sender, Message::CounterDec);
        dec.set_label_size(90);
        flex.end();
        flex.set_margin(10);
        flex.set_pad(10);
        Self {
            model: 0,
            out,
            inc,
            dec,
        }
    }
    pub fn inc(&mut self) {
        if self.model < 255 {
            self.model += 1;
            self.out.set_value(&self.model.to_string());
        }
    }
    pub fn dec(&mut self) {
        if self.model > 0 {
            self.model -= 1;
            self.out.set_value(&self.model.to_string());
        }
    }
}
