use crate::{PAD, WIDTH};
use fltk::{button::Button, frame::Frame, group::Flex, output::Output, prelude::*};

pub fn counter() {
    let mut flex = Flex::default_fill().with_label("    Counter    ");
    Frame::default();
    let mut col = Flex::default().column();
    Frame::default();
    let mut out = Output::default();
    out.set_value("0");
    out.set_text_size(WIDTH);
    let mut row = Flex::default();
    for label in ["-", "+"] {
        let mut button = Button::default().with_label(label);
        button.set_label_size(WIDTH);
        button.set_callback({
            let mut out = out.clone();
            move |button| {
                let mut model: u8 = out.value().parse::<u8>().unwrap();
                if button.label() == "-" {
                    if model > 0 {
                        model = model.saturating_sub(1);
                    }
                } else if model < 255 {
                    model = model.saturating_add(1);
                }
                out.set_value(&model.to_string());
            }
        });
    }
    row.end();
    row.set_pad(0);
    Frame::default();
    col.end();
    col.fixed(&out, WIDTH);
    col.fixed(&row, WIDTH);
    col.set_pad(0);
    Frame::default();
    flex.end();
    flex.set_margin(PAD);
    flex.set_pad(PAD);
    flex.fixed(&col, WIDTH * 2);
}
