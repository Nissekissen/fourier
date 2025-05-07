use audio_lib::{AudioStreamer, WavFileSource, AudioSource}; // Changed WavFileSource to MicrophoneSource
use fft_lib::{fft, get_frequencies};
use rodio::{Decoder, OutputStream, source::Source};

use piston_window::{color::BLACK, *};
use std::sync::mpsc;
use std::thread;
use std::io::BufReader;
use std::fs::File;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant}; // Added Instant

mod plot;
// mod utils; // utils::circular_vec::CircularVec is no longer used directly here
use plot::Plot;
// use utils::circular_vec::CircularVec; // No longer used for fft_data

// const HISTORY_LENGTH: usize = 2_usize.pow(16); // No longer used
// const NUM_FREQUENCIES: usize = 32; // This is implicitly CHUNK_SIZE / 2
const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 600;
const MAX_LOUDNESS: f64 = 0.05; // TODO: Adjust this for microphone sensitivity
// const SAMPLE_RATE: u32 = 48_000; // Will be obtained from AudioSource
const CHUNK_SIZE: usize = 256; // Chunk size for audio processing, might need adjustment for mic
const FILE_PATH: &str = "./audio/hank.wav"; // Path to the audio file

fn main() {
    // --- 1. Initialize PistonWindow and Plot ---
    let mut window: PistonWindow = WindowSettings::new("FFT - Audio File", [WINDOW_WIDTH, WINDOW_HEIGHT]) // Changed title
        .exit_on_esc(true)
        .build()
        .unwrap();

    let plot = Plot::new(
        (20.0, 20.0),
        (WINDOW_WIDTH as f64 - 40.0, WINDOW_HEIGHT as f64 - 40.0),
    )
    .caption("FFT")
    .x_axis_label("Frequency (Hz)") // Corrected: X-axis is usually frequency for FFT
    .y_axis_label("Amplitude");    // Corrected: Y-axis is usually amplitude/loudness

    // Audio streaming setup
    let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();

    // Using WavFileSource
    let wav_source =
        WavFileSource::new(FILE_PATH).expect("Failed to create Wavsource");
    let sample_rate = wav_source.get_sample_rate();

    let mut audio_streamer = AudioStreamer::new(wav_source, CHUNK_SIZE);

    // --- Setup rodio playback components (but don't start playing yet) ---
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file_for_playback = BufReader::new(File::open(FILE_PATH).unwrap());
    let source_for_playback = Decoder::new(file_for_playback).unwrap();

    // Store only the latest FFT data for display
    let mut latest_fft_data: Option<fft_lib::Frequencies> = None;
    let mut audio_stream_ended = false;

    let mut frame_count: u64 = 0;
    let mut last_time = Instant::now();

    // --- 2. Start the audio streaming thread and rodio playback ---
    // Spawn the audio streaming thread. It will start processing audio data immediately.
    let audio_thread_handle = thread::spawn(move || {
        println!("Audio streaming thread started (WavFileSource).");
        if let Err(e) = audio_streamer.run(audio_tx) { // audio_streamer and audio_tx are moved here
            eprintln!("Error running audio streamer: {}", e);
        }
        println!("Audio streaming thread finished.");
    });

    // Start rodio playback almost immediately after starting the streaming thread.
    // source_for_playback is moved here.
    let _ = stream_handle.play_raw(source_for_playback.convert_samples());


    // The initial render block and the 50ms sleep are removed.
    // The window will become visible with the first event processed in the loop.
    // Playback and data streaming start before the loop.

    // --- Main event loop ---
    while let Some(event) = window.next() {
        // Process incoming audio chunks and compute FFTs
        if !audio_stream_ended {
            loop {
                match audio_rx.try_recv() {
                    Ok(chunk) => {
                        let fft_result = fft(chunk.into_iter().map(|val| val as f64).collect::<Vec<f64>>().as_slice());
                        let frequencies_data = get_frequencies(&fft_result, sample_rate);
                        latest_fft_data = Some(frequencies_data); // Update with the latest FFT data
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        // No new data in the channel for now
                        break;
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        // Sender has disconnected, audio stream has ended
                        audio_stream_ended = true;
                        println!("Audio stream ended (sender disconnected).");
                        if latest_fft_data.is_none() {
                            println!("Warning: No FFT data was processed from the audio stream.");
                        }
                        break;
                    }
                }
            }
        }

        if let Some(_render_args) = event.render_args() {
            window.draw_2d(&event, |c, g, _device| {
                clear([1.0; 4], g);
                plot.draw_bg(&c, g);

                // Display the latest FFT data if available
                if let Some(f) = &latest_fft_data {
                    if f.frequencies.is_empty() {
                        return; // Avoid division by zero if frequencies list is empty
                    }

                    for (i, decibel) in f.amplitudes.iter().enumerate() {
                        let bar_width = (plot.width() / f.frequencies.len() as f64) * 0.8;
                        let x = (i as f64 / f.frequencies.len() as f64) * plot.width()
                            + bar_width * 0.1 // 10% of bar_width for spacing on the left
                            + 5.0; // Additional small margin
                        
                        // Normalize loudness. Ensure decibel is positive for height calculation.
                        // MAX_LOUDNESS might need significant adjustment for microphone input.
                        let normalized_loudness = decibel.abs() / MAX_LOUDNESS;
                        let bar_height = (normalized_loudness * plot.height()).min(plot.height()); // Cap height
                        let y = plot.height() - bar_height;

                        plot.rect(BLACK, [x, y, bar_width, bar_height], &c, g);
                    }
                } else if !audio_stream_ended {
                    // Optionally, draw a "Waiting for audio..." message
                    // For simplicity, this is omitted here.
                } else if audio_stream_ended && latest_fft_data.is_none() {
                    // Optionally, draw "No audio data received"
                }
            });
        }

        frame_count += 1; // Still useful for FPS calculation

        if let Some(_update_args) = event.update_args() {
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

    // Wait for the audio streaming thread to finish
    if audio_thread_handle.join().is_err() {
        eprintln!("Error joining audio thread.");
    }
}

