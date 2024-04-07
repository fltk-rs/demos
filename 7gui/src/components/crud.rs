use crate::Message;
use fltk::{
    app::Sender, browser::*, button::*, enums::*, frame::*, group::*, input::*, prelude::*,
};

pub struct Crud {
    model: Vec<String>,
    pub browser: HoldBrowser,
    pub filter: Input,
    firstname: Input,
    lastname: Input,
    pub create: Button,
    pub update: Button,
    pub delete: Button,
}

impl Crud {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    CRUD    ").column();
        let mut row = Flex::default_fill();
        let col = Flex::default_fill().column();
        let mut browser = HoldBrowser::default();
        col.end();
        let col = Flex::default_fill().column();
        Frame::default();
        col.end();
        row.fixed(&col, 150);
        let col = Flex::default_fill().column();
        let firstname = Input::default().with_label("Name:");
        let lastname = Input::default().with_label("Surname:");
        let mut filter = Input::default().with_label("Filter prefix:");
        col.end();
        row.end();
        let col = Flex::default_fill();
        let mut create = Button::default().with_label("Create");
        let mut update = Button::default().with_label("Update");
        let mut delete = Button::default().with_label("Delete");
        col.end();
        flex.fixed(&col, 35);
        flex.end();
        flex.set_margin(10);
        flex.set_pad(10);
        filter.set_trigger(CallbackTrigger::Changed);
        update.deactivate();
        delete.deactivate();
        browser.emit(sender, Message::CrudSelect);
        filter.emit(sender, Message::CrudRead);
        create.emit(sender, Message::CrudCreate);
        update.emit(sender, Message::CrudUpdate);
        delete.emit(sender, Message::CrudDelete);
        let mut component = Self {
            model: Vec::new(),
            browser,
            filter,
            firstname,
            lastname,
            create,
            update,
            delete,
        };
        component.filter();
        component
    }
    pub fn select(&mut self) {
        if self.browser.value() == 0 {
            self.update.deactivate();
            self.delete.deactivate();
        } else {
            self.update.activate();
            self.delete.activate();
        }
    }
    pub fn filter(&mut self) {
        let prefix = self.filter.value().to_lowercase();
        self.browser.clear();
        for item in &self.model {
            if item.to_lowercase().starts_with(&prefix) {
                self.browser.add(item);
            }
        }
        self.browser.sort();
        self.browser.select(1);
        self.select();
    }
    pub fn create(&mut self) {
        self.model.push(format!(
            "{}, {}",
            self.lastname.value(),
            self.firstname.value()
        ));
        self.filter();
    }
    pub fn update(&mut self) {
        let selected_name = self.browser.text(self.browser.value()).unwrap();
        let index = self.model.iter().position(|s| s == &selected_name).unwrap();
        self.model[index] = format!("{}, {}", self.lastname.value(), self.firstname.value());
        self.filter();
    }
    pub fn delete(&mut self) {
        let selected_name = self.browser.text(self.browser.value()).unwrap();
        let index = self.model.iter().position(|s| s == &selected_name).unwrap();
        self.model.remove(index);
        self.filter();
        self.select();
    }
}
