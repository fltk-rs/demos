use fltk::{enums::*, prelude::*, *};
use std::ops::{Deref, DerefMut};

pub struct FancySlider {
    s: valuator::Slider,
}

impl FancySlider {
    pub fn new(x: i32, y: i32) -> Self {
        let mut s = valuator::Slider::new(x, y, 300, 10, "");
        s.set_type(valuator::SliderType::Horizontal);
        s.set_frame(FrameType::RFlatBox);
        s.set_color(Color::from_u32(0x868db1));
        s.draw(|s| {
            draw::set_draw_color(Color::Blue);
            draw::draw_pie(
                s.x() - 10 + (s.w() as f64 * s.value()) as i32,
                s.y() - 10,
                30,
                30,
                0.,
                360.,
            );
        });
        Self { s }
    }
}

impl Deref for FancySlider {
    type Target = valuator::Slider;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl DerefMut for FancySlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}
