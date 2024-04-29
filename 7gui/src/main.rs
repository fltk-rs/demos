#![forbid(unsafe_code)]

mod components;

use {
    components::{Circles, Crud},
    fltk::{
        app,
        app::WidgetId,
        button::RadioButton,
        enums::{Color, Event, Font},
        group::{Flex, Wizard},
        misc::Progress,
        prelude::{ButtonExt, GroupExt, WidgetBase, WidgetExt, WindowExt},
        window::Window,
    },
    std::{env, fs, path::Path},
};

#[derive(Clone, Copy)]
pub enum Message {
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
}

pub const PAD: i32 = 10;
pub const HEIGHT: i32 = PAD * 3;
pub const WIDTH: i32 = HEIGHT * 5;

pub fn main() {
    let (sender, receiver) = app::channel::<Message>();
    let mut window = crate::window();
    let mut page = Flex::default_fill().column();
    let mut header = Flex::default();
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
        button.set_callback(move |button| crate::switch(button, idx as i32));
        if idx == 0 {
            button.set_value(true);
        }
    }
    header.end();
    header.set_pad(0);
    page.fixed(&header, HEIGHT);
    let wizard = Wizard::default_fill().with_id("Hero");
    components::counter();
    components::temperature();
    components::flightbooker();
    components::timer();
    let mut crud = Crud::build(sender);
    let mut circles = Circles::build(sender);
    wizard.end();
    crate::progress(wizard.bounds().len() as f64, &mut page);
    page.end();
    page.set_pad(PAD);
    page.set_margin(PAD);
    window.end();
    window.show();
    app::set_font_size(16);
    app::set_font(Font::Courier);
    app::App::default().with_scheme(app::AppScheme::Plastic);
    while app::wait() {
        match receiver.recv() {
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
            None => {}
        }
    }
}

fn progress(maximum: f64, flex: &mut Flex) -> Progress {
    let mut element = Progress::default().with_id("Progress");
    element.set_selection_color(Color::Black);
    element.set_maximum(maximum);
    element.set_value(1_f64);
    flex.fixed(&element, PAD);
    element
}

fn switch(_: &mut RadioButton, page: i32) {
    let mut wizard = app::widget_from_id::<Wizard>("Hero").unwrap();
    wizard.set_current_widget(&wizard.child(page).unwrap());
    app::widget_from_id::<Progress>("Progress")
        .unwrap()
        .set_value((page + 1) as f64);
}

fn window() -> Window {
    const DEFAULT: [u8; 4] = [
        3,   // [2] window_width * U8 +
        195, // [3] window_width_fract
        2,   // [4] window_height * U8 +
        30,  // [5] window_height_fract
    ];
    const U8: i32 = 255;
    const NAME: &str = "Fl7GUI";
    let file: String = env::var("HOME").unwrap() + "/.config/" + NAME;
    let params: Vec<u8> = if Path::new(&file).exists() {
        if let Ok(value) = fs::read(&file) {
            if value.len() == DEFAULT.len() {
                value
            } else {
                Vec::from(DEFAULT)
            }
        } else {
            Vec::from(DEFAULT)
        }
    } else {
        Vec::from(DEFAULT)
    };
    let mut element = Window::default()
        .with_size(
            params[0] as i32 * U8 + params[1] as i32,
            params[2] as i32 * U8 + params[3] as i32,
        )
        .with_label(NAME)
        .center_screen();
    element.size_range(
        DEFAULT[0] as i32 * U8 + DEFAULT[1] as i32,
        DEFAULT[2] as i32 * U8 + DEFAULT[3] as i32,
        0,
        0,
    );
    element.set_xclass(NAME);
    element.make_resizable(true);
    element.set_callback(move |window| {
        if app::event() == Event::Close {
            fs::write(
                &file,
                [
                    (window.width() / U8) as u8,
                    (window.width() % U8) as u8,
                    (window.height() / U8) as u8,
                    (window.height() % U8) as u8,
                ],
            )
            .unwrap();
            app::quit();
        }
    });
    element
}
