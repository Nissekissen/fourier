#![allow(unused)]

mod plot;
mod utils;

use audio_lib::{AudioSource, AudioStreamer, WavFileSource};
use fft_lib::{fft, get_frequencies};
use plot::bar_visualizer::{BarVisualizer, Rotation};

use piston_window::{color::BLACK, *};
use plot::Visualizer;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread::{self, JoinHandle};
use utils::fps_counter::FpsCounter;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 600;
pub const MAX_LOUDNESS: f64 = 0.06; // TODO: Adjust this for microphone sensitivity
const CHUNK_SIZE: usize = 256; // Chunk size for audio processing, might need adjustment for microphone
const FILE_PATH: &'static str = "./audio/pigstep.wav"; // Path to the audio file

fn main() {
    // Initialize the visualization components
    let num_bars = CHUNK_SIZE / 2;
    let (mut window, visualizer) = initialize_visualization(num_bars);

    // Set up audio processing
    let (audio_rx, audio_thread_handle, sample_rate) = setup_audio_streaming();

    // Set up audio playback
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file_for_playback = BufReader::new(File::open(FILE_PATH).unwrap());
    let source_for_playback = Decoder::new(file_for_playback).unwrap();

    // Start rodio playback
    let _ = stream_handle.play_raw(source_for_playback.convert_samples());

    run_event_loop(&mut window, &visualizer, audio_rx, sample_rate);

    // Cleanup
    if audio_thread_handle.join().is_err() {
        eprintln!("Error joining audio thread.");
    }
}

/// Initialize the visualization window and plot
fn initialize_visualization(num_bars: usize) -> (PistonWindow, impl Visualizer) {
    let window: PistonWindow =
        WindowSettings::new("FFT - Audio File", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let visualizer = BarVisualizer::new(
        10.0,
        10.0,
        WINDOW_WIDTH as f64 - 20.0,
        WINDOW_HEIGHT as f64 - 20.0,
        Rotation::Up,
        num_bars,
    );

    (window, visualizer)
}

/// Set up the audio streaming and playback
fn setup_audio_streaming() -> (Receiver<Vec<f32>>, JoinHandle<()>, u32) {
    let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();

    // Initialize WavFileSource
    let wav_source = WavFileSource::new(FILE_PATH).expect("Failed to create Wavsource");
    let sample_rate = wav_source.get_sample_rate();

    // Set up audio streamer with the correct chunk size
    let mut audio_streamer = AudioStreamer::new(wav_source, CHUNK_SIZE);

    // Start the audio streaming thread
    let audio_thread_handle = thread::spawn(move || {
        println!("Audio streaming thread started (WavFileSource).");
        if let Err(e) = audio_streamer.run(audio_tx) {
            eprintln!("Error running audio streamer: {}", e);
        }
        println!("Audio streaming thread finished.");
    });

    (audio_rx, audio_thread_handle, sample_rate)
}

/// Process audio data and compute FFT
fn process_audio_data(
    audio_rx: &Receiver<Vec<f32>>,
    sample_rate: u32,
    latest_fft_data: &mut Option<fft_lib::Frequencies>,
    audio_stream_ended: &mut bool,
) {
    if *audio_stream_ended {
        return;
    }

    loop {
        match audio_rx.try_recv() {
            Ok(chunk) => {
                let fft_result = fft(chunk
                    .into_iter()
                    .map(|val| val as f64)
                    .collect::<Vec<f64>>()
                    .as_slice());
                let frequencies_data = get_frequencies(&fft_result, sample_rate);
                *latest_fft_data = Some(frequencies_data);
            }
            Err(TryRecvError::Empty) => {
                break;
            }
            Err(TryRecvError::Disconnected) => {
                *audio_stream_ended = true;
                println!("Audio stream ended (sender disconnected).");
                if latest_fft_data.is_none() {
                    println!("Warning: No FFT data was processed from the audio stream.");
                }
                break;
            }
        }
    }
}

/// Render the visualization
fn render_visualization<G, V: Visualizer>(
    c: Context,
    g: &mut G,
    visualizer: &V,
    latest_fft_data: &Option<fft_lib::Frequencies>,
    audio_stream_ended: bool,
    _glyph_cache: &mut Glyphs,
) where
    G: Graphics<Texture = <Glyphs as CharacterCache>::Texture>,
{
    clear([1.0; 4], g);

    if let Some(f) = latest_fft_data {
        if f.frequencies.is_empty() {
            return; // Avoid division by zero if frequencies list is empty
        }

        visualizer.draw(f.amplitudes.as_slice(), &c, g);
    } else if !audio_stream_ended {
    } else if audio_stream_ended && latest_fft_data.is_none() {
    }
}

/// Run the main event loop
fn run_event_loop(
    window: &mut PistonWindow,
    visualizer: &impl Visualizer,
    audio_rx: Receiver<Vec<f32>>,
    sample_rate: u32,
) {
    let mut audio_stream_ended = false;
    let mut latest_fft_data: Option<fft_lib::Frequencies> = None;

    // Create a glyph cache for text rendering
    let mut glyph_cache = window.load_font("assets/Roboto-Regular.ttf").unwrap();

    let mut fps_counter = FpsCounter::new();

    while let Some(event) = window.next() {
        // Process incoming audio chunks and compute FFTs
        process_audio_data(
            &audio_rx,
            sample_rate,
            &mut latest_fft_data,
            &mut audio_stream_ended,
        );

        if let Some(_render_args) = event.render_args() {
            window.draw_2d(&event, |c, g, _device| {
                render_visualization(
                    c,
                    g,
                    visualizer,
                    &latest_fft_data,
                    audio_stream_ended,
                    &mut glyph_cache,
                );
            });
        }

        fps_counter.execute();
    }
}
