use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavReader, WavSpec};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant}; // Modified this line
// std::thread is not directly used in this file anymore after the change, but keep if other parts use it.

pub trait AudioSource {
    fn get_sample_rate(&self) -> u32;
    fn get_duration(&self) -> Duration;
    fn get_length(&self) -> u64;
    fn start_streaming(&mut self, sender: Sender<Vec<f32>>, chunk_size: usize) -> Result<(), anyhow::Error>;
}

/// Wav file source for audio streaming.
/// # How to use:
/// ```rust
/// let (audio_tx, audio_rx) = std::sync::mpsc::channel::<Vec<f32>>();
/// 
/// let wav_source = WavFileSource::new("path/to/audio.wav")
///    .map_err(|e| anyhow::anyhow!("Failed to create WavFileSource: {}", e))?;
/// let sample_rate = wav_source.get_sample_rate();
/// let mut wav_streamer = AudioStreamer::new(wav_source, 128_usize);
/// 
/// // Run the WAV streamer in a separate thread
/// let wav_thread_handle = std::thread::spawn(move || {
///     println!("WAV file thread started.");
///     if let Err(e) = wav_streamer.run(audio_tx) {
///         eprintln!("Error running WAV file streamer: {}", e);
///     }
///     println!("WAV file thread finished.");
/// }
/// 
/// // Main thread example
/// loop {
///     match audio_rx.recv_timeout(Duration::from_secs(1)) {
///         Ok(data_chunk) => {
///             // Process the audio data_chunk here
///             println!("Received audio data chunk of size: {}", data_chunk.len());
///         }
///         Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
///             if wav_thread_handle.is_finished() {
///                 println!("Main thread: WAV file thread has finished. Exiting loop.");
///                 break;
///             }
///             // println!("Main thread: Still waiting for WAV data...");
///             continue;
///         }
///         Err(e) => {
///             eprintln!("Main thread: Error receiving audio data: {}. Exiting loop.", e);
///             break;
///         }
///     }
/// }
/// 
/// ```.
/// 
pub struct WavFileSource {
    reader: WavReader<BufReader<File>>,
    spec: WavSpec,
    sample_rate: u32,
    duration: Duration,
    length: u64,
}

impl WavFileSource {
    pub fn new(file_path: &str) -> Result<Self, String> {
        let file = File::open(file_path).map_err(|e| e.to_string())?;
        let reader = WavReader::new(BufReader::new(file)).map_err(|e| e.to_string())?;
        let spec = reader.spec();
        let sample_rate = spec.sample_rate;
        let length = reader.len() as u64;
        let duration = Duration::from_secs_f32(length as f32 / sample_rate as f32);

        Ok(Self { reader, spec, sample_rate, duration, length })
    }
}

impl AudioSource for WavFileSource {
    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn get_duration(&self) -> Duration {
        self.duration
    }

    fn get_length(&self) -> u64 {
        self.length
    }

    fn start_streaming(&mut self, sender: Sender<Vec<f32>>, chunk_size: usize) -> Result<(), anyhow::Error> {
        let sample_rate = self.spec.sample_rate;
        let mut buffer = vec![0.0; chunk_size];
        let mut next_chunk_target_time = Instant::now();

        loop {
            let mut written = 0;
            for i in 0..chunk_size {
                if let Some(sample_result) = self.reader.samples::<i16>().next() {
                    let sample = sample_result.map_err(|e| anyhow::anyhow!("Error reading sample: {}", e))?;
                    buffer[i] = sample as f32 / i16::MAX as f32;
                    written += 1;
                } else {
                    break; // End of file
                }
            }

            if written > 0 {
                let actual_data_duration = Duration::from_secs_f32(written as f32 / sample_rate as f32);

                if sender.send(buffer[0..written].to_vec()).is_err() {
                    eprintln!("WAV stream: Receiver dropped. Stopping.");
                    return Ok(()); 
                }
                
                next_chunk_target_time += actual_data_duration;

                let current_time = Instant::now();
                if next_chunk_target_time > current_time {
                    std::thread::sleep(next_chunk_target_time - current_time);
                }
            } else {
                break; // No data written, means end of file.
            }
        }
        Ok(())
    }
}

/// Microphone source for audio streaming
/// # How to use:
/// ```rust
/// let (audio_tx, audio_rx) = std::sync::mpsc::channel::<Vec<f32>>();
/// 
/// let mic_source = MicrophoneSource::new()
///    .map_err(|e| anyhow::anyhow!("Failed to create MicrophoneSource: {}", e))?;
/// let sample_rate = mic_source.get_sample_rate(); // If you need to use it, save it here
/// let mut mic_streamer = AudioStreamer::new(mic_source, 128_usize);
/// 
/// // Run the microphone streamer in a separate thread
/// let mic_thread_handle = std::thread::spawn(move || {
///     println!("Microphone thread started.");
///     if let Err(e) = mic_streamer.run(audio_tx) {
///         eprintln!("Error running microphone streamer: {}", e);
///     }
///     println!("Microphone thread finished.");
/// }
/// 
/// // Main thread example
/// loop {
///     match audio_rx.recv_timeout(Duration::from_secs(1)) {
///         Ok(data_chunk) => {
///             // Process the audio data_chunk here
///             println!("Received audio data chunk of size: {}", data_chunk.len());
///         }
///         Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
///             if mic_thread_handle.is_finished() {
///                 println!("Main thread: Microphone thread has finished. Exiting loop.");
///                 break;
///             }
///             continue;
///         }
///         Err(e) => {
///             eprintln!("Main thread: Error receiving audio data: {}. Exiting loop.", e);
///             break;
///         }
///     }
/// }
///     
/// ```
pub struct MicrophoneSource {
    device: cpal::Device,
    config: cpal::StreamConfig,
}

