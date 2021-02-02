use fltk::{app, prelude::*, window::Window};
use pixels::{Error, Pixels, SurfaceTexture};

pub enum Message {
    DrawRequested,
    Other,
}

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;


struct World {
    box_x: i16,
    box_y: i16,
}

fn main() -> Result<(), Error> {
    let app = app::App::default();
    let (s, r) = app::channel::<Message>();
    let mut win = Window::default().with_size(WIDTH as i32, HEIGHT as i32).with_label("Pixels");
    win.end();
    win.show();
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &win);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let world = World::new();
    win.draw(move || s.send(Message::DrawRequested));

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::DrawRequested => {
                    world.draw(pixels.get_frame());
                    if pixels
                        .render()
                        .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                        .is_err()
                    {
                        win.hide();
                    }
                }
                _ => (),
            }
        }
    }
    Ok(())
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let inside_the_box = x >= self.box_x
                && x < self.box_x + BOX_SIZE
                && y >= self.box_y
                && y < self.box_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
