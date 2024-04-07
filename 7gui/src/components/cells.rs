use fltk::{app::*, draw::*, enums::*, input::*, prelude::*, table::*, window::*};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace0, not_line_ending};
use nom::combinator::{all_consuming, cut, eof, map, verify};
use nom::multi::separated_list1;
use nom::number::complete::float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use std::borrow::Cow;
use std::cell::RefCell;
use std::char;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

struct Row(usize);

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

struct Col(usize);

impl fmt::Display for Col {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            char::from_u32(u32::from('A') + u32::try_from(self.0).unwrap()).unwrap()
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}
impl Coord {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let alpha = map(alpha1, |s| {
            usize::from_str_radix(s, 36).unwrap() - usize::from_str_radix("A", 36).unwrap()
        });
        let num = verify(map(digit1, |s| usize::from_str(s).unwrap()), |num| *num > 0);
        map(tuple((alpha, num)), |(col, row)| Self::new(row - 1, col))(input)
    }
}

#[derive(Debug, PartialEq)]
enum Formula {
    Empty,
    Coord(Coord),
    Range(Coord, Coord),
    Number(f32),
    Label(String),
    Application(String, Vec<Formula>),
    Invalid,
}
impl Formula {
    fn parse(input: &str) -> nom::IResult<&str, Formula> {
        alt((
            preceded(
                tag("="),
                cut(all_consuming(preceded(multispace0, Self::parse_expr))),
            ),
            map(eof, |_| Formula::Empty),
            map(all_consuming(float), Formula::Number),
            map(all_consuming(not_line_ending), |s: &str| {
                Formula::Label(s.to_string())
            }),
        ))(input)
    }
    fn parse_expr(input: &str) -> nom::IResult<&str, Formula> {
        alt((
            map(
                separated_pair(Coord::parse, tag(":"), Coord::parse),
                |(start, end)| Formula::Range(start, end),
            ),
            map(Coord::parse, Formula::Coord),
            map(float, Formula::Number),
            map(
                pair(
                    alpha1,
                    terminated(
                        delimited(
                            preceded(multispace0, tag("(")),
                            separated_list1(
                                preceded(multispace0, tag(",")),
                                preceded(multispace0, Self::parse_expr),
                            ),
                            preceded(multispace0, tag(")")),
                        ),
                        multispace0,
                    ),
                ),
                |(name, args)| Formula::Application(name.to_string(), args),
            ),
        ))(input)
    }
    fn references(&self) -> Vec<Coord> {
        match self {
            Formula::Coord(coord) => vec![*coord],
            Formula::Range(start, end) => {
                let mut refs = vec![];
                for col in min(start.col, end.col)..=max(start.col, end.col) {
                    for row in min(start.row, end.row)..=max(start.row, end.row) {
                        refs.push(Coord::new(row, col));
                    }
                }
                refs
            }
            Formula::Application(_func, args) => {
                args.iter().flat_map(|arg| arg.references()).collect()
            }
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CellError;

#[derive(Debug)]
struct Cell {
    text: String,
    formula: Formula,
    value: Result<f32, CellError>,
    subscribers: HashSet<Coord>,
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            text: String::new(),
            formula: Formula::Empty,
            value: Ok(f32::NAN),
            subscribers: HashSet::new(),
        }
    }
}

struct Spreadsheet {
    cells: HashMap<Coord, Cell>,
    default_cell: Cell,
}
impl Spreadsheet {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            default_cell: Cell::default(),
        }
    }
    fn cell(&self, coord: Coord) -> &Cell {
        self.cells.get(&coord).unwrap_or(&self.default_cell)
    }
    fn set_text(&mut self, coord: Coord, text: &str) {
        let new_formula = Formula::parse(text)
            .map(|(_, output)| output)
            .unwrap_or(Formula::Invalid);

        // Unsubscribe cells referenced by the old formula
        let old_formula = &self.cell(coord).formula;
        for referenced_coord in old_formula.references() {
            let referenced_cell = self.cells.entry(referenced_coord).or_default();
            referenced_cell.subscribers.remove(&coord);
        }

        // Subscribe cells referenced by the new formula
        for referenced_coord in new_formula.references() {
            let referenced_cell = self.cells.entry(referenced_coord).or_default();
            referenced_cell.subscribers.insert(coord);
        }

        let cell = self.cells.entry(coord).or_default();
        cell.text = text.to_string();
        cell.formula = new_formula;

        self.update_value(coord);
    }
    fn update_value(&mut self, coord: Coord) {
        let cell = self.cell(coord);
        let old_value = cell.value;
        let new_value = self.evaluate(&cell.formula);
        if old_value != new_value {
            let cell = self.cells.entry(coord).or_default();
            cell.value = new_value;

            // Update cells that are subscribed to this cell
            let subscribers = cell.subscribers.iter().copied().collect::<Vec<Coord>>();
            for subscribed_coord in subscribers {
                self.update_value(subscribed_coord);
            }
        }
    }
    fn evaluate(&self, formula: &Formula) -> Result<f32, CellError> {
        match formula {
            Formula::Empty => Ok(f32::NAN),
            Formula::Coord(coord) => Ok(self.cell(*coord).value?),
            Formula::Range(_start, _end) => Err(CellError),
            Formula::Number(number) => Ok(*number),
            Formula::Label(_) => Ok(f32::NAN),
            Formula::Application(func, args) => {
                let mut values: Vec<f32> = Vec::new();
                for arg in args {
                    for arg_val in self.evaluate_list(arg)? {
                        values.push(arg_val);
                    }
                }
                let is_binary_op = values.len() == 2;
                match func.to_ascii_lowercase().as_ref() {
                    "add" if is_binary_op => Ok(values[0] + values[1]),
                    "sub" if is_binary_op => Ok(values[0] - values[1]),
                    "div" if is_binary_op => Ok(values[0] / values[1]),
                    "mul" if is_binary_op => Ok(values[0] * values[1]),
                    "mod" if is_binary_op => Ok(values[0] % values[1]),
                    "sum" => Ok(values.iter().sum()),
                    "prod" => Ok(values.iter().product()),
                    _ => Err(CellError),
                }
            }
            Formula::Invalid => Err(CellError),
        }
    }
    fn evaluate_list(&self, formula: &Formula) -> Result<Vec<f32>, CellError> {
        match formula {
            Formula::Range(_start, _end) => formula
                .references()
                .iter()
                .map(|coord| self.cell(*coord).value)
                .collect(),
            formula => Ok(vec![self.evaluate(formula)?]),
        }
    }
}

