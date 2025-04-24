use fft_lib::fft;

fn main() {
    let data = vec![0; 100];
    let result = fft(&data);
    println!("Result length: {}", result.real.len());
}
