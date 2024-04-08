use crate::Message;
use fltk::{
    app::Sender, button::Button, enums::*, frame::Frame, group::*, misc::Progress, prelude::*,
    valuator::*,
};
use std::{thread, time::Duration};

pub struct Timer {
    pub progress: Progress,
    pub slider: HorSlider,
}

impl Timer {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    Timer    ").column();
        Frame::default();
        let mut progress = Progress::default().with_label("Elapsed Time: 0.0s");
        let mut slider = HorSlider::default();
        let row = Flex::default();
        Frame::default();
        Button::default()
            .with_label("Reset")
            .emit(sender, Message::Reset);
        Frame::default();
        row.end();
        Frame::default();
        flex.end();
        flex.set_margin(10);
        flex.set_pad(10);
        const DURATION_DEFAULT: f64 = 15.0;
        const DURATION_MAXIMUM: f64 = 30.0;
        progress.set_selection_color(Color::Blue);
        progress.set_maximum(DURATION_DEFAULT);
        slider.set_value(DURATION_DEFAULT);
        slider.set_maximum(DURATION_MAXIMUM);
        slider.emit(sender, Message::ChangeDuration);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));
            sender.send(Message::Tick);
        });
        Self { progress, slider }
    }
    pub fn tick(&mut self) {
        if self.slider.value() - self.progress.value() >= 0.01 {
            self.progress.set_value(self.progress.value() + 0.1);
            self.progress
                .set_label(&format!("Elapsed Time: {:.1}s", self.progress.value()));
        }
    }
}