struct SpreadsheetWidgetInner {
    table: Table,
    input: Input,
    sheet: Spreadsheet,
    edit_cell: std::option::Option<Coord>,
}

impl SpreadsheetWidgetInner {
    fn start_editing(&mut self, row: i32, col: i32) {
        self.edit_cell = Some(Coord::new(row.try_into().unwrap(), col.try_into().unwrap()));
    }
    fn finish_editing(&mut self) {
        if let Some(edit_cell) = self.edit_cell {
            let text = self.input.value();
            self.sheet.set_text(edit_cell, &text);
            self.edit_cell = None;
            self.input.hide();
            // Prevent mouse cursor from remaining hidden after pressing Enter.
            self.input.window().unwrap().set_cursor(Cursor::Default);
        }
    }
    fn draw_cell(
        &mut self,
        table_context: TableContext,
        row: i32,
        col: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        push_clip(x, y, width, height);
        fltk::draw::set_font(Font::Helvetica, 16);
        match table_context {
            TableContext::RowHeader => {
                let color = self.table.row_header_color();
                draw_box(FrameType::GtkThinUpBox, x, y, width, height, color);
                set_draw_color(Color::Black);
                draw_text2(
                    &Row(row.try_into().unwrap()).to_string(),
                    x,
                    y,
                    width,
                    height,
                    Align::Bottom | Align::Center,
                );
            }
            TableContext::ColHeader => {
                let color = self.table.col_header_color();
                draw_box(FrameType::GtkThinUpBox, x, y, width, height, color);
                set_draw_color(Color::Black);
                draw_text2(
                    &Col(col.try_into().unwrap()).to_string(),
                    x,
                    y,
                    width,
                    height,
                    Align::Bottom | Align::Center,
                );
            }
            TableContext::Cell => {
                let coord = Coord::new(row.try_into().unwrap(), col.try_into().unwrap());
                if self.edit_cell == Some(coord) {
                    self.input.resize(x, y, width, height);
                    self.input.show();
                    self.input.set_value(&self.sheet.cell(coord).text);
                    self.input.take_focus().expect("input refused focus");
                    self.input.redraw();
                } else {
                    let color = if self.table.is_selected(row, col) {
                        Color::from_u32(0x0099ff)
                    } else {
                        Color::BackGround2
                    };
                    let cell = self.sheet.cell(coord);
                    let text = match (cell.value, &cell.formula) {
                        (Err(_), _) => Cow::from("ERROR"),
                        (Ok(_), Formula::Label(_)) => Cow::from(&cell.text),
                        (Ok(_), Formula::Empty) => Cow::from(""),
                        (Ok(value), _) => Cow::from(value.to_string()),
                    };
                    draw_box(FrameType::GtkThinUpBox, x, y, width, height, color);
                    set_draw_color(Color::Black);
                    draw_text2(&text, x, y, width, height, Align::Bottom | Align::Left);
                }
            }
            _ => {}
        }
        pop_clip();
    }
}

