mod term {
    use fltk::{
        enums::*,
        prelude::*,
        *,
    };
    use portable_pty::{
        native_pty_system,
        CommandBuilder,
        PtySize
    };
    use std::io::{
        Read,
        Write
    };

    pub struct AnsiTerm {
        st: text::SimpleTerminal,
    }

    impl Default for AnsiTerm {
        fn default() -> Self {
            AnsiTerm::new(0, 0, 0, 0, None)
        }
    }

    impl AnsiTerm {
        pub fn new<L: Into<Option<&'static str>>>(
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            label: L,
        ) -> Self {
            let mut st = text::SimpleTerminal::new(x, y, w, h, label);
            // SimpleTerminal handles many common ansi escape sequence
            st.set_ansi(true);
            let pair = native_pty_system()
                .openpty(PtySize {
                    cols: 80,
                    rows: 24,
                    pixel_width: 80 * 10,
                    pixel_height: 24 * 16,
                })
                .unwrap();

            let cmd = if cfg!(target_os = "windows") {
                CommandBuilder::new("cmd.exe")
            } else {
                let mut cmd = CommandBuilder::new("/bin/bash");
                cmd.args(["-i"]);
                cmd
            };

            let mut child = pair.slave.spawn_command(cmd).unwrap();
            let mut writer = pair.master.try_clone_writer().unwrap();
            let mut reader = pair.master.try_clone_reader().unwrap();

            std::thread::spawn({
                let mut st = st.clone();
                move || {
                    while child.try_wait().is_ok() {
                        let mut msg = [0u8; 1024];
                        if let Ok(sz) = reader.read(&mut msg) {
                            let msg = &msg[0..sz];
                            // we want to handle some escape sequences that the default SimpleTerminal doesn't
                            format(msg, &mut st);
                            app::awake();
                        }
                        std::thread::sleep(std::time::Duration::from_millis(30));
                    }
                }
            });

            let mut cmd = String::new();
            st.handle(move |t, ev| {
                let mut buf = t.buffer().unwrap();
                let mut sbuf = t.style_buffer().unwrap();
                match ev {
                    Event::KeyDown => match app::event_key() {
                        Key::Enter => {
                            let len = cmd.len() as i32;
                            let text_len = t.text().len() as i32;
                            buf.remove(text_len - len, text_len);
                            sbuf.remove(text_len - len, text_len);
                            writer.write_all(cmd.as_bytes()).unwrap();
                            writer.write_all(b"\n").unwrap();
                            cmd.clear();
                            true
                        }
                        Key::BackSpace => {
                            if !cmd.is_empty() {
                                let c = cmd.pop().unwrap();
                                let len = if c.is_ascii() {
                                    1
                                } else {
                                    utils::char_len(c) as i32
                                };
                                let text_len = t.text().len() as i32;
                                buf.remove(text_len - len, text_len);
                                sbuf.remove(text_len - len, text_len);
                                true
                            } else {
                                false
                            }
                        }
                        _ => {
                            if let Some(ch) = app::event_text().chars().next() {
                                if app::compose().is_some() {
                                    let temp = ch.to_string();
                                    cmd.push_str(&temp);
                                    t.append(&temp);
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                    },
                    Event::KeyUp => {
                        if app::event_state() == Shortcut::Ctrl {
                            let key = app::event_key();
                            if key != Key::ControlL && key != Key::ControlR {
                                if let Some(ch) = char::from_u32(key.bits() as u32 - 96) {
                                    writer.write_all(&[ch as u8]).unwrap();
                                }
                            }
                        }
                        false
                    }
                    _ => false,
                }
            });
            Self { st }
        }
    }

    fltk::widget_extends!(AnsiTerm, text::SimpleTerminal, st);
    fn format(msg: &[u8], st: &mut text::SimpleTerminal) {
        // handles the sticky title-bell sequence
        if let Some(pos0) = msg.windows(4).position(|m| m == b"\x1b]0;") {
            let mut pos1 = pos0;
            while pos1 < msg.len() && msg[pos1] != b'[' {
                pos1 += 1;
            }
            st.append2(&msg[0..pos0]);
            st.append2(&msg[pos1 - 1..]);
        } else {
            st.append2(msg);
        }
    }
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

use fltk::image::IcoImage;

fn main() {
    use fltk::{prelude::*, *};
    let a = app::App::default();
    let mut w = window::Window::default().with_size(WIDTH, HEIGHT);
    let icon: IcoImage = IcoImage::load(&std::path::Path::new("src/fltk.ico")).unwrap();
    w.make_resizable(true);
    w.set_icon(Some(icon));
    crate::term::AnsiTerm::default()
        .with_size(WIDTH - 4, HEIGHT - 4)
        .center_of_parent();
    w.end();
    w.show();
    a.run().unwrap();
}
