use chrono::{DateTime, Datelike, Local, NaiveDate};
use fltk::{app, draw, enums::*, menu, prelude::*, table, window};
use std::{cell::RefCell, rc::Rc};

/// Defines a calendar dialog
pub struct Calendar {
    wind: window::Window,
    table: table::TableRow,
    month_choice: menu::Choice,
    year_choice: menu::Choice,
}

impl Calendar {
    /// Creates a new calendar dialog
    pub fn new(x: i32, y: i32) -> Self {
        // get today's date
        let local: DateTime<Local> = Local::now();
        let curr = (local.month() - 1) as i32;
        let curr_year = local.year();
        // create window with month and year choice widgets
        let mut wind = window::Window::new(x, y, 400, 300, "Calendar");
        let mut month_choice = menu::Choice::new(100, 5, 80, 40, "");
        month_choice.add_choice("Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec");
        month_choice.set_value(curr);
        let mut year_choice = menu::Choice::new(220, 5, 80, 40, "");
        for i in 1900..curr_year + 1 {
            year_choice.add_choice(&format!("{}", i));
        }
        year_choice.set_value(curr_year - 1900);
        // Create a table with the days of the selected month
        let mut table = table::TableRow::new(5, 50, 390, 250, "");
        table.set_type(table::TableRowSelectMode::Single);
        table.set_rows(5);
        table.set_cols(7);
        table.set_col_header(true);
        table.set_col_width_all(table.width() / 7);
        table.set_row_height_all(table.height() / 4 - table.col_header_height());
        table.end();
        wind.end();
        wind.make_modal(true);
        wind.show();

        let curr = Rc::from(RefCell::from(curr + 1));
        let curr_year = Rc::from(RefCell::from(curr_year));

        let curr_rc = curr.clone();
        let curr_year_rc = curr_year.clone();
        table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
            table::TableContext::ColHeader => {
                let day = match col + 1 {
                    1 => "Mon",
                    2 => "Tue",
                    3 => "Wed",
                    4 => "Thu",
                    5 => "Fri",
                    6 => "Sat",
                    7 => "Sun",
                    _ => unreachable!(),
                };
                draw_header(day, x, y, w, h)
            }
            table::TableContext::Cell => {
                let day = row * 7 + col + 1;
                let max_days = match *curr_rc.borrow() {
                    1 => 31,
                    2 => {
                        if *curr_year_rc.borrow() % 4 == 0 {
                            29
                        } else {
                            28
                        }
                    }
                    3 => 31,
                    4 => 30,
                    5 => 31,
                    6 => 30,
                    7 => 31,
                    8 => 31,
                    9 => 30,
                    10 => 31,
                    11 => 30,
                    12 => 31,
                    _ => unreachable!(),
                };
                if day < (max_days + 1) {
                    draw_data(day, x, y, w, h, t.is_selected(row, col));
                }
            }
            _ => (),
        });

        let curr_rc = curr;
        // redraw table when the month changes
        month_choice.set_callback(move |c| {
            *curr_rc.borrow_mut() = c.value() + 1;
            c.parent().unwrap().redraw();
        });

        let curr_year_rc = curr_year;
        // redraw table when the year changes
        year_choice.set_callback(move |c| {
            *curr_year_rc.borrow_mut() = c.value() + 1900;
            c.parent().unwrap().redraw();
        });

        // choose the day by double clicking a cell
        table.handle(|t, ev| {
            if ev == Event::Push && app::event_clicks() {
                t.top_window().unwrap().hide();
                true
            } else {
                false
            }
        });

        // Keep the window shown awaiting input
        while wind.shown() {
            app::wait();
        }

        Self {
            wind,
            table,
            month_choice,
            year_choice,
        }
    }
    /// Creates a new calendar widget with a default position
    pub fn default() -> Self {
        let mut s = Self::new(0, 0);
        s.wind.free_position();
        s
    }
    /// Get the date selected by the calendar dialog
    pub fn get_date(&self) -> Option<chrono::naive::NaiveDate> {
        // get table selection
        let (r, c, _, _) = self.table.get_selection();
        if r == -1 || c == -1 {
            None
        } else {
            let day = r * 7 + c + 1;
            NaiveDate::from_ymd_opt(
                self.year_choice.value() + 1900,
                self.month_choice.value() as u32 + 1,
                day as u32,
            )
        }
    }
}

// draw header with day names
fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::pop_clip();
}

// draw the numbers
fn draw_data(day: i32, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xbcd9ea));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(&format!("{}", day), x, y, w, h, Align::Center);
    draw::pop_clip();
}