struct SpreadsheetWidget {
    _inner: Rc<RefCell<SpreadsheetWidgetInner>>,
}

impl SpreadsheetWidget {
    fn new(x: i32, y: i32, width: i32, height: i32, sheet: Spreadsheet) -> Self {
        let mut table = Table::new(x, y, width, height, "");
        table.end(); // Table implicitly calls begin.
        table.set_row_header(true);
        table.set_col_header(true);
        table.set_rows(100);
        table.set_cols(26);

        let mut cell_input = Input::default().with_size(75, 25).with_pos(0, 0);
        cell_input.set_trigger(CallbackTrigger::EnterKeyAlways);
        cell_input.hide();

        let inner = Rc::new(RefCell::new(SpreadsheetWidgetInner {
            table,
            input: cell_input,
            sheet,
            edit_cell: None,
        }));

        let inner_clone = inner.clone();
        inner.borrow_mut().input.set_callback(move |_| {
            inner_clone.borrow_mut().finish_editing();
        });

        let inner_clone = inner.clone();
        inner.borrow_mut().table.draw_cell(
            move |_, table_context, row, col, x, y, width, height| {
                inner_clone
                    .borrow_mut()
                    .draw_cell(table_context, row, col, x, y, width, height);
            },
        );

        let inner_clone = inner.clone();
        inner.borrow_mut().table.handle(move |widget, event| {
            if event == Event::Push && widget.callback_context() == TableContext::Cell {
                inner_clone.borrow_mut().finish_editing();
                if event_clicks() {
                    inner_clone
                        .borrow_mut()
                        .start_editing(widget.callback_row(), widget.callback_col());
                }
                widget.redraw();
                true
            } else {
                false
            }
        });

        Self { _inner: inner }
    }
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default().with_label("Cells");
    let sheet = Spreadsheet::new();
    let _sheet_widget = SpreadsheetWidget::new(0, 0, 400, 200, sheet);
    wind.set_size(400, 200);
    wind.end();
    wind.show();
    app.run().unwrap();
}

#[cfg(test)]
mod tests {
    use super::{CellError, Coord, Formula, Spreadsheet};

    macro_rules! spreadsheet(
        { $($key:ident => $value:expr),+ } => {
            {
                let mut s = Spreadsheet::new();
                $(s.set_text(Coord::parse(stringify!($key)).unwrap().1, &$value.to_string());)+
                s
            }
         };
    );

    macro_rules! coord(
        ( $c:ident ) => { Coord::parse(stringify!($c)).unwrap().1 }
    );

    #[test]
    fn coord_parsing_works() {
        assert!(Coord::parse("A0").is_err());
        assert_eq!(Coord::parse("a1"), Ok(("", Coord::new(0, 0))));
        assert_eq!(Coord::parse("A1"), Ok(("", Coord::new(0, 0))));
        assert_eq!(Coord::parse("B2"), Ok(("", Coord::new(1, 1))));
        assert_eq!(Coord::parse("Z99"), Ok(("", Coord::new(98, 25))));
    }

