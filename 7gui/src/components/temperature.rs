use crate::Message;
use fltk::{app::Sender, enums::*, frame::*, group::*, input::*, prelude::*};

pub struct Temperature {
    pub celsius: IntInput,
    pub fahrenheit: IntInput,
}

impl Temperature {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    Temperature    ");
        flex.fixed(&Frame::default(), 100);
        let col = Flex::default_fill().column();
        let mut celsius = IntInput::default().with_label("Celsius = ");
        let mut fahrenheit = IntInput::default().with_label("Fahrenheit = ");
        col.end();
        flex.end();
        flex.set_margin(10);
        flex.set_pad(10);
        celsius.set_trigger(CallbackTrigger::Changed);
        fahrenheit.set_trigger(CallbackTrigger::Changed);
        celsius.emit(sender, Message::Celsius);
        fahrenheit.emit(sender, Message::Fahrenheit);
        Self {
            celsius,
            fahrenheit,
        }
    }
    pub fn celsius(&mut self) {
        if let Ok(celsius) = self.celsius.value().parse::<i32>() {
            let value = f64::from(celsius) * (9.0 / 5.0) + 32.0;
            self.fahrenheit.set_value(&format!("{}", value.round()))
        } else {
            self.fahrenheit.set_value("");
        }
    }
    pub fn fahrenheit(&mut self) {
        if let Ok(fahrenheit) = self.fahrenheit.value().parse::<i32>() {
            let value = (f64::from(fahrenheit) - 32.0) * (5.0 / 9.0);
            self.celsius.set_value(&format!("{}", value.round()))
        } else {
            self.celsius.set_value("");
        }
    }
}
