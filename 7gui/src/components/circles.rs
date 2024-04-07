use {
    crate::Message,
    fltk::{
        app::*, button::*, draw::*, enums::*, group::*, menu::*, prelude::*, valuator::*, widget::*,
    },
    std::{
        cell::RefCell,
        collections::HashMap,
        ops::{Deref, DerefMut},
        rc::Rc,
    },
};

const WIDGET_PADDING: i32 = 10;

const CANVAS_WIDTH: i32 = 350;
const CANVAS_HEIGHT: i32 = 250;

const MOUSE_LEFT: i32 = 1;
const MOUSE_RIGHT: i32 = 3;

type Coord = (i32, i32);
type Radius = i32;

fn distance(a: Coord, b: Coord) -> i32 {
    f64::sqrt(f64::from(i32::pow(a.0 - b.0, 2) + i32::pow(a.1 - b.1, 2))) as i32
}

pub struct Model {
    sender: Sender<Message>,
    states: Vec<HashMap<Coord, Radius>>,
    current_state_index: usize,
}
impl Model {
    fn build(sender: Sender<Message>) -> Self {
        Self {
            sender,
            states: vec![HashMap::new()],
            current_state_index: 0,
        }
    }
    pub fn save(&mut self) {
        for _ in self.current_state_index + 1..self.states.len() {
            self.states.pop();
        }
        self.states
            .push(self.states[self.current_state_index].clone());
        self.current_state_index += 1;
    }
    pub fn undo(&mut self) {
        assert!(self.can_undo());
        self.current_state_index -= 1;
        self.sender.send(Message::ModelChanged);
    }
    fn can_undo(&self) -> bool {
        self.current_state_index > 0
    }
    pub fn redo(&mut self) {
        assert!(self.can_redo());
        self.current_state_index += 1;
        self.sender.send(Message::ModelChanged);
    }
    fn can_redo(&self) -> bool {
        self.current_state_index + 1 < self.states.len()
    }
    pub fn set(&mut self, coord: Coord, radius: Radius) {
        self.states[self.current_state_index].insert(coord, radius);
        self.sender.send(Message::ModelChanged);
    }
    fn circles(&self) -> &HashMap<Coord, Radius> {
        &self.states[self.current_state_index]
    }
}

struct Canvas {
    widget: Widget,
    circles: Rc<RefCell<HashMap<Coord, Radius>>>,
    selected: Rc<RefCell<std::option::Option<(Coord, Radius)>>>,
}

impl Canvas {
    fn new(x: i32, y: i32, width: i32, height: i32, sender: Sender<Message>) -> Self {
        let mut canvas = Canvas {
            widget: Widget::new(x, y, width, height, ""),
            circles: Rc::default(),
            selected: Rc::default(),
        };
        canvas.widget.set_trigger(CallbackTrigger::Release);

        let circles = canvas.circles.clone();
        let selected = canvas.selected.clone();
        canvas.widget.handle(move |widget, event| {
            let event_pos = (event_x() - widget.x(), event_y() - widget.y());
            match event {
                Event::Enter => true,
                Event::Move => {
                    let new_selection = circles
                        .borrow()
                        .iter()
                        .map(|(pos, radius)| (*pos, *radius))
                        .filter(|(pos, radius)| distance(*pos, event_pos) <= *radius)
                        .min_by_key(|(pos, _radius)| distance(*pos, event_pos));
                    if new_selection != *selected.borrow() {
                        widget.redraw();
                        selected.replace(new_selection);
                    }
                    true
                }
                Event::Released if event_button() == MOUSE_LEFT => {
                    if selected.borrow().is_none() {
                        sender.send(Message::Add(event_pos));
                    }
                    true
                }
                Event::Released if event_button() == MOUSE_RIGHT => {
                    let selected = *selected.borrow(); // Limit borrow lifetime.
                    if selected.is_some() {
                        let menu = MenuItem::new(&["Adjust diameter..."]);
                        if menu.popup(event_x(), event_y()).is_some() {
                            sender.send(Message::AdjustOpened);
                        }
                    }
                    true
                }
                _ => false,
            }
        });

        let circles = canvas.circles.clone();
        let selected = canvas.selected.clone();
        canvas.widget.draw(move |wid| {
            push_clip(wid.x(), wid.y(), wid.width(), wid.height());
            draw_rect_fill(wid.x(), wid.y(), wid.width(), wid.height(), Color::White);
            for (pos, radius) in &*circles.borrow() {
                let draw_x = wid.x() + pos.0 - radius;
                let draw_y = wid.y() + pos.1 - radius;
                let diameter = radius * 2;
                if matches!(*selected.borrow(), Some((selected_pos, _)) if selected_pos == *pos) {
                    set_draw_color(Color::from_rgb(200, 200, 200));
                    draw_pie(draw_x, draw_y, diameter, diameter, 0.0, 360.0);
                }
                set_draw_color(Color::Black);
                draw_arc(draw_x, draw_y, diameter, diameter, 0.0, 360.0);
            }
            set_draw_color(Color::Black);
            draw_rect(wid.x(), wid.y(), wid.width(), wid.height());
            pop_clip();
        });

        canvas
    }
    fn set_circles(&mut self, circles: &HashMap<Coord, Radius>) {
        self.circles.replace(circles.clone());
        self.redraw();
    }
    fn selected(&self) -> std::option::Option<(Coord, Radius)> {
        *self.selected.borrow()
    }
}

impl Deref for Canvas {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.widget
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.widget
    }
}

pub struct Circles {
    pub model: Model,
    canvas: Canvas,
    diameter: HorNiceSlider,
    undo: Button,
    redo: Button,
}

impl Circles {
    pub fn build(sender: Sender<Message>) -> Self {
        let mut flex = Flex::default_fill().with_label("    Circles    ").column();
        let row = Flex::default_fill();
        let mut undo = Button::default().with_label("Undo");
        undo.emit(sender, Message::Undo);
        let mut redo = Button::default().with_label("Redo");
        redo.emit(sender, Message::Redo);
        row.end();
        flex.fixed(&row, 35);
        let canvas = Canvas::new(
            WIDGET_PADDING,
            undo.y() + undo.height() + WIDGET_PADDING,
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            sender,
        );
        let row = Flex::default_fill();
        let mut diameter = HorNiceSlider::default().with_align(Align::Left);
        diameter.set_align(Align::Top | Align::Left);
        diameter.set_minimum(1.0);
        diameter.set_maximum(50.0);
        diameter.emit(sender, Message::RadiusChanged);
        row.end();
        flex.fixed(&row, 25);
        flex.end();
        flex.set_margin(10);
        flex.set_pad(10);
        sender.send(Message::ModelChanged);
        Self {
            model: Model::build(sender),
            canvas,
            diameter,
            undo,
            redo,
        }
    }
    pub fn opened(&mut self) {
        self.model.save();
        let (pos, radius) = self.canvas.selected().unwrap();
        self.diameter
            .set_label(&format!("Adjust diameter of circle at {:?}.", pos));
        self.diameter.set_value(f64::from(radius));
    }
    pub fn radius_changed(&mut self) {
        self.model.set(
            self.canvas.selected().unwrap().0,
            self.diameter.value() as Radius,
        )
    }
    pub fn model_changed(&mut self) {
        set_activated(&mut self.undo, self.model.can_undo());
        set_activated(&mut self.redo, self.model.can_redo());
        self.canvas.set_circles(self.model.circles());
    }
}

fn set_activated<T: WidgetExt>(widget: &mut T, is_activated: bool) {
    if is_activated {
        widget.activate();
    } else {
        widget.deactivate();
    }
}
