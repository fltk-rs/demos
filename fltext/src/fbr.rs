#![allow(clippy::single_match)]

use crate::{cbs, state::STATE, utils};
use fltk::{enums::*, prelude::*, *};
use notify::{event::EventKind, Event, RecursiveMode, Watcher};
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn menu_cb(m: &mut impl MenuExt) {
    if let Ok(mpath) = m.item_pathname(None) {
        match mpath.as_str() {
            "/New File...\t" => cbs::new_file(),
            "/New Dir...\t" => cbs::new_dir(),
            _ => (),
        }
    }
}

pub fn init_menu(m: &mut (impl MenuExt + 'static)) {
    m.add(
        "New File...\t",
        Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal,
        menu_cb,
    );
    m.add(
        "New Dir...\t",
        Shortcut::Ctrl | Shortcut::Shift | 'n',
        menu::MenuFlag::Normal,
        menu_cb,
    );
}

pub fn fbr_cb(f: &mut browser::FileBrowser, watcher: &mut dyn Watcher) {
    if let Some(path) = f.text(f.value()) {
        let path = PathBuf::from(path);
        if path.exists() {
            if path.is_dir() {
                watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();
                f.load(&path).expect("Couldn't load directory!");
                let cwd = env::current_dir().unwrap();
                env::set_current_dir(cwd.join(path)).unwrap();
                let mut info: frame::Frame = app::widget_from_id("info").unwrap();
                info.set_label(&format!(
                    "Directory: {}",
                    utils::strip_unc_path(&env::current_dir().unwrap())
                ));
                f.set_damage(true);
            } else {
                let mut is_image = false;
                if let Some(ext) = path.extension() {
                    match ext.to_str().unwrap() {
                        "jpg" | "gif" | "png" | "bmp" => is_image = true,
                        _ => (),
                    }
                }
                if is_image {
                    let img = image::SharedImage::load(path).unwrap();
                    let mut win: window::Window = app::widget_from_id("image_dialog").unwrap();
                    win.resize(win.x(), win.y(), img.w(), img.h());
                    win.child(0).unwrap().set_image(Some(img));
                    win.show();
                } else {
                    STATE.with(move |s| {
                        s.append(Some(path.canonicalize().unwrap()));
                    });
                }
            }
        }
    }
}

pub struct Fbr {
    g: group::Group,
}

impl Fbr {
    pub fn new(current_path: &Path) -> Self {
        let mut g = group::Group::default().with_id("fbr_group");
        let mut fbr = browser::FileBrowser::default()
            .with_type(browser::BrowserType::Hold)
            .with_id("fbr");
        fbr.load(current_path)
            .expect("Failed to load working directory");
        fbr.set_color(Color::Background.darker());
        let mut m = menu::MenuButton::default()
            .with_type(menu::MenuButtonType::Popup3)
            .with_id("pop1");
        init_menu(&mut m);
        g.end();
        let mut watcher = notify::recommended_watcher({
            let mut fbr = fbr.clone();
            move |res: Result<Event, notify::Error>| match res {
                Ok(event) => {
                    let mut needs_update = false;
                    match event.kind {
                        EventKind::Create(_) => {
                            needs_update = true;
                        }
                        EventKind::Remove(_) => {
                            needs_update = true;
                        }
                        _ => (),
                    }
                    if needs_update {
                        fbr.load(env::current_dir().unwrap()).unwrap();
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        })
        .unwrap();
        watcher
            .watch(current_path, RecursiveMode::NonRecursive)
            .unwrap();
        fbr.set_callback(move |f| fbr_cb(f, &mut watcher));
        g.resize_callback(move |_, x, y, w, h| {
            m.resize(x, y, w, h);
            fbr.resize(x, y, w, h);
        });

        Self { g }
    }
}

fltk::widget_extends!(Fbr, group::Group, g);
