use fft_lib::{fft, FftResult};
use piston_window::{color::BLACK, *};

mod plot;
mod utils;

use plot::Plot;
use utils::circular_vec::CircularVec;

const HISTORY_LENGTH: usize = 512;
const NUM_FREQUENCIES: usize = 32;
const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 400;
const MAX_LOUDNESS: f64 = 10.0; // TODO: Adjust this

fn main() {
    let mut window: PistonWindow = WindowSettings::new("FFT", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut frame_count: u64 = 0;

    let plot = Plot::new(
        (20.0, 20.0),
        (WINDOW_WIDTH as f64 - 40.0, WINDOW_HEIGHT as f64 - 40.0),
    )
    .caption("FFT")
    .x_axis_label("Time (s)")
    .y_axis_label("Frequency (Hz)");

    let mut fft_data: CircularVec<FftResult> = CircularVec::new(HISTORY_LENGTH);
    for _i in 0..HISTORY_LENGTH {
        let mut data: Vec<f64> = Vec::with_capacity(NUM_FREQUENCIES);
        for j in 0..NUM_FREQUENCIES {
            data.push((j as f64 * 0.1).sin() * 10.0);
        }
        fft_data.push(fft(&data));
    }

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |c, g, _device| {
                clear([1.0; 4], g);

                plot.draw_bg(&c, g);

                let i = frame_count as usize % fft_data.len();
                let fft = fft_data.get(i).unwrap();

                for (i, (_frequency, decibel)) in fft.real.iter().zip(&fft.imag).enumerate() {
                    let x1 = (i * 5) as f64;
                    let y1 = plot.height() as f64;
                    let x2 = x1 + 5.0;
                    let y2 = y1 - (decibel / MAX_LOUDNESS) as f64;

                    plot.rect(BLACK, [x1, y1, x2, y2], &c, g);
                }
            });

            frame_count += 1;
        }
    }
}
