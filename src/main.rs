use std::{net::TcpStream, sync::mpsc::channel};
use std::time::Duration;
use fft_lib::{fft, FftResult, Frequencies, get_frequenices};
use audio_lib::{AudioSource, WavFileSource, MicrophoneSource, AudioStreamer};
use std::thread;

fn main() -> Result<(), anyhow::Error> {
    let (audio_sender, audio_receiver) = channel::<Vec<f32>>();

    // --- Microphone Stream Setup ---
    let mic_source = MicrophoneSource::new()?;
    let sample_rate = mic_source.get_sample_rate();
    let mut mic_streamer = AudioStreamer::new(mic_source);

    // Run the microphone streamer in a separate thread
    let mic_thread_handle = thread::spawn(move || {
        println!("Microphone thread started.");
        if let Err(e) = mic_streamer.run(audio_sender) {
            eprintln!("Error running microphone streamer: {}", e);
        }
        println!("Microphone thread finished.");
    });

    // --- Example: Receive and process audio data from the microphone ---
    println!("Main thread: Waiting for audio data from microphone...");
    loop {
        match audio_receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(data_chunk) => {
                // Process the audio data_chunk here
                let fft_result = fft(&data_chunk.iter().map(|&x| x as f64).collect::<Vec<f64>>());
                let frequencies = get_frequenices(&fft_result, sample_rate as u32);
                let avg_first_5 = frequencies.amplitudes.iter().take(5).sum::<f64>() / 5.0;
                println!("Average of amplitudes 0-{}: {}", frequencies.frequencies[4], avg_first_5);
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // You can add logic here if no data is received for a while
                // Or check if the mic_thread_handle is finished if you want to stop
                if mic_thread_handle.is_finished() {
                    println!("Main thread: Microphone thread has finished. Exiting loop.");
                    break;
                }
                // println!("Main thread: Still waiting for mic data...");
                continue;
            }
            Err(e) => {
                eprintln!("Main thread: Error receiving audio data: {}. Exiting loop.", e);
                break;
            }
        }
        // Add a condition to break the loop, e.g., after a certain time or a specific command
        // For this example, it will loop indefinitely until an error or the sender is dropped.
    }

    // Optionally, wait for the microphone thread to complete, though in a real app,
    // you might have a different shutdown mechanism.
    // if let Err(e) = mic_thread_handle.join() {
    //     eprintln!("Error joining microphone thread: {:?}", e);
    // }

    println!("Exiting main function.");
    Ok(())
}