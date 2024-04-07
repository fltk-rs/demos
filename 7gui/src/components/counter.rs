use crate::Message;
use fltk::{app::Sender, button::*, group::*, output::*, prelude::*};

pub struct Counter {
    model: u8,
    out: Output,
    pub inc: Button,
    pub dec: Button,
}

impl Counter {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    Counter    ").column();
        let mut out = Output::default();
        let row = Flex::default_fill();
        let mut dec = Button::default().with_label("-");
        let mut inc = Button::default().with_label("+");
        row.end();
        out.set_value("0");
        inc.emit(sender, Message::CounterInc);
        dec.emit(sender, Message::CounterDec);
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
