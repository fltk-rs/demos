use {
    crate::{
        constants::{WIDGET_HEIGHT, WIDGET_SPACE, WIDGET_WIDTH},
        elements,
    },
    fltk::{
        app,
        button::Button,
        enums::Font,
        frame::Frame,
        group::Flex,
        menu::Choice,
        prelude::{GroupExt, MenuExt, ValuatorExt, WidgetBase, WidgetExt},
        valuator::Counter,
    },
};

#[derive(Clone)]
pub struct Footer {
    fonts: Vec<Font>,
    pub layout: Flex,
    pub open: Button,
    _left: Frame,
    pub font: Choice,
    pub trans: Button,
    pub size: Counter,
    _right: Frame,
    pub save: Button,
}

impl Footer {
    pub fn build(flex: &mut Flex, font: i32, size: i32) -> Self {
        let fonts = Vec::from([
            "Courier".to_string(),
            "Helvetica".to_string(),
            "Times".to_string(),
        ]);
        let mut layout = Flex::default_fill();
        let mut component = Self {
            layout: layout.clone(),
            open: elements::button("@#fileopen", "Open...", &mut layout),
            _left: Frame::default(),
            font: elements::choice("Fonts", &fonts.join("|"), font, &mut layout),
            trans: elements::button("@#circle", "Translate", &mut layout),
            size: Counter::default().with_type(fltk::valuator::CounterType::Simple),
            _right: Frame::default(),
            save: elements::button("@#filesaveas", "Save as...", &mut layout),
            fonts: fonts
                .iter()
                .map(|item| Font::by_name(item))
                .collect::<Vec<Font>>(),
        };
        layout.end();
        layout.set_pad(WIDGET_SPACE);
        layout.fixed(&component.size, WIDGET_WIDTH);
        layout.fixed(&component.font, WIDGET_WIDTH);
        flex.fixed(&layout, WIDGET_HEIGHT);
        component.size.set_range(14.0, 22.0);
        component.size.set_precision(0);
        component.size.set_value(size as f64);
        component.fonts();
        component
    }
    pub fn fonts(&mut self) {
        app::set_font(self.fonts[self.font.value() as usize]);
        app::redraw();
    }
}
