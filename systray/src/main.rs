extern crate native_windows_gui as nwg;
use fltk::*;
use lazy_static::lazy_static;
use nwg::NativeUi;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref FLTK_WIN_SHOWN: AtomicBool = AtomicBool::new(false);
}

#[derive(Default)]
pub struct SystemTray {
    window: nwg::MessageWindow,
    icon: nwg::Icon,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    tray_item1: nwg::MenuItem,
    tray_item2: nwg::MenuItem,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn show_main_win(&self) {
        fltk_gui();
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

mod system_tray_ui {
    use super::*;
    use native_windows_gui as nwg;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct SystemTrayUi {
        inner: Rc<SystemTray>,
        default_handler: RefCell<Vec<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<SystemTrayUi> for SystemTray {
        fn build_ui(mut data: SystemTray) -> Result<SystemTrayUi, nwg::NwgError> {
            use nwg::Event as E;

            // Resources
            nwg::Icon::builder()
                .source_file(Some("./sat.ico"))
                .build(&mut data.icon)?;

            nwg::MessageWindow::builder().build(&mut data.window)?;

            nwg::TrayNotification::builder()
                .parent(&data.window)
                .icon(Some(&data.icon))
                .tip(Some("Hello"))
                .build(&mut data.tray)?;

            nwg::Menu::builder()
                .popup(true)
                .parent(&data.window)
                .build(&mut data.tray_menu)?;

            nwg::MenuItem::builder()
                .text("Show fltk Window")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item1)?;

            nwg::MenuItem::builder()
                .text("Exit")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item2)?;

            // Wrap-up
            let ui = SystemTrayUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnContextMenu => {
                            if &handle == &evt_ui.tray {
                                SystemTray::show_menu(&evt_ui);
                            }
                        }
                        E::OnMenuItemSelected => {
                            if &handle == &evt_ui.tray_item1 {
                                SystemTray::show_main_win(&evt_ui);
                            } else if &handle == &evt_ui.tray_item2 {
                                SystemTray::exit(&evt_ui);
                            }
                        }
                        _ => {}
                    }
                }
            };

            ui.default_handler
                .borrow_mut()
                .push(nwg::full_bind_event_handler(
                    &ui.window.handle,
                    handle_events,
                ));

            return Ok(ui);
        }
    }

    impl Drop for SystemTrayUi {
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for SystemTrayUi {
        type Target = SystemTray;

        fn deref(&self) -> &SystemTray {
            &self.inner
        }
    }
}

fn main() {
    let app = app::App::default();
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    fltk_gui();
    nwg::dispatch_thread_events_with_callback(move || app.run().unwrap());
}

pub fn fltk_gui() {
    if FLTK_WIN_SHOWN.load(Ordering::Relaxed) {
        return;
    }
    let mut win = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::new(10, 10, 380, 200, "");
    frame.set_frame(FrameType::EngraveBox);
    let mut but = button::Button::new(160, 220, 80, 40, "Click me!");
    win.end();
    win.show();
    win.set_callback2(|w| {
        if app::event() == Event::Hide || app::event() == Event::Close {
            FLTK_WIN_SHOWN.store(false, Ordering::Relaxed);
            w.hide();
        }
    });
    but.set_callback(move || frame.set_label("Hello world!"));
    FLTK_WIN_SHOWN.store(true, Ordering::Relaxed);
}
