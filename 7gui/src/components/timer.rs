use crate::{Message, HEIGHT, PAD, WIDTH};
use fltk::{
    app::Sender, button::Button, enums::*, frame::Frame, group::*, misc::Progress, prelude::*,
    valuator::*,
};
use std::{thread, time::Duration};

pub struct Timer {
    pub progress: Progress,
    pub slider: HorSlider,
    pub dial: Dial,
}

impl Timer {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    Timer    ");
        Frame::default();
        let mut col = Flex::default().column();
        Frame::default();
        let mut progress = Progress::default().with_label("Elapsed Time: 0.0s");
        let mut slider = HorSlider::default();
        let mut row = Flex::default();
        Frame::default();
        let mut button = Button::default().with_label("Start");
        button.set_callback({
            let mut progress = progress.clone();
            move |button| {
                progress.set_value(0.0);
                button.set_label("Reset");
                thread::spawn({
                    let mut progress = progress.clone();
                    let mut button = button.clone();
                    move || {
                        while progress.value() < progress.maximum() {
                            thread::sleep(Duration::from_millis(100));
                            sender.send(Message::Tick);
                        }
                        progress.set_label("Done!");
                        button.set_label("Start");
                    }
                });
            }
        });
        let mut dial = Dial::default();
        Frame::default();
        row.end();
        row.fixed(&dial, HEIGHT);
        row.fixed(&button, WIDTH);
        Frame::default();
        col.end();
        col.fixed(&slider, HEIGHT);
        col.fixed(&progress, HEIGHT);
        col.fixed(&row, HEIGHT);
        col.set_margin(PAD);
        col.set_pad(PAD);
        Frame::default();
        flex.end();
        flex.fixed(&col, WIDTH * 3);
        const DURATION_MAXIMUM: f64 = 30.0;
        const DURATION_DEFAULT: f64 = DURATION_MAXIMUM / 2.0;
        progress.set_selection_color(Color::Blue);
        progress.set_maximum(DURATION_DEFAULT);
        dial.set_maximum(DURATION_DEFAULT);
        dial.set_precision(0);
        dial.set_callback(move |dial| dial.set_value(dial.value() + 1_f64));
        dial.deactivate();
        slider.set_value(DURATION_DEFAULT);
        slider.set_maximum(DURATION_MAXIMUM);
        slider.set_callback({
            let mut progress = progress.clone();
            move |slider| progress.set_maximum(slider.value())
        });
        Self {
            progress,
            slider,
            dial,
        }
    }
    pub fn tick(&mut self) {
        let step: f64 = 0.1;
        self.dial.do_callback();
        if self.progress.value() < self.slider.value() {
            self.progress.set_value(self.progress.value() + step);
            self.progress
                .set_label(&format!("Elapsed Time: {:.1}s", self.progress.value()));
        }
    }
}
