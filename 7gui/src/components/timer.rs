use crate::{HEIGHT, PAD, WIDTH};
use fltk::{
    app, button::Button, enums::*, frame::Frame, group::*, misc::Progress, prelude::*, valuator::*,
};

const DURATION_MAXIMUM: f64 = 30.0;
const DURATION_DEFAULT: f64 = DURATION_MAXIMUM / 2.0;

pub fn timer() {
    let mut flex = Flex::default_fill().with_label("    Timer    ");
    Frame::default();
    let mut col = Flex::default().column();
    Frame::default();
    progress(&mut col);
    slider(&mut col);
    let mut row = Flex::default();
    Frame::default();
    let mut button = Button::default().with_label("Start");
    button.set_callback(tick);
    dial(&mut row);
    Frame::default();
    row.end();
    row.fixed(&button, WIDTH);
    Frame::default();
    col.end();
    col.fixed(&row, HEIGHT);
    col.set_margin(PAD);
    col.set_pad(PAD);
    Frame::default();
    flex.end();
    flex.fixed(&col, WIDTH * 3);
}

fn dial(flex: &mut Flex) {
    let mut element = Dial::default().with_id("Spinner");
    element.set_maximum(DURATION_DEFAULT);
    element.set_precision(0);
    element.set_callback(move |dial| dial.set_value(dial.value() + 1_f64));
    element.deactivate();
    flex.fixed(&element, HEIGHT);
}

fn slider(flex: &mut Flex) {
    let mut element = HorSlider::default().with_id("Duration");
    element.set_value(DURATION_DEFAULT);
    element.set_maximum(DURATION_MAXIMUM);
    element.set_callback(move |slider| {
        app::widget_from_id::<Progress>("Timer")
            .unwrap()
            .set_maximum(slider.value())
    });
    flex.fixed(&element, HEIGHT);
}

fn progress(flex: &mut Flex) {
    let mut element = Progress::default()
        .with_label("Elapsed Time: 0.0s")
        .with_id("Timer");
    element.set_selection_color(Color::Blue);
    element.set_maximum(DURATION_DEFAULT);
    flex.fixed(&element, HEIGHT);
}

fn tick(button: &mut Button) {
    let mut progress = app::widget_from_id::<Progress>("Timer").unwrap();
    let slider = app::widget_from_id::<HorSlider>("Duration").unwrap();
    progress.set_value(0.0);
    button.set_label("Reset");
    while progress.value() < progress.maximum() {
        let step: f64 = 0.1;
        app::sleep(step);
        app::wait();
        app::widget_from_id::<Dial>("Spinner")
            .unwrap()
            .do_callback();
        if progress.value() < slider.value() {
            progress.set_value(progress.value() + step);
            progress.set_label(&format!("Elapsed Time: {:.1}s", progress.value()));
        }
    }
    progress.set_label("Done!");
    button.set_label("Start");
}
