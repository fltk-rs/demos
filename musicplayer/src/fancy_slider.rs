use {
    fltk::{
        draw,
        enums::*,
        prelude::{ValuatorExt, WidgetBase, WidgetExt},
        valuator::{Slider, SliderType},
    },
    std::ops::{Deref, DerefMut},
};

pub struct FancySlider {
    slider: Slider,
}

impl FancySlider {
    pub fn new(x: i32, y: i32) -> Self {
        let mut slider = Slider::new(x, y, 300, 10, "").with_type(SliderType::Horizontal);
        slider.set_color(Color::from_u32(0x868db1));
        slider.set_frame(FrameType::RFlatBox);
        slider.draw(|slider| {
            draw::set_draw_color(Color::Blue);
            draw::draw_pie(
                slider.x() - 10 + (slider.w() as f64 * slider.value()) as i32,
                slider.y() - 10,
                30,
                30,
                0.,
                360.,
            );
        });
        Self { slider }
    }
}

impl Deref for FancySlider {
    type Target = Slider;

    fn deref(&self) -> &Self::Target {
        &self.slider
    }
}

impl DerefMut for FancySlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slider
    }
}
