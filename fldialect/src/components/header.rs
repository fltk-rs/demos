use {
    crate::{
        commands,
        constants::{WIDGET_HEIGHT, WIDGET_SPACE},
        elements,
    },
    fltk::{
        app,
        button::{Button, ToggleButton},
        frame::Frame,
        group::Flex,
        menu::{Choice, MenuButton},
        prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt},
    },
};

#[derive(Clone)]
pub struct Header {
    tick: u8,
    pub close: ToggleButton,
    _left: Frame,
    pub from: Choice,
    pub switch: Button,
    pub to: Choice,
    _right: Frame,
    pub menu: MenuButton,
}

impl Header {
    pub fn build(flex: &mut Flex, from: i32, to: i32) -> Self {
        let lang = commands::list();
        let mut layout = Flex::default_fill();
        let mut component = Self {
            tick: 0,
            menu: MenuButton::default(),
            _left: Frame::default(),
            from: elements::choice("From...", &lang, from, &mut layout),
            switch: elements::button("@#refresh", "Switch", &mut layout),
            to: elements::choice("To...", &lang, to, &mut layout),
            _right: Frame::default(),
            close: elements::toggle_button("@#<", "Speak", &mut layout),
        };
        layout.end();
        layout.set_pad(WIDGET_SPACE);
        layout.fixed(&component.menu, WIDGET_HEIGHT);
        layout.fixed(&component.from, 112);
        layout.fixed(&component.to, 112);
        flex.fixed(&layout, WIDGET_HEIGHT);
        component.menu.set_tooltip("Open application menu");
        component
    }
    pub fn switch(&mut self) {
        if self.from.value() != self.to.value() {
            let temp = self.from.value();
            self.from.set_value(self.to.value());
            self.to.set_value(temp);
            app::redraw();
        }
    }
    pub fn tick(&mut self) {
        match (6..9).contains(&self.tick) {
            true => self.tick += 1,
            false => self.tick = 7,
        };
        self.switch.set_label(&format!("@{}refresh", self.tick));
    }
}
