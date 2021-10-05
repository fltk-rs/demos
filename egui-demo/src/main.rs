use egui_backend::{
    egui,
    fltk::{enums::*, prelude::*, *},
    gl, DpiScaling,
};
use fltk_egui as egui_backend;
use std::rc::Rc;
use std::{cell::RefCell, time::Instant};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    app::get_system_colors();
    app::set_font_size(20);
    let mut main_win = window::Window::new(100, 100, SCREEN_WIDTH as _, SCREEN_HEIGHT as _, None);
    let mut glut_win = window::GlWindow::new(5, 5, main_win.w() - 200, main_win.h() - 10, None);
    glut_win.set_mode(Mode::Opengl3);
    glut_win.end();
    let mut col = group::Flex::default()
        .column()
        .with_size(185, 590)
        .right_of(&glut_win, 5);
    col.set_frame(FrameType::DownBox);
    let mut frm = frame::Frame::default();
    frm.set_color(Color::Red.inactive());
    frm.set_frame(FrameType::FlatBox);
    let mut slider = valuator::Slider::default().with_type(valuator::SliderType::HorizontalFill);
    slider.set_slider_frame(FrameType::RFlatBox);
    slider.set_slider_size(0.20);
    slider.set_color(Color::Blue.inactive());
    slider.set_selection_color(Color::Red);
    col.set_size(&mut slider, 20);
    col.end();
    main_win.end();
    main_win.make_resizable(true);
    main_win.show();
    glut_win.make_current();

    let (painter, egui_input_state) =
        egui_backend::with_fltk(&mut glut_win, DpiScaling::Custom(1.5));
    let mut egui_ctx = egui::CtxRef::default();

    let state_rc = Rc::from(RefCell::from(egui_input_state));
    let painter_rc = Rc::from(RefCell::from(painter));
    let state = state_rc.clone();
    let painter = painter_rc.clone();
    main_win.handle({
        let mut w = glut_win.clone();
        move |_, ev| match ev {
            enums::Event::Push
            | enums::Event::Released
            | enums::Event::KeyDown
            | enums::Event::KeyUp
            | enums::Event::MouseWheel
            | enums::Event::Resize
            | enums::Event::Move
            | enums::Event::Drag => {
                let mut state = state.borrow_mut();
                state.fuse_input(&mut w, ev, &mut painter.borrow_mut());
                true
            }
            _ => false,
        }
    });

    let start_time = Instant::now();
    let mut name = String::new();
    let mut age = 0;
    let mut quit = false;

    while a.wait() {
        let mut state = state_rc.borrow_mut();
        let mut painter = painter_rc.borrow_mut();
        state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(state.input.take());
        frm.set_label(&format!("Hello {}", &name));
        slider.set_value(age as f64 / 120.);

        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.6, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        egui::CentralPanel::default().show(&egui_ctx, |ui| {
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
            ui.separator();
            if ui
                .button("Quit?")
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                quit = true;
            }
        });

        let (egui_output, paint_cmds) = egui_ctx.end_frame();
        state.fuse_output(&mut glut_win, &egui_output);

        let paint_jobs = egui_ctx.tessellate(paint_cmds);

        //Draw egui texture
        painter.paint_jobs(None, paint_jobs, &egui_ctx.texture());

        glut_win.swap_buffers();
        glut_win.flush();
        app::sleep(0.006);
        app::awake();
        if quit {
            break;
        }
    }
}
