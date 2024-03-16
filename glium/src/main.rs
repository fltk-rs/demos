#[macro_use]
extern crate glium;

use glium::Surface;
use fltk::{
    prelude::*,
    *,
};

use std::{
    rc::Rc,
    cell::RefCell,
    os::raw::c_void
};

#[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);



fn main() {
    let app = app::App::default();
    let mut win = window::GlWindow::default().with_size(730, 430);
    win.make_resizable(true);
    win.set_mode(enums::Mode::Opengl3);
    win.end();
    win.show();
    let gl_window = Rc::new(RefCell::new(win.clone()));

    struct Backend {
        gl_window: Rc<RefCell<window::GlWindow>>,
    }

    unsafe impl glium::backend::Backend for Backend {
        fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
            Ok(self.gl_window.borrow_mut().swap_buffers())
        }

        unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
            self.gl_window.borrow().get_proc_address(symbol) as *const _
        }

        fn get_framebuffer_dimensions(&self) -> (u32, u32) {
            (self.gl_window.borrow().width() as u32, self.gl_window.borrow().height() as u32)
        }

        fn is_current(&self) -> bool {
            unimplemented!()
        }

        unsafe fn make_current(&self) {
            self.gl_window.borrow_mut().make_current()
        }
    }

    let context = unsafe {
        let backend = Backend { gl_window: gl_window };
        glium::backend::Context::new(backend, false, Default::default())
    }.unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&context, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&context, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = glium::Frame::new(context.clone(), context.get_framebuffer_dimensions());
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();
    target.finish().unwrap();

    app.run().unwrap();
}
