use fltk::*;
use serde::{Deserialize, Serialize};
use std::*;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

struct FlatButton {
    frm: frame::Frame,
}

const RED: u32 = 0xe51c23;
const GREEN: u32 = 0x8bc34a;

impl FlatButton {
    pub fn new(w: i32, h: i32, title: &str) -> FlatButton {
        let mut w = FlatButton {
            frm: frame::Frame::new(0, 0, w, h, title),
        };
        w.frm.set_frame(FrameType::RFlatBox);
        w.frm.set_color(Color::Red);
        let mut w_c = w.clone();
        w.frm.handle(Box::new(move |ev| match ev {
            Event::Push => {
                if w_c.color() == Color::from_u32(GREEN) {
                    w_c.set_color(Color::from_u32(RED));
                } else {
                    w_c.set_color(Color::from_u32(GREEN));
                }
                w_c.redraw();
                true
            }
            _ => false,
        }));
        w
    }
}

impl ops::Deref for FlatButton {
    type Target = frame::Frame;

    fn deref(&self) -> &Self::Target {
        &self.frm
    }
}

impl ops::DerefMut for FlatButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frm
    }
}

#[tokio::main]
async fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::DoubleWindow::new(200, 200, 600, 400, "Todos");
    let mut scroll = group::Scroll::default().with_size(600, 350);
    let mut pack = group::Pack::default().with_size(580, 350).center_of(&scroll);
    pack.end();
    scroll.end();

    let mut choice = menu::Choice::new(260, 355, 80, 40, "User");

    win.end();
    win.show();

    app::background(255, 255, 255);
    pack.set_spacing(5);
    choice.set_color(Color::from_u32(0x673ab7));
    choice.set_text_color(Color::White);
    scroll.set_scrollbar_size(7);
    scroll.set_type(group::ScrollType::Vertical);
    let mut scrollbar = scroll.scrollbar();
    scrollbar.set_type(valuator::ScrollbarType::VerticalNice);
    scrollbar.set_color(Color::from_u32(0x757575));
    scrollbar.set_selection_color(Color::Red);

    for user in 1..=10 {
        let pack = pack.clone();
        let win = win.clone();
        choice.add(
            &user.to_string(),
            Shortcut::None,
            menu::MenuFlag::Normal,
            Box::new(move || {
                let mut pack = pack.clone();
                let mut win = win.clone();
                tokio::spawn(async move {
                    let resp = reqwest::get(&format!(
                        "https://jsonplaceholder.typicode.com/todos?userId={}",
                        user
                    ))
                    .await
                    .unwrap()
                    .json::<Vec<Item>>()
                    .await
                    .unwrap();
                    pack.clear();
                    for item in resp {
                        if item.user_id == user {
                            let mut frm = FlatButton::new(580, 100, &item.title);
                            if item.completed {
                                frm.set_color(Color::from_u32(GREEN));
                            } else {
                                frm.set_color(Color::from_u32(RED));
                            }
                            pack.add(&*frm);
                        }
                    }
                    win.redraw();
                });
            }),
        );
    }

    app.run().unwrap();
}
