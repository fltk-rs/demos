#![allow(dead_code)]

use crate::state::STATE;
use fltk::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

pub struct FindDialog {
    win: window::Window,
}

impl FindDialog {
    pub fn new() -> Self {
        let idx = Rc::from(RefCell::from(0));
        let mut win = window::Window::new(0, 0, 300, 50, "Find").with_id("find");
        win.set_border(false);
        let mut row = group::Flex::default_fill();
        row.set_margin(10);
        let f = frame::Frame::default().with_label("Find:");
        row.fixed(&f, 30);
        let mut i = input::Input::default();
        i.set_trigger(enums::CallbackTrigger::EnterKeyAlways);
        let mut reg = button::ToggleButton::default().with_label(".*");
        reg.set_selection_color(reg.color().lighter());
        reg.set_tooltip("Use regex");
        row.fixed(&reg, 30);
        let mut b = button::Button::default().with_label("Next");
        i.set_callback({
            move |i| {
                let val = i.value();
                let reg_val = reg.value();
                if reg_val && regex::Regex::new(&val).is_err() {
                    i.set_text_color(enums::Color::Red);
                    return;
                } else {
                    i.set_text_color(enums::Color::Foreground);
                }
                if !val.is_empty() {
                    STATE.with({
                        let idx = idx.clone();
                        move |s| {
                            if let Some(buf) = s.buf().as_mut() {
                                let text = buf.text();
                                if reg_val {
                                    if let Ok(re) = regex::Regex::new(&val) {
                                        let v: Vec<_> =
                                            re.find_iter(&text).map(|m| m.range()).collect();
                                        if !v.is_empty() {
                                            let mut idx = idx.borrow_mut();
                                            let curr = &v[*idx];
                                            let mut ed: text::TextEditor =
                                                s.current_editor().unwrap();
                                            buf.select(curr.start as i32, curr.end as i32);
                                            ed.scroll(
                                                ed.count_lines(0, curr.start as i32, true),
                                                0,
                                            );
                                            *idx += 1;
                                            if *idx == v.len() {
                                                *idx = 0;
                                            }
                                        }
                                    }
                                } else {
                                    let v: Vec<_> = text.match_indices(&val).collect();
                                    if !v.is_empty() {
                                        let mut idx = idx.borrow_mut();
                                        let curr = v[*idx];
                                        let mut ed: text::TextEditor = s.current_editor().unwrap();
                                        buf.select(curr.0 as i32, (curr.0 + val.len()) as i32);
                                        ed.scroll(ed.count_lines(0, curr.0 as i32, true), 0);
                                        *idx += 1;
                                        if *idx == v.len() {
                                            *idx = 0;
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            }
        });
        b.set_callback(move |_| i.do_callback());
        row.fixed(&b, 60);
        row.end();
        win.end();
        win.handle(|win, ev| match ev {
            enums::Event::Hide => {
                win.hide();
                true
            }
            enums::Event::Close => {
                win.hide();
                true
            }
            _ => false,
        });
        Self { win }
    }
}

pub struct ReplaceDialog {
    win: window::Window,
}

impl ReplaceDialog {
    pub fn new() -> Self {
        let mut win = window::Window::new(0, 0, 300, 80, "Replace").with_id("replace");
        win.set_border(false);
        let mut col = group::Flex::default_fill().column();
        col.set_margin(5);
        let mut row = group::Flex::default();
        let f = frame::Frame::default().with_label("Search:");
        row.fixed(&f, 60);
        let mut search = input::Input::default();
        search.set_trigger(enums::CallbackTrigger::Changed);
        let mut reg = button::ToggleButton::default().with_label(".*");
        reg.set_selection_color(reg.color().lighter());
        reg.set_tooltip("Use regex");
        row.fixed(&reg, 30);
        row.end();
        let mut row = group::Flex::default();
        let f = frame::Frame::default().with_label("Replace:");
        row.fixed(&f, 60);
        let replace = input::Input::default();
        let mut b = button::Button::default().with_label("@>");
        b.set_tooltip("Apply");
        row.fixed(&b, 30);
        row.end();
        col.end();
        win.end();
        search.set_callback({
            let reg = reg.clone();
            move |i| {
                let val = i.value();
                let reg_val = reg.value();
                if reg_val && regex::Regex::new(&val).is_err() {
                    i.set_text_color(enums::Color::Red);
                } else {
                    i.set_text_color(enums::Color::Foreground);
                }
            }
        });
        b.set_callback(move |_| {
            let search = search.value();
            let replace = replace.value();
            let reg_val = reg.value();
            if reg_val && regex::Regex::new(&search).is_err() {
                return;
            }
            STATE.with({
                move |s| {
                    if let Some(buf) = s.buf().as_mut() {
                        let text = buf.text();
                        if reg_val {
                            if let Ok(re) = regex::Regex::new(&search) {
                                let ntext = re.replace(&text, &replace);
                                buf.set_text(&ntext);
                            }
                        } else {
                            let ntext = text.replace(&search, &replace);
                            buf.set_text(&ntext);
                        }
                        s.was_modified(true);
                    }
                }
            });
        });
        win.handle(|win, ev| match ev {
            enums::Event::Hide => {
                win.hide();
                true
            }
            enums::Event::Close => {
                win.hide();
                true
            }
            _ => false,
        });
        Self { win }
    }
}

pub struct ImageDialog {
    win: window::Window,
}

impl ImageDialog {
    pub fn new() -> Self {
        let mut win = window::Window::default()
            .with_size(400, 300)
            .with_id("image_dialog");
        let mut f = frame::Frame::default_fill();
        win.end();
        win.resize_callback(move |_win, _, _, w, h| f.resize(0, 0, w, h));
        Self { win }
    }
}
