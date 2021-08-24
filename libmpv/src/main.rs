use fltk::{enums::*, prelude::*, *};
use libmpv_sys::*;
use std::ffi::{CStr, CString};
use std::mem::transmute;
use std::os::raw::*;
use std::ptr::*;

pub unsafe extern "C" fn get_proc_address_mpv(
    ctx: *mut c_void,
    name: *const c_char,
) -> *mut c_void {
    let win = window::GlutWindow::from_widget_ptr(ctx as _);
    win.get_proc_address(&CStr::from_ptr(name).to_string_lossy().to_string()) as _
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
    mpv_win.set_mode(Mode::Opengl3);
    win.end();
    win.make_resizable(true);
    win.show();
    mpv_win.make_current();

    let mut mpv_gl: *mut mpv_render_context = null_mut();

    unsafe {
        let mpv = mpv_create();
        assert!(!mpv.is_null());
        if mpv_initialize(mpv) < 0 {
            std::process::exit(1);
        }

        let mut params = vec![
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_API_TYPE,
                data: transmute(MPV_RENDER_API_TYPE_OPENGL),
            },
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_INIT_PARAMS,
                data: transmute(&mut mpv_opengl_init_params {
                    get_proc_address: Some(get_proc_address_mpv),
                    get_proc_address_ctx: mpv_win.as_widget_ptr() as _,
                    extra_exts: null(),
                }),
            },
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_ADVANCED_CONTROL,
                data: transmute(&mut 1),
            },
            mpv_render_param {
                type_: 0,
                data: null_mut(),
            },
        ];

        if mpv_render_context_create(&mut mpv_gl, mpv, params.as_mut_ptr()) < 0 {
            std::process::exit(1);
        }

        let mut cmd: Vec<*const c_char> = vec![
            "loadfile\0".as_ptr() as _,
            CString::new(args[1].as_str()).unwrap().into_raw(),
            null(),
        ];
        mpv_command(mpv, cmd.as_mut_ptr());
    }

    while a.wait() {
        unsafe {
            let mut render_params = vec![
                mpv_render_param {
                    type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
                    data: transmute(&mut mpv_opengl_fbo {
                        fbo: 0,
                        w: mpv_win.w(),
                        h: mpv_win.h(),
                        internal_format: 0,
                    }),
                },
                mpv_render_param {
                    type_: mpv_render_param_type_MPV_RENDER_PARAM_FLIP_Y,
                    data: transmute(&mut 1),
                },
                mpv_render_param {
                    type_: 0,
                    data: null_mut(),
                },
            ];
            mpv_render_context_render(mpv_gl, render_params.as_mut_ptr());
            mpv_render_context_update(mpv_gl);
        }
        mpv_win.swap_buffers();
        app::sleep(0.016);
        app::awake();
    }
}
