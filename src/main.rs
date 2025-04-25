use fft_lib::FftResult;
use piston_window::*;

mod plot;
mod utils;

use plot::Plot;
use utils::circular_vec::CircularVec;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const NUM_ENTRIES: usize = 1000;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("FFT", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let plot = Plot::new((20.0, 20.0), (WIDTH as f64 - 20.0, HEIGHT as f64 - 20.0))
        .x_axis_label("Time (s)")
        .y_axis_label("Frequency (Hz)");

    let fft_data: CircularVec<FftResult> = CircularVec::new(NUM_ENTRIES);

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |c, g, _device| {
                clear([0.0; 4], g);

                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [0.0, 0.0, 100.0, 100.0],
                    c.transform,
                    g,
                );
            });
        }
    }
}
