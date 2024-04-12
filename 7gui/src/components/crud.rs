use {
    crate::{Message, PAD},
    fltk::{
        app::Sender, browser::*, button::*, menu::Choice, enums::*, frame::*, group::*, input::*, prelude::*,
    },
};

pub struct Crud {
    model: Vec<[String; 3]>,
    pub browser: HoldBrowser,
    pub filter: Input,
    choice: Choice,
    firstname: Input,
    lastname: Input,
    secretname: Input,
    pub create: Button,
    pub update: Button,
    pub delete: Button,
}

impl Crud {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    CRUD    ");
        let mut col = Flex::default_fill().column();
        let mut row = Flex::default();
        row.fixed(&Frame::default(), 30);
        let mut filter = Input::default().with_label("@#search");
        let mut choice = Choice::default();
        choice.set_tooltip("Search");
        choice.add_choice("Name|Surname");
        choice.set_value(0);
        choice.emit(sender, Message::CrudRead);
        row.end();
        col.fixed(&row, 30);
        let mut browser = HoldBrowser::default();
        let mut row = Flex::default();
        let mut create = Button::default().with_label("@#+  Create");
        let mut update = Button::default().with_label("@#refresh  Update");
        let mut delete = Button::default().with_label("@#1+  Delete");
        row.end();
        row.set_pad(PAD);
        col.end();
        col.set_pad(PAD);
        col.fixed(&row, 30);
        flex.fixed(&Frame::default(), 100);
        let mut col = Flex::default_fill().column();
        let firstname = Input::default().with_label("Name:");
        let lastname = Input::default().with_label("Surname:");
        let secretname = Input::default().with_label("Secretname:");
        col.end();
        col.fixed(&firstname, 30);
        col.fixed(&lastname, 30);
        col.fixed(&secretname, 30);
        col.fixed(&filter, 30);
        flex.end();
        flex.set_margin(PAD);
        flex.set_pad(PAD);
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
            secretname,
            create,
            update,
            choice,
            delete,
        };
        component.filter();
        component
    }
    pub fn select(&mut self){
        if self.browser.value() == 0 {
            self.update.deactivate();
            self.delete.deactivate();
        } else {
            let row: [String; 3] = self.model[(self.browser.value() - 1) as usize].clone();
            self.firstname.set_value(&row[0]);
            self.lastname.set_value(&row[1]);
            self.secretname.set_value(&row[2]);
            self.update.activate();
            self.delete.activate();
        }
    }
    pub fn filter(&mut self) {
        self.firstname.set_value("");
        self.lastname.set_value("");
        let prefix = self.filter.value().to_lowercase();
        self.browser.clear();
        self.model.sort();
        for item in &self.model {
            if item[self.choice.value() as usize].to_lowercase().starts_with(&prefix) {
                self.browser.add(&format!("{}, {}", item[0], item[1]));
            }
        }
    }
    pub fn create(&mut self) {
        self.model.push([self.lastname.value(),self.firstname.value(),self.secretname.value()]);
        self.filter();
    }
    pub fn update(&mut self) {
        if !self.firstname.value().is_empty() && !self.lastname.value().is_empty() {
            if let Some(name) = self.browser.text(self.browser.value()) {
                let index = self.model.iter().position(|value| format!("{}, {}", value[0], value[1]) == name).unwrap();
                self.model[index] = [self.lastname.value(), self.firstname.value(),self.secretname.value()];
                self.filter();
            }
        }
    }
    pub fn delete(&mut self) {
        if let Some(name) = self.browser.text(self.browser.value()) {
            let index = self.model.iter().position(|value| format!("{}, {}", value[0], value[1]) == name).unwrap();
            self.model.remove(index);
            self.filter();
        }
    }
}
