#![forbid(unsafe_code)]

mod components;

use {
    components::{Circles, Counter, Crud, FlightBooker, Temperature, Timer},
    fltk::{
        app,
        button::RadioButton,
        enums::{Color, Event, Font},
        group::{Flex, Wizard},
        misc::Progress,
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
    Switch(i32),
    Quit(bool),
}

pub fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Plastic);
    let (sender, receiver) = app::channel::<Message>();
    let mut window = Window::default()
        .with_size(960, 540)
        .with_label("Fl7GUI")
        .center_screen();
    let mut flex = Flex::default_fill().column();
    let mut row = Flex::default();
    for (idx, label) in [
        "Counter",
        "Temperature",
        "Flightbooker",
        "Timer",
        "CRUD",
        "Circle",
    ]
    .into_iter()
    .enumerate()
    {
        RadioButton::default()
            .with_label(label)
            .emit(sender, Message::Switch(idx as i32));
    }
    row.end();
    row.set_pad(0);
    flex.fixed(&row, 30);
    let mut wizard = Wizard::default_fill();
    let mut counter = Counter::build(sender);
    let mut temperature = Temperature::build(sender);
    let mut flightbooker = FlightBooker::build(sender);
    let mut timer = Timer::build(sender);
    let mut crud = Crud::build(sender);
    let mut circles = Circles::build(sender);
    wizard.end();
    let mut process = Progress::default();
    process.set_selection_color(Color::Black);
    process.set_maximum(wizard.bounds().len() as f64);
    process.set_value(1_f64);
    flex.end();
    flex.set_pad(0);
    flex.fixed(&process, 10);
    flex.set_margin(10);
    window.end();
    window.make_resizable(true);
    window.show();
    window.emit(sender, Message::Quit(true));
    app::set_font_size(16);
    app::set_font(Font::Courier);
    while app.wait() {
        match receiver.recv() {
            Some(Message::Switch(page)) => {
                wizard.set_current_widget(&wizard.child(page).unwrap());
                process.set_value((page + 1) as f64);
            }
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