    #[test]
    fn formula_references_works() {
        assert_eq!(Formula::Number(1.0).references(), []);
        assert_eq!(Formula::Coord(coord!(A1)).references(), [coord!(A1)]);
        assert_eq!(
            Formula::Range(coord!(A1), coord!(B2)).references(),
            [coord!(A1), coord!(A2), coord!(B1), coord!(B2)]
        );
        assert_eq!(
            Formula::Range(coord!(B2), coord!(A1)).references(),
            [coord!(A1), coord!(A2), coord!(B1), coord!(B2)]
        );
        assert_eq!(
            Formula::Application(
                "sum".to_string(),
                vec![
                    Formula::Coord(coord!(A1)),
                    Formula::Range(coord!(B1), coord!(C1))
                ]
            )
            .references(),
            [coord!(A1), coord!(B1), coord!(C1)]
        );
    }

    #[test]
    fn formula_parsing_works() {
        assert_eq!(Formula::parse("1.0"), Ok(("", Formula::Number(1.0))));
        assert_eq!(Formula::parse("1.1"), Ok(("", Formula::Number(1.1))));
        assert_eq!(Formula::parse("10"), Ok(("", Formula::Number(10.0))));
        assert_eq!(
            Formula::parse("1.0foo"),
            Ok(("", Formula::Label("1.0foo".to_string())))
        );
        assert_eq!(
            Formula::parse("foo"),
            Ok(("", Formula::Label("foo".to_string())))
        );
        assert_eq!(Formula::parse(""), Ok(("", Formula::Empty)));
        assert_eq!(Formula::parse("=1.0"), Ok(("", Formula::Number(1.0))));
        assert!(Formula::parse("=1.0foo").is_err());
        assert_eq!(Formula::parse("=A1"), Ok(("", Formula::Coord(coord!(A1)))));
        assert_eq!(Formula::parse("=A2"), Ok(("", Formula::Coord(coord!(A2)))));
        assert_eq!(
            Formula::parse("=A2:C4"),
            Ok(("", Formula::Range(coord!(A2), coord!(C4))))
        );
        assert_eq!(
            Formula::parse("=sum(A1,sub(A2,1.0))"),
            Ok((
                "",
                Formula::Application(
                    "sum".to_string(),
                    vec![
                        Formula::Coord(coord!(A1)),
                        Formula::Application(
                            "sub".to_string(),
                            vec![Formula::Coord(coord!(A2)), Formula::Number(1.0),],
                        )
                    ],
                )
            ))
        );
        assert_eq!(
            Formula::parse("= sum ( A1 , A2 ) "),
            Ok((
                "",
                Formula::Application(
                    "sum".to_string(),
                    vec![Formula::Coord(coord!(A1)), Formula::Coord(coord!(A2))]
                )
            ))
        );
    }

    #[test]
    fn spreadsheet_works() {
        let mut sheet = Spreadsheet::new();

        assert!(sheet.cell(coord!(A1)).value.unwrap().is_nan());
        assert_eq!(sheet.cell(coord!(A1)).text, "");

        sheet.set_text(coord!(A1), "");
        assert!(sheet.cell(coord!(A1)).value.unwrap().is_nan());
        assert_eq!(sheet.cell(coord!(A1)).text, "");

        sheet.set_text(coord!(A1), "foobar");
        assert!(sheet.cell(coord!(A1)).value.unwrap().is_nan());
        assert_eq!(sheet.cell(coord!(A1)).text, "foobar");

        sheet.set_text(coord!(A1), "1.1");
        assert_eq!(sheet.cell(coord!(A1)).value, Ok(1.1));
        assert_eq!(sheet.cell(coord!(A1)).text, "1.1");

        sheet.set_text(coord!(A1), "=1.1");
        assert_eq!(sheet.cell(coord!(A1)).value, Ok(1.1));
        assert_eq!(sheet.cell(coord!(A1)).text, "=1.1");

        sheet.set_text(coord!(A1), "2.0");
        sheet.set_text(coord!(B2), "=A1");
        assert_eq!(sheet.cell(coord!(B2)).value, Ok(2.0));

        sheet.set_text(coord!(B2), "=A2");
        assert!(sheet.cell(coord!(B2)).value.unwrap().is_nan());

        sheet.set_text(coord!(A1), "=FOOBAR()");
        assert_eq!(sheet.cell(coord!(A1)).value, Err(CellError));

        sheet.set_text(coord!(B2), "=A1");
        assert_eq!(sheet.cell(coord!(B2)).value, Err(CellError));
    }

