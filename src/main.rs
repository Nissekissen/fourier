use fft_lib::{fft, get_frequencies};
use audio_lib::wav_file_to_vec;

fn main() {
    let audio = wav_file_to_vec("./audio/meow.wav").unwrap();
    for chunk in &audio.chunked_data {
        let fft_result = fft(chunk.clone().as_slice());
        let frequencies = get_frequencies(&fft_result, audio.sample_rate);
        println!("Frequencies: {:?}", frequencies);
    }
}
