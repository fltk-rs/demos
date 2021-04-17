use fltk::app;

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
        systray::dispatch_thread_events_with_callback(move || app.run().unwrap());
    }

    #[cfg(not(target_os = "windows"))]
    app.run().unwrap();
}

