use crate::Message;
use chrono::{offset::Local, NaiveDate};
use fltk::{
    app::Sender, button::Button, dialog::*, enums::*, frame::Frame, group::Flex, input::Input,
    menu::*, prelude::*,
};

pub struct FlightBooker {
    pub choice: Choice,
    pub start: Input,
    pub back: Input,
    pub book: Button,
}

impl FlightBooker {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill()
            .with_label("    FlightBooker    ")
            .column();
        Frame::default();
        let row = Flex::default_fill();
        Frame::default();
        let mut choice = Choice::default();
        let mut start = Input::default();
        Frame::default();
        row.end();
        let row = Flex::default_fill();
        Frame::default();
        let mut back = Input::default();
        let mut book = Button::default().with_label("Book");
        Frame::default();
        row.end();
        Frame::default();
        let current: &str = &Local::now()
            .naive_local()
            .date()
            .format("%Y-%m-%d")
            .to_string();
        flex.end();
        flex.set_margin(10);
        flex.set_pad(10);
        choice.add_choice("one-way flight|return flight");
        choice.set_value(0);
        start.set_trigger(CallbackTrigger::Changed);
        start.set_value(current);
        back.deactivate();
        back.set_trigger(CallbackTrigger::Changed);
        back.set_value(current);
        choice.emit(sender, Message::Update);
        start.emit(sender, Message::Update);
        back.emit(sender, Message::Update);
        book.emit(sender, Message::Book);
        Self {
            choice,
            start,
            back,
            book,
        }
    }
    pub fn update(&mut self) {
        if self.choice.value() == 0 {
            self.back.deactivate();
            if get_date(&mut self.start).is_ok() {
                self.book.activate();
            } else {
                self.book.deactivate();
            }
        } else {
            self.back.activate();
            let start_date = get_date(&mut self.start);
            let return_date = get_date(&mut self.back);
            if start_date.is_ok()
                && return_date.is_ok()
                && start_date.unwrap() <= return_date.unwrap()
            {
                self.book.activate();
            } else {
                self.book.deactivate();
            }
        }
    }
    pub fn book(&mut self) {
        alert_default(&if self.choice.value() == 0 {
            format!(
                "You have booked a one-way flight for {}.",
                self.start.value()
            )
        } else {
            format!(
                "You have booked a return flight from {} to {}",
                self.start.value(),
                self.back.value()
            )
        });
    }
}
fn get_date(input: &mut Input) -> Result<NaiveDate, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d");
    input.set_color(match date {
        Ok(_) => Color::BackGround2,
        Err(_) => Color::Red,
    });
    input.redraw();
    date
}
