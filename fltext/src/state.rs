#![allow(dead_code)]

use crate::gui;
use fltk::{app, group, prelude::*, text, utils::oncelock::Lazy};
use std::collections::HashMap;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU32, Ordering},
};

static COUNT: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Debug)]
pub struct MyBuffer {
    pub modified: bool,
    pub id: String,
    pub buf: text::TextBuffer,
    pub current_file: Option<PathBuf>,
}

pub struct State {
    pub map: HashMap<usize, MyBuffer>,
    pub current_dir: PathBuf,
}

impl State {
    pub fn new(current_dir: PathBuf) -> Self {
        let map = HashMap::default();
        State { map, current_dir }
    }
    pub fn append(&mut self, current_path: Option<PathBuf>) {
        let mut tabs: group::Tabs = app::widget_from_id("tabs").unwrap();
        let mut open = false;
        let mut edid = 0;
        for (k, v) in &self.map {
            if v.current_file == current_path {
                open = true;
                edid = *k;
                break;
            }
        }
        if !open {
            let old_count = COUNT.load(Ordering::Relaxed);
            let id = format!("edrow{}", old_count);
            COUNT.store(old_count + 1, Ordering::Relaxed);
            let ed = gui::create_ed(&mut tabs, &id, &current_path);
            let mybuf = MyBuffer {
                modified: false,
                id,
                buf: ed.buffer().unwrap(),
                current_file: current_path.map(|p| p.canonicalize().unwrap()),
            };
            self.map.insert(ed.as_widget_ptr() as usize, mybuf);
        } else {
            tabs.set_value(
                &text::TextEditor::from_dyn_widget_ptr(edid as *mut _)
                    .unwrap()
                    .parent()
                    .unwrap(),
            )
            .ok();
            tabs.set_damage(true);
        }
    }
    pub fn current_id(&self) -> Option<usize> {
        let tabs: group::Tabs = app::widget_from_id("tabs").unwrap();
        if tabs.children() == 0 {
            return None;
        }
        tabs.value()
            .unwrap()
            .child(0)
            .map(|ed| ed.as_widget_ptr() as usize)
    }
    pub fn was_modified(&mut self, flag: bool) {
        let mut tabs: group::Tabs = app::widget_from_id("tabs").unwrap();
        if tabs.children() == 0 {
            return;
        }
        let mut edrow = tabs.value().unwrap();
        if let Some(c) = edrow.child(0) {
            let id = c.as_widget_ptr() as usize;
            let mybuf = self.map.get_mut(&id).unwrap();
            mybuf.modified = flag;
            if let Some(f) = mybuf.current_file.as_ref() {
                if flag {
                    edrow.set_label(&format!("\t{} *", f.file_name().unwrap().to_str().unwrap()));
                } else {
                    edrow.set_label(&format!("\t{}", f.file_name().unwrap().to_str().unwrap()));
                }
                tabs.redraw();
            }
        }
    }
    pub fn modified(&self) -> bool {
        if let Some(current_id) = self.current_id() {
            let mybuf = self.map.get(&current_id).unwrap();
            mybuf.modified
        } else {
            false
        }
    }
    pub fn buf(&self) -> Option<text::TextBuffer> {
        if let Some(current_id) = self.current_id() {
            let mybuf = self.map.get(&current_id).unwrap();
            Some(mybuf.buf.clone())
        } else {
            None
        }
    }
    pub fn current_file(&self) -> Option<PathBuf> {
        if let Some(current_id) = self.current_id() {
            let mybuf = self.map.get(&current_id).unwrap();
            mybuf.current_file.clone()
        } else {
            None
        }
    }
    pub fn set_current_file(&mut self, path: PathBuf) {
        if let Some(current_id) = self.current_id() {
            let mybuf = self.map.get_mut(&current_id).unwrap();
            mybuf.current_file = Some(path)
        }
    }
    pub fn current_editor(&self) -> Option<text::TextEditor> {
        let tabs: group::Tabs = app::widget_from_id("tabs").unwrap();
        if tabs.children() == 0 {
            return None;
        }
        tabs.value()
            .unwrap()
            .child(0)
            .map(|c| text::TextEditor::from_dyn_widget(&c).unwrap())
    }
}

pub static STATE: Lazy<app::GlobalState<State>> = Lazy::new(app::GlobalState::<State>::get);

pub fn init_state(current_file: Option<PathBuf>, current_path: PathBuf) {
    let mut state = State::new(current_path);
    state.append(current_file);
    app::GlobalState::new(state);
}
