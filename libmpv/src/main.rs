// the current libmpv-rs doesn't have the render_gl api
// So we use a fork

use fltk::{prelude::*, *};
use libmpv::{
    render::{OpenGLInitParams, RenderContext, RenderParam, RenderParamApiType},
    FileState, Mpv,
};
use std::os::raw::c_void;

pub fn get_proc_address(win: &window::GlutWindow, name: &str) -> *mut c_void {
    win.get_proc_address(name) as _
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: mpv <video file>");
        std::process::exit(1);
    }

    let a = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut mpv_win = window::GlutWindow::default()
        .with_size(390, 290)
        .center_of_parent();
    win.end();
    win.make_resizable(true);
    win.show();
    mpv_win.make_current();

    let mut mpv = Mpv::new().expect("Error while creating MPV");
    let render_context = RenderContext::new(
        unsafe { mpv.ctx.as_mut() },
        vec![
            RenderParam::ApiType(RenderParamApiType::OpenGl),
            RenderParam::InitParams(OpenGLInitParams {
                get_proc_address,
                ctx: mpv_win.clone(),
            }),
        ],
    )
    .expect("Failed creating render context");
    mpv.event_context_mut().disable_deprecated_events().unwrap();
    mpv.playlist_load_files(&[(&args[1], FileState::AppendPlay, None)])
        .unwrap();

    while a.wait() {
        render_context
            .render::<window::GlutWindow>(0, mpv_win.w() as _, mpv_win.h() as _, true)
            .expect("Failed to draw on glutin window");
        mpv_win.swap_buffers();
        app::awake();
    }
}
