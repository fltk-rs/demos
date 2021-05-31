use fltk::app;
use std::sync::atomic::Ordering;

mod fltk_gui;
#[cfg(target_os = "windows")]
mod systray;

fn main() {
    let app = app::App::default();
    fltk_gui::fltk_gui();

    #[cfg(target_os = "windows")]
    {
        use crate::systray::NativeUi;
        systray::init().expect("Failed to init Native Windows GUI");
        let _ui = systray::SystemTray::build_ui(Default::default()).expect("Failed to build UI");
        systray::dispatch_thread_events_with_callback(move || {
            if fltk_gui::FLTK_WIN_SHOWN.load(Ordering::Relaxed) {
                app.run().unwrap();
            } else {
                app::sleep(0.030);
            }
        });
    }

    #[cfg(not(target_os = "windows"))]
    app.run().unwrap();
}
