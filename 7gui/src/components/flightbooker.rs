use {
    crate::{HEIGHT, PAD, WIDTH},
    chrono::{offset::Local, NaiveDate},
    fltk::{
        button::Button, dialog::*, enums::*, frame::Frame, group::Flex, input::Input, menu::Choice,
        prelude::*,
    },
};

pub fn flightbooker() {
    let mut flex = Flex::default_fill().with_label("    FlightBooker    ");
    Frame::default();
    let mut col = Flex::default().column();
    Frame::default();
    let mut choice = Choice::default();
    let mut book = Button::default().with_label("Book");
    Frame::default();
    col.end();
    col.fixed(&choice, HEIGHT);
    col.fixed(&book, HEIGHT);
    col.set_pad(PAD);
    flex.fixed(&col, WIDTH);
    let mut col = Flex::default().column();
    Frame::default();
    let mut start = Input::default();
    let mut back = Input::default();
    Frame::default();
    col.end();
    col.fixed(&start, HEIGHT);
    col.fixed(&back, HEIGHT);
    col.set_pad(PAD);
    flex.fixed(&col, WIDTH);
    Frame::default();
    let current: &str = &Local::now()
        .naive_local()
        .date()
        .format("%Y-%m-%d")
        .to_string();
    flex.end();
    flex.set_margin(PAD);
    flex.set_pad(PAD);
    choice.add_choice("one-way flight|return flight");
    choice.set_value(0);
    start.set_trigger(CallbackTrigger::Changed);
    start.set_value(current);
    back.deactivate();
    back.set_trigger(CallbackTrigger::Changed);
    back.set_value(current);
    choice.set_callback({
        let mut start = start.clone();
        let mut back = back.clone();
        let mut book = book.clone();
        move |choice| update(choice, &mut start, &mut back, &mut book)
    });
    start.set_callback({
        let choice = choice.clone();
        let mut back = back.clone();
        let mut book = book.clone();
        move |start| update(&choice, start, &mut back, &mut book)
    });
    back.set_callback({
        let choice = choice.clone();
        let mut start = start.clone();
        let mut book = book.clone();
        move |back| update(&choice, &mut start, back, &mut book)
    });
    book.set_callback({
        let choice = choice.clone();
        let start = start.clone();
        let back = back.clone();
        move |_| {
            alert_default(&if choice.value() == 0 {
                format!("You have booked a one-way flight for {}.", start.value())
            } else {
                format!(
                    "You have booked a return flight from {} to {}",
                    start.value(),
                    back.value()
                )
            });
        }
    });
}

fn update(choice: &Choice, start: &mut Input, back: &mut Input, book: &mut Button) {
    if choice.value() == 0 {
        back.deactivate();
        if get_date(start).is_ok() {
            book.activate();
        } else {
            book.deactivate();
        }
    } else {
        back.activate();
        let start_date = get_date(start);
        let return_date = get_date(back);
        if start_date.is_ok() && return_date.is_ok() && start_date.unwrap() <= return_date.unwrap()
        {
            book.activate();
        } else {
            book.deactivate();
        }
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
