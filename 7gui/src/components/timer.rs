use crate::Message;
use fltk::{app::Sender, button::*, enums::*, group::*, misc::*, prelude::*, valuator::*};
use std::{thread, time::Duration};

pub struct Timer {
    pub progress: Progress,
    pub slider: HorSlider,
    pub button: Button,
}

impl Timer {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    Timer    ").column();
        let mut component = Timer {
            progress: Progress::default().with_label("Elapsed Time: 0.0s"),
            slider: HorSlider::default(),
            button: Button::default().with_label("Reset"),
        };
        flex.end();
        flex.set_margin(10);
        flex.set_pad(10);
        const DURATION_DEFAULT: f64 = 15.0;
        const DURATION_MAXIMUM: f64 = 30.0;
        component.progress.set_selection_color(Color::Blue);
        component.progress.set_maximum(DURATION_DEFAULT);
        component.slider.set_value(DURATION_DEFAULT);
        component.slider.set_maximum(DURATION_MAXIMUM);
        component.slider.emit(sender, Message::ChangeDuration);
        component.button.emit(sender, Message::Reset);
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));
            sender.send(Message::Tick);
        });
        component
    }
    pub fn tick(&mut self) {
        if self.slider.value() - self.progress.value() >= 0.01 {
            self.progress.set_value(self.progress.value() + 0.1);
            self.progress
                .set_label(&format!("Elapsed Time: {:.1}s", self.progress.value()));
        }
    }
}
