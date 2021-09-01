use fltk::{enums::*, prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;
use egui::{vec2, Pos2, Rect};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let a = app::App::default();
    let mut win = window::GlutWindow::new(100, 100, SCREEN_WIDTH as _, SCREEN_HEIGHT as _, None);
    win.set_mode(Mode::Opengl3);
    win.end();
    win.make_resizable(true);
    win.show();
    win.make_current();

    let mut painter = fltk_egui::Painter::new(&win, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut egui_ctx = egui::CtxRef::default();

    let native_pixels_per_point = win.pixels_per_unit();

    let (width, height) = (win.w(), win.h());

    let egui_input_state = fltk_egui::EguiInputState::new(egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            Pos2::new(0f32, 0f32),
            vec2(width as f32, height as f32) / native_pixels_per_point,
        )),
        pixels_per_point: Some(native_pixels_per_point),
        ..Default::default()
    });

    let state_rc = Rc::from(RefCell::from(egui_input_state));
    let state = state_rc.clone();
    win.handle(move |_, ev| match ev {
        enums::Event::Push
        | enums::Event::Released
        | enums::Event::KeyDown
        | enums::Event::KeyUp
        | enums::Event::MouseWheel
        | enums::Event::Resize
        | enums::Event::Move
        | enums::Event::Drag => {
            fltk_egui::input_to_egui(ev, &mut state.borrow_mut());
            true
        }
        _ => false,
    });

    let mut name = String::new();
    let mut age = 0;
    while a.wait() {
        let state = state_rc.clone();
        egui_ctx.begin_frame(state.borrow_mut().input.take());
        state.borrow_mut().input.pixels_per_point = Some(native_pixels_per_point);

        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.6, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        egui::Window::new("Egui with FLTK and GL").show(&egui_ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut name);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{}', age {}", name, age));
        });

        let (egui_output, paint_cmds) = egui_ctx.end_frame();

        //Handle cut, copy text from egui
        if !egui_output.copied_text.is_empty() {
            fltk_egui::copy_to_clipboard(&mut state.borrow_mut(), egui_output.copied_text);
        }

        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        //Since we are custom drawing an OpenGL Triangle we don't need egui to clear the background.
        painter.paint_jobs(
            None,
            paint_jobs,
            &egui_ctx.texture(),
            native_pixels_per_point,
        );

        win.swap_buffers();
        win.flush();

        app::sleep(0.016);
        app::awake();
    }
}