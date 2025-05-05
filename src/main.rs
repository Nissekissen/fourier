use audio_lib::AudioCapture;
use std::sync::mpsc::channel;
use std::time::Duration;
use fft_lib::{fft, FftResult, Frequencies, get_frequenices};

fn main() -> Result<(), anyhow::Error> {
    let (tx, rx) = channel::<Vec<f32>>();

    let mut capture = AudioCapture::new(tx)?;
    capture.start()?;

    println!("Recording...");

    // Example consumer loop
    loop {
        if let Ok(chunk) = rx.recv_timeout(Duration::from_millis(500)) {
            // println!("Received audio chunk of size: {}", chunk.len());
            let mut audio_data = vec![0.0; chunk.len()];
            for (i, &sample) in chunk.iter().enumerate() {
                audio_data[i] = sample as f64;
            }

            let fft_result: FftResult = fft(&audio_data);
            let frequencies = get_frequenices(&fft_result, 48000);

            // Print the average amplitude of the first 10 frequencies
            let avg_amplitude: f64 = frequencies.amplitudes.iter().take(10).sum::<f64>() / 10.0;
            println!("Average amplitude of 0-{}hz: {:?}", frequencies.frequencies[9].round(), avg_amplitude);
        } else {
            println!("No audio received in last 500ms");
        }
    }
}