    #[test]
    fn function_with_wrong_arguments_returns_error() {
        let sheet = spreadsheet! {A1 => "=ADD(A2)"};
        assert_eq!(sheet.cell(coord!(A1)).value, Err(CellError));
    }

    #[test]
    fn function_add_works() {
        let sheet = spreadsheet! {A1 => 3, B1 => 5, C1 => "=ADD(A1,B1)"};
        assert_eq!(sheet.cell(coord!(C1)).value, Ok(8.0));
    }

    #[test]
    fn function_sub_works() {
        let sheet = spreadsheet! {A1 => 3, B1 => 5, C1 => "=SUB(A1,B1)"};
        assert_eq!(sheet.cell(coord!(C1)).value, Ok(-2.0));
    }

    #[test]
    fn function_div_works() {
        let sheet = spreadsheet! {A1 => 10, B1 => 2, C1 => "=DIV(A1,B1)"};
        assert_eq!(sheet.cell(coord!(C1)).value, Ok(5.0));
    }

    #[test]
    fn function_mul_works() {
        let sheet = spreadsheet! {A1 => 10, B1 => 2, C1 => "=MUL(A1,B1)"};
        assert_eq!(sheet.cell(coord!(C1)).value, Ok(20.0));
    }

    #[test]
    fn function_mod_works() {
        let sheet = spreadsheet! {A1 => 22, B1 => 10, C1 => "=MOD(A1,B1)"};
        assert_eq!(sheet.cell(coord!(C1)).value, Ok(2.0));
    }

    #[test]
    fn function_sum_works() {
        let sheet = spreadsheet! {
            A1 => 1, B1 => 2, C1 => 3,
            A2 => "=SUM(A1,B1,C1)"
        };
        assert_eq!(sheet.cell(coord!(A2)).value, Ok(6.0));
    }

    #[test]
    fn function_prod_works() {
        let sheet = spreadsheet! {
            A1 => 3, B1 => 5, C1 => 7,
            A2 => "=PROD(A1,B1,C1)"
        };
        assert_eq!(sheet.cell(coord!(A2)).value, Ok(105.0));
    }

    #[test]
    fn application_with_range_argument_works() {
        let sheet = spreadsheet! {
            A1 => 1, B1 => 2, C1 => 3,
            A2 => "=SUM(A1:C1)"
        };
        assert_eq!(sheet.cell(coord!(A2)).value, Ok(6.0));
    }

    #[test]
    fn application_with_application_argument_works() {
        let sheet = spreadsheet! {
            A1 => 1, B1 => 2, C1 => 3,
            A2 => "=PROD(A1,SUM(B1,C1))"
        };
        assert_eq!(sheet.cell(coord!(A2)).value, Ok(5.0));
    }

    #[test]
    fn nested_application_works() {
        let sheet = spreadsheet! {A1 => 3, B1 => 5, C1 => "=PROD(A1,SUM(A1,B1))"};
        assert_eq!(sheet.cell(coord!(C1)).value, Ok(24.0));
    }

    #[test]
    fn changes_propagate() {
        let mut sheet = spreadsheet! {A1 => 1, B1 => "=A1"};
        sheet.set_text(coord!(A1), "2");
        assert_eq!(sheet.cell(coord!(B1)).value, Ok(2.0));
    }

    #[test]
    fn old_cell_references_are_removed() {
        let mut sheet = spreadsheet! {A1 => 1, B1 => "=A1"};
        sheet.set_text(coord!(B1), "2");
        assert!(sheet.cells.get(&coord!(A1)).unwrap().subscribers.is_empty())
    }
}
