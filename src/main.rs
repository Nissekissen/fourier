use audio_lib::wav_file_to_vec;
use fft_lib::{fft, get_frequencies};

use piston_window::{color::BLACK, *};
use std::time::{Duration, Instant};

mod plot;
mod utils;
use plot::Plot;
use utils::circular_vec::CircularVec;

const HISTORY_LENGTH: usize = 2_usize.pow(16);
const NUM_FREQUENCIES: usize = 32;
const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 600;
const MAX_LOUDNESS: f64 = 0.05; // TODO: Adjust this
const SAMPLE_RATE: u32 = 48_000;

fn main() {
    let audio = wav_file_to_vec("./audio/meow.wav", 64).unwrap();
    let mut fft_data = Vec::with_capacity(audio.chunked_data.len());
    for chunk in &audio.chunked_data {
        let fft_result = fft(&chunk);
        let frequencies = get_frequencies(&fft_result, audio.sample_rate);
        fft_data.push(frequencies);
    }

    let mut window: PistonWindow = WindowSettings::new("FFT", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut frame_count: u64 = 0;
    let mut last_time = Instant::now();

    let plot = Plot::new(
        (20.0, 20.0),
        (WINDOW_WIDTH as f64 - 40.0, WINDOW_HEIGHT as f64 - 40.0),
    )
    .caption("FFT")
    .x_axis_label("Time (s)")
    .y_axis_label("Frequency (Hz)");

    // let mut fft_data: CircularVec<fft_lib::Frequencies> = CircularVec::new(HISTORY_LENGTH);
    // for _i in 0..HISTORY_LENGTH {
    //     let mut data: Vec<f64> = Vec::with_capacity(NUM_FREQUENCIES);
    //     for j in 0..NUM_FREQUENCIES {
    //         data.push((j as f64 * 0.1).sin() * 10.0);
    //     }
    //     let result = fft(&data);
    //     let frequencies = get_frequencies(&result, SAMPLE_RATE);
    //     fft_data.push(frequencies);
    // }

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |c, g, _device| {
                clear([1.0; 4], g);

                plot.draw_bg(&c, g);

                let i = frame_count as usize % fft_data.len();
                let f = fft_data.get(i).unwrap();

                for (i, decibel) in f.amplitudes.iter().enumerate() {
                    let bar_width = (plot.width() / f.frequencies.len() as f64) * 0.8;
                    let x = (i as f64 / f.frequencies.len() as f64) * plot.width()
                        + bar_width * 0.1
                        + 5.0;
                    let normalized_height = (decibel / MAX_LOUDNESS).abs() * plot.height();

                    let bar_height = normalized_height;
                    let y = plot.height() - bar_height;

                    plot.rect(BLACK, [x, y, bar_width, bar_height], &c, g);
                }
            });
        }
        frame_count += 1;

        if let Some(_update) = event.update_args() {
            let now = Instant::now();
            let elapsed = now.duration_since(last_time);

            if elapsed >= Duration::from_secs(1) {
                let fps = frame_count as f32 / elapsed.as_secs_f32();
                println!("FPS: {}", fps);
                frame_count = 0;
                last_time = now;
            }
        }
    }
}
