use fft_lib::{fft, FftResult};
use piston_window::*;

mod plot;
mod utils;

use plot::Plot;
use utils::circular_vec::CircularVec;

const NUM_ENTRIES: usize = 512;
const NUM_FREQUENCIES: usize = 256;
const WIDTH: u32 = NUM_ENTRIES as u32 * 2;
const HEIGHT: u32 = NUM_FREQUENCIES as u32;
const MAX_LOUDNESS: f64 = 20.0; // TODO: Adjust this

fn main() {
    let mut window: PistonWindow = WindowSettings::new("FFT", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let plot = Plot::new((20.0, 20.0), (WIDTH as f64 - 40.0, HEIGHT as f64 - 40.0))
        .caption("FFT")
        .x_axis_label("Time (s)")
        .y_axis_label("Frequency (Hz)");

    let mut fft_data: CircularVec<FftResult> = CircularVec::new(NUM_ENTRIES);

    for i in 0..NUM_ENTRIES {
        fft_data.push(fft(&[i as f64; NUM_FREQUENCIES]));
    }

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |c, g, _device| {
                clear([0.0; 4], g);

                plot.draw_bg(&c, g);

                for (i, fft) in fft_data.iter().enumerate() {
                    for (frequency, decibel) in fft.real.iter().zip(&fft.imag) {
                        plot.point(
                            [(decibel / MAX_LOUDNESS) as f32, 0.0, 0.0, 1.0],
                            (WIDTH / 2 - i as u32) as f64,
                            *frequency,
                            &c,
                            g,
                        );
                    }
                }
            });
        }
    }
}
