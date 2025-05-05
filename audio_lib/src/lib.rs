// Crate for audio processing
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::Sender;

pub struct Audio {
    pub sample_rate: u32,
    pub duration: u32, // in milliseconds
    pub length: u32,   // number of samples
    pub raw_data: Vec<f32>,
    pub chunked_data: Vec<Vec<f64>>,
}

pub struct AudioCapture {
    stream: cpal::Stream,
}

impl AudioCapture {
    pub fn new(tx: Sender<Vec<f32>>) -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

        let supported_config = device.default_input_config()?; // Safe default
        let config = cpal::StreamConfig {
            channels: supported_config.channels(),
            sample_rate: supported_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default, // Let cpal/device choose a safe buffer
        };

        println!("Sample rate: {}", config.sample_rate.0);

        let mut chunk = Vec::with_capacity(1024);

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    chunk.push(sample);
                    if chunk.len() >= 1024 {
                        if tx.send(chunk.clone()).is_err() {
                            eprintln!("Failed to send audio data to the channel.");
                        }
                        chunk.clear();
                    }
                }
            },
            |err| eprintln!("Error occurred on stream: {}", err),
            None,
        )?;

        Ok(Self { stream })
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        self.stream.play()?;
        Ok(())
    }
}

pub fn wav_file_to_vec(file_path: &str) -> Result<Audio, String> {
    use hound::{WavReader, WavSpec};
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = WavReader::new(BufReader::new(file)).map_err(|e| e.to_string())?;

    let spec: WavSpec = reader.spec();
    let sample_rate = spec.sample_rate;
    let length = reader.len() as u32;
    let duration = (length as f32 / sample_rate as f32 * 1000.0) as u32; // convert to milliseconds

    let raw_data: Vec<f32> = reader
        .into_samples::<i16>() // Read samples as i16 (common format for WAV files)
        .map(|s| s.unwrap() as f32 / i16::MAX as f32) // Normalize to f32 in the range [-1.0, 1.0]
        .collect();

    // CHunk the data into chunks of 1024 samples
    let chunk_size = 1024;
    let chunked_data: Vec<Vec<f64>> = raw_data
        .chunks(chunk_size as usize)
        .map(|chunk| {
            let mut chunk_vec: Vec<f64> = chunk.iter().map(|&x| x as f64).collect();
            // Pad the chunk with zeros if it's not a full chunk
            if chunk_vec.len() < chunk_size as usize {
                chunk_vec.resize(chunk_size as usize, 0.0);
            }
            chunk_vec
        })
        .collect();

    Ok(Audio {
        sample_rate,
        duration,
        length,
        raw_data,
        chunked_data,
    })
}
