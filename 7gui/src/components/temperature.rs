use crate::{HEIGHT, PAD, WIDTH};
use fltk::{enums::*, frame::Frame, group::Flex, input::IntInput, prelude::*};

pub fn temperature() {
    let mut flex = Flex::default_fill().with_label("    Temperature    ");
    Frame::default();
    let mut col = Flex::default_fill().column();
    Frame::default();
    let mut celsius = IntInput::default().with_label("Celsius = ");
    let mut fahrenheit = IntInput::default().with_label("Fahrenheit = ");
    Frame::default();
    col.end();
    col.fixed(&celsius, HEIGHT);
    col.fixed(&fahrenheit, HEIGHT);
    col.set_pad(PAD);
    Frame::default();
    flex.end();
    flex.set_margin(PAD);
    flex.set_pad(PAD);
    flex.fixed(&col, WIDTH * 2);
    celsius.set_trigger(CallbackTrigger::Changed);
    fahrenheit.set_trigger(CallbackTrigger::Changed);
    celsius.set_callback({
        let mut fahrenheit = fahrenheit.clone();
        move |celsius| {
            if let Ok(celsius) = celsius.value().parse::<i32>() {
                let value = f64::from(celsius) * (9.0 / 5.0) + 32.0;
                fahrenheit.set_value(&format!("{}", value.round()))
            } else {
                fahrenheit.set_value("");
            }
        }
    });
    fahrenheit.set_callback({
        let mut celsius = celsius.clone();
        move |fahrenheit| {
            if let Ok(fahrenheit) = fahrenheit.value().parse::<i32>() {
                let value = (f64::from(fahrenheit) - 32.0) * (5.0 / 9.0);
                celsius.set_value(&format!("{}", value.round()))
            } else {
                celsius.set_value("");
            }
        }
    });
}
