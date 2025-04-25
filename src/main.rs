use fft_lib::fft;

fn main() {
    let data: Vec<f64> = vec![0.0; 100];
    let result = fft(&data);
    println!("Result length: {}", result.real.len());
}
