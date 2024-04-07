#![forbid(unsafe_code)]

mod components;

use {
    components::{Circles, Counter, Crud, FlightBooker, Temperature, Timer},
    fltk::{
        app,
        enums::{Event, Font},
        group::{Flex, Tabs},
        prelude::{GroupExt, ValuatorExt, WidgetBase, WidgetExt, WindowExt},
        window::Window,
    },
};

#[derive(Clone, Copy)]
pub enum Message {
    CounterInc,
    CounterDec,
    Celsius,
    Fahrenheit,
    Update,
    Book,
    Reset,
    ChangeDuration,
    Tick,
    CrudSelect,
    CrudCreate,
    CrudRead,
    CrudUpdate,
    CrudDelete,
    Undo,
    Redo,
    Add((i32, i32)),
    AdjustOpened,
    RadiusChanged,
    ModelChanged,
    Quit(bool),
}

pub fn main() {
    let mut window = Window::default()
        .with_size(960, 540)
        .with_label("Fl7GUI")
        .center_screen();
    let mut flex = Flex::default_fill();
    let mut tabs = Tabs::default_fill();
    let (sender, receiver) = app::channel::<Message>();
    let mut counter = Counter::build(sender);
    let mut temperature = Temperature::build(sender);
    let mut flightbooker = FlightBooker::build(sender);
    let mut timer = Timer::build(sender);
    let mut crud = Crud::build(sender);
    let mut circles = Circles::build(sender);
    tabs.end();
    tabs.auto_layout();
    flex.end();
    flex.set_margin(10);
    window.end();
    window.make_resizable(true);
    window.show();
    window.emit(sender, Message::Quit(false));
    app::set_font(Font::Courier);
    while app::App::default().with_scheme(app::AppScheme::Plastic).wait() {
        match receiver.recv() {
            Some(Message::CounterInc) => counter.inc(),
            Some(Message::CounterDec) => counter.dec(),
            Some(Message::Celsius) => temperature.celsius(),
            Some(Message::Fahrenheit) => temperature.fahrenheit(),
            Some(Message::Update) => flightbooker.update(),
            Some(Message::Book) => flightbooker.book(),
            Some(Message::Reset) => timer.progress.set_value(0.0),
            Some(Message::ChangeDuration) => timer.progress.set_maximum(timer.slider.value()),
            Some(Message::Tick) => timer.tick(),
            Some(Message::CrudSelect) => crud.select(),
            Some(Message::CrudCreate) => crud.create(),
            Some(Message::CrudRead) => crud.filter(),
            Some(Message::CrudUpdate) => crud.update(),
            Some(Message::CrudDelete) => crud.delete(),
            Some(Message::Undo) => circles.model.undo(),
            Some(Message::Redo) => circles.model.redo(),
            Some(Message::AdjustOpened) => circles.opened(),
            Some(Message::RadiusChanged) => circles.radius_changed(),
            Some(Message::ModelChanged) => circles.model_changed(),
            Some(Message::Add(pos)) => {
                circles.model.save();
                circles.model.set(pos, 20);
            }
            Some(Message::Quit(force)) => {
                if force || app::event() == Event::Close {
                    app::quit();
                }
            }
            None => {}
        }
    }
}
