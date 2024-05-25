#![forbid(unsafe_code)]

use {
    fltk::{
        app,
        button::Button,
        dialog::alert_default,
        draw,
        enums::{Align, Event, FrameType},
        frame::Frame,
        group::Flex,
        menu::{MenuButton, MenuButtonType},
        prelude::*,
        window::Window,
    },
    fltk_theme::{color_themes, ColorTheme},
};

const HEIGHT: i32 = 30;
const PAD: i32 = 10;

#[derive(Debug, Clone)]
struct Model {
    grid: [[i32; 9]; 9],
}

impl Model {
    fn solvable(&self) -> bool {
        let mut items: [i32; 9];

        for row in self.grid {
            items = [0; 9];
            for value in row {
                if value > 0 && value < 10 {
                    items[(value - 1) as usize] += 1;
                }
            }
            if items.iter().any(|&n| n > 1) {
                return false;
            }
        }

        for i in 0..9 {
            items = [0; 9];
            for row in self.grid {
                if row[i] > 0 && row[i] < 10 {
                    items[(row[i] - 1) as usize] += 1;
                }
            }
            if items.iter().any(|&n| n > 1) {
                return false;
            }
        }

        for &x in [0, 3, 6].iter() {
            for &y in [0, 3, 6].iter() {
                items = [0; 9];
                for i in 0..3 {
                    for j in 0..3 {
                        if self.grid[y + i][x + j] > 0 && self.grid[y + i][x + j] < 10 {
                            items[(self.grid[y + i][x + j] - 1) as usize] += 1;
                        }
                    }
                }
                if items.iter().any(|&n| n > 1) {
                    return false;
                }
            }
        }
        true
    }

    fn possible(&self, y: usize, x: usize, number: i32) -> bool {
        if self.grid[y].iter().any(|&n| n == number) {
            return false;
        }

        if self.grid.iter().any(|n| n[x] == number) {
            return false;
        }

        let x0: usize = (x / 3) * 3;
        let y0: usize = (y / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                if self.grid[y0 + i][x0 + j] == number {
                    return false;
                }
            }
        }
        true
    }

    fn find_next_cell2fill(&self) -> (usize, usize) {
        for (x, row) in self.grid.iter().enumerate() {
            for (y, &val) in row.iter().enumerate() {
                if val == 0 {
                    return (x, y);
                }
            }
        }
        (99, 99)
    }

    fn solve(&mut self) -> bool {
        let (i, j) = self.find_next_cell2fill();
        if i == 99 {
            return true;
        }
        for e in 1..10 {
            if self.possible(i, j, e) {
                self.grid[i][j] = e;
                if self.solve() {
                    return true;
                }
                self.grid[i][j] = 0;
            }
        }
        false
    }
}

fn main() -> Result<(), FltkError> {
    let grid =
        app::GlobalState::<Model>::new(Model { grid: [[0; 9]; 9] }).with(move |model| model.grid);
    let app = app::App::default();
    let menu = crate::menu();
    let mut window = crate::window();
    let mut page = Flex::default_fill().column(); //Page
    {
        let mut header = Flex::default();
        Button::default()
            .with_label("Solve")
            .set_callback(crate::solve);
        Button::default()
            .with_label("Clear")
            .set_callback(crate::clear);
        header.end();
        header.set_pad(PAD);
        page.fixed(&header, HEIGHT);
    }
    {
        let mut hero = Flex::default_fill().column();
        for (x, row) in grid.iter().enumerate() {
            let mut hbox = Flex::default();
            for (y, _) in row.iter().enumerate() {
                let menu = menu.clone();
                crate::frame(&mut hbox, x, y).handle(move |frame, event| match event {
                    Event::Push => match app::event_mouse_button() {
                        app::MouseButton::Right => {
                            if let Some(item) = menu.popup() {
                                frame.set_label(&item.label().unwrap());
                                frame.do_callback();
                            };
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                });
            }
            hbox.end();
            hbox.set_pad(0);
            hero.fixed(&hbox, HEIGHT);
        }
        hero.set_pad(0);
    }
    page.end();
    page.set_pad(PAD);
    page.set_margin(PAD);
    page.set_frame(FrameType::FlatBox);
    window.end();
    window.show();
    app.run()
}

fn clear(button: &mut Button) {
    button.deactivate();
    app::GlobalState::<Model>::get().with(move |model| {
        for (x, row) in model.grid.clone().iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                model.grid[x][y] = 0;
            }
        }
    });
    app::redraw();
    button.activate();
}

fn solve(button: &mut Button) {
    button.deactivate();
    app::GlobalState::<Model>::get().with(move |model| {
        if model.solvable() {
            model.solve();
            for (x, row) in model.grid.clone().iter().enumerate() {
                for (y, _) in row.iter().enumerate() {
                    if model.grid[x][y] == 0 {
                        alert_default("Not solvable");
                        break;
                    }
                }
            }
        } else {
            alert_default("Not solvable");
        };
    });
    button.activate();
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default().with_type(MenuButtonType::Popup3);
    element.add_choice("1|2|3|4|5|6|7|8|9");
    element.set_text_size(HEIGHT);
    element
}

fn frame(flex: &mut Flex, x: usize, y: usize) -> Frame {
    let mut element = Frame::default();
    element.set_frame(FrameType::DownBox);
    element.set_label_size(HEIGHT);
    element.set_callback(move |frame| {
        if let Ok(value) = frame.label().parse::<i32>() {
            app::GlobalState::<Model>::get().with(move |model| model.grid[x][y] = value);
        }
    });
    element.draw(move |frame| {
        let value = app::GlobalState::<Model>::get().with(move |model| model.grid[x][y]);
        let binding = value.to_string();
        draw::draw_text2(
            match value == 0 {
                true => "",
                false => &binding,
            },
            frame.x(),
            frame.y(),
            frame.w(),
            frame.h(),
            Align::Center,
        );
    });
    flex.fixed(&element, HEIGHT);
    element
}

fn window() -> Window {
    const NAME: &str = "FlSudoku";
    let mut element = Window::default()
        .with_size(290, 330)
        .with_label(NAME)
        .center_screen();
    element.set_xclass(NAME);
    element.make_resizable(false);
    element.set_callback(move |_| {
        if app::event() == Event::Close {
            app::quit();
        }
    });
    ColorTheme::new(color_themes::DARK_THEME).apply();
    element
}
