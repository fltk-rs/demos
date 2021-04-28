use fltk::{prelude::*, *};
use plotters::prelude::*;
use plotters::style::Color;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use plotters_bitmap::BitMapBackend;
use std::collections::VecDeque;
use std::error::Error;
use std::time::SystemTime;
const W: usize = 480;
const H: usize = 320;

const SAMPLE_RATE: f64 = 10_000.0;
const FREAME_RATE: f64 = 30.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0u8; W * H * 3];

    let fx: f64 = 1.0;
    let fy: f64 = 1.1;
    let xphase: f64 = 0.0;
    let yphase: f64 = 0.1;

    let app = app::App::default();
    let mut win = window::Window::default().with_size(W as i32, H as i32);
    let mut frame = frame::Frame::default().size_of(&win);
    win.end();
    win.show();
    let root =
        BitMapBackend::<RGBPixel>::with_buffer_and_format(&mut buf, (W as u32, H as u32))?
            .into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .set_all_label_area_size(30)
        .build_cartesian_2d(-1.2..1.2, -1.2..1.2)?;

    chart
        .configure_mesh()
        .label_style(("sans-serif", 15).into_font().color(&GREEN))
        .axis_style(&GREEN)
        .draw()?;

    let cs = chart.into_chart_state();
    drop(root);

    let mut data = VecDeque::new();
    let start_ts = SystemTime::now();
    let mut last_flushed = 0.0;

    // unsafe { draw::draw_rgb_nocopy(&mut frame, &buf); }

    while app.wait() {
        let epoch = SystemTime::now()
            .duration_since(start_ts)
            .unwrap()
            .as_secs_f64();

        if let Some((ts, _, _)) = data.back() {
            if epoch - ts < 1.0 / SAMPLE_RATE {
                std::thread::sleep(std::time::Duration::from_secs_f64(epoch - ts));
                continue;
            }
            let mut ts = *ts;
            while ts < epoch {
                ts += 1.0 / SAMPLE_RATE;
                let phase_x: f64 = 2.0 * ts * std::f64::consts::PI * fx + xphase;
                let phase_y: f64 = 2.0 * ts * std::f64::consts::PI * fy + yphase;
                data.push_back((ts, phase_x.sin(), phase_y.sin()));
            }
        }

        let phase_x = 2.0 * epoch * std::f64::consts::PI * fx + xphase;
        let phase_y = 2.0 * epoch * std::f64::consts::PI * fy + yphase;
        data.push_back((epoch, phase_x.sin(), phase_y.sin()));

        if epoch - last_flushed > 1.0 / FREAME_RATE {
            let root = BitMapBackend::<RGBPixel>::with_buffer_and_format(
                &mut buf,
                (W as u32, H as u32),
            )?
            .into_drawing_area();
            let mut chart = cs.clone().restore(&root);
            chart.plotting_area().fill(&BLACK)?;

            chart
                .configure_mesh()
                .bold_line_style(&GREEN.mix(0.2))
                .light_line_style(&TRANSPARENT)
                .draw()?;

            chart.draw_series(data.iter().zip(data.iter().skip(1)).map(
                |(&(e, x0, y0), &(_, x1, y1))| {
                    PathElement::new(
                        vec![(x0, y0), (x1, y1)],
                        &GREEN.mix(((e - epoch) * 20.0).exp()),
                    )
                },
            ))?;

            drop(root);
            drop(chart);

            draw::draw_rgb(&mut frame, &buf).unwrap();

            last_flushed = epoch;
        }

        while let Some((e, _, _)) = data.front() {
            if ((e - epoch) * 20.0).exp() > 0.1 {
                break;
            }
            data.pop_front();
        }
        win.redraw();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}
