#![forbid(unsafe_code)]

mod components;

use {
    components::{Circles, Crud, Timer},
    fltk::{
        app,
        button::RadioButton,
        enums::{Color, Event, Font},
        group::{Flex, Wizard},
        misc::Progress,
        prelude::{ButtonExt, GroupExt, WidgetBase, WidgetExt, WindowExt},
        window::Window,
    },
};

#[derive(Clone, Copy)]
pub enum Message {
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
        let mut button = RadioButton::default().with_label(label);
        button.emit(sender, Message::Switch(idx as i32));
        if idx == 0 {
            button.set_value(true);
        }
    }
    row.end();
    row.set_pad(0);
    flex.fixed(&row, 30);
    let mut wizard = Wizard::default_fill();
    components::counter();
    components::temperature();
    components::flightbooker();
    let mut timer = Timer::build(sender);
    let mut crud = Crud::build(sender);
    let mut circles = Circles::build(sender);
    wizard.end();
    let mut process = Progress::default();
    process.set_selection_color(Color::Black);
    process.set_maximum(wizard.bounds().len() as f64);
    process.set_value(1_f64);
    flex.end();
    flex.set_pad(10);
    flex.fixed(&process, 10);
    flex.set_margin(10);
    window.end();
    window.make_resizable(true);
    window.show();
    window.emit(sender, Message::Quit(true));
    app::set_font_size(16);
    app::set_font(Font::Courier);
    app::App::default().with_scheme(app::AppScheme::Plastic);
    while app::wait() {
        match receiver.recv() {
            Some(Message::Switch(page)) => {
                wizard.set_current_widget(&wizard.child(page).unwrap());
                process.set_value((page + 1) as f64);
            }
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

pub const PAD: i32 = 10;
pub const WIDTH: i32 = 150;
pub const HEIGHT: i32 = 30;