impl MicrophoneSource {
    pub fn new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No default input device available"))?;
        println!("Using input device: {}", device.name().unwrap_or_else(|_| "Unknown".to_string()));
        let config = device
            .default_input_config()
            .map_err(|e| anyhow::anyhow!("Failed to get default input config: {}", e))?
            .config();

        println!("Microphone Default input config: Sample Rate: {:?}, Channels: {:?}, Format: {:?}", config.sample_rate, config.channels, device.default_input_config().unwrap().sample_format());

        Ok(MicrophoneSource {
            device,
            config,
        })
    }
}

impl AudioSource for MicrophoneSource {
    fn get_sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }

    fn get_duration(&self) -> Duration {
        Duration::from_secs(u64::MAX) // Indefinite for microphone
    }

    fn get_length(&self) -> u64 {
        u64::MAX // Indefinite for microphone
    }

    fn start_streaming(&mut self, sender: Sender<Vec<f32>>, chunk_size: usize) -> Result<(), anyhow::Error> {
        let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

        // Channel to signal this function to stop from the audio callback
        let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();

        let callback_sender = sender.clone(); // Clone sender for the callback
        
        let mut internal_buffer: Vec<f32> = Vec::with_capacity(chunk_size * 2); // Pre-allocate some space

        let stream = self.device.build_input_stream(
            &self.config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                internal_buffer.extend_from_slice(data);

                while internal_buffer.len() >= chunk_size {
                    // Drain the first desired_chunk_size elements from the buffer
                    let chunk_to_send: Vec<f32> = internal_buffer.drain(0..chunk_size).collect();
                    
                    if callback_sender.send(chunk_to_send).is_err() {
                        // Receiver has been dropped, signal the main streaming loop to stop.
                        // Ignore error if stop_rx has already been dropped (main function exited)
                        let _ = stop_tx.send(());
                        return; // Stop processing in this callback invocation
                    }
                }
            },
            err_fn,
            None // Timeout
        ).map_err(|e| anyhow::anyhow!("Failed to build input stream: {}", e))?;

        stream.play().map_err(|e| anyhow::anyhow!("Failed to play stream: {}", e))?;
        println!("Microphone stream started. Waiting for stop signal...");

        // Block this function (and thus the thread it's running on)
        // until the audio callback signals that the receiver was dropped.
        // This keeps the `stream` object alive.
        match stop_rx.recv() {
            Ok(()) => {
                println!("MicrophoneSource: Stop signal received. Ending stream.");
            }
            Err(_) => {
                // This means stop_tx (held by the audio callback) was dropped.
                // This could happen if the callback panics or the stream is closed abruptly.
                eprintln!("MicrophoneSource: Stop signal channel disconnected. Ending stream.");
            }
        }
        // When this function returns, `stream` is dropped, and the cpal stream stops.
        // Note: Any data remaining in internal_buffer that is less than chunk_size
        // will not be sent when the stream stops.
        Ok(())
    }
}

/// AudioStreamer struct for streaming audio data from a source.
pub struct AudioStreamer<T: AudioSource + Send + 'static> {
    source: T,
    sample_rate: u32,
    chunk_size: usize,
    #[allow(dead_code)] // channels is not used yet
    channels: u16,
}

impl<T: AudioSource + Send + 'static> AudioStreamer<T> {
    /// Creates a new AudioStreamer instance.
    /// # Arguments
    /// * `source` - The audio source to stream from.
    /// * `chunk_size` - The size of the audio chunks to stream.
    /// # Returns
    /// A new `AudioStreamer` instance.
    /// # Panics
    /// If the chunk size is not a power of 2.
    pub fn new(source: T, chunk_size: usize) -> Self {
        let sample_rate = source.get_sample_rate();
        let channels = 1; // Assuming mono for now, or get from source if available/needed

        if chunk_size != chunk_size.next_power_of_two() {
            panic!("Chunk size must be a power of 2.");
        }

        AudioStreamer { source, sample_rate, channels, chunk_size }
    }

    /// Starts streaming audio data from the source.
    /// # Arguments
    /// * `sender` - The channel sender to send audio data to.,
    /// # Returns
    /// A result indicating success or failure.
    pub fn run(&mut self, sender: Sender<Vec<f32>>) -> Result<(), anyhow::Error> {
        println!("AudioStreamer: Starting source streaming...");
        self.source.start_streaming(sender, self.chunk_size)?;
        println!("AudioStreamer: Source streaming finished.");
        Ok(())
    }
}
