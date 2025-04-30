#![allow(non_snake_case)]

/// Represents the result of a Fast Fourier Transform (FFT).
#[derive(Debug)]
pub struct FftResult {
    /// Vector containing the real components of the FFT result.
    pub real: Vec<f64>,
    /// Vector containing the imaginary components of the FFT result.
    pub imag: Vec<f64>,
}

/// Represent the frequencies in a range of samples
pub struct Frequencies {
    /// The frequencies corresponding to the FFT result.
    pub frequencies: Vec<f64>,
    /// The amplitudes corresponding to the FFT result.
    pub amplitudes: Vec<f64>,
    /// The total number of samples.
    pub total_samples: usize,
    /// The sample rate of the input signal.
    pub sample_rate: u32,
    /// The start time of the signal.
    pub start_time: f64,
}

fn twiddle_factor(k: f64, N: usize) -> (f64, f64) {
    let angle = -2.0 * std::f64::consts::PI * k / (N as f64);
    (angle.cos(), angle.sin())
}

/// Calculates the frequencies corresponding to each element in the FFT result.
///
/// # Arguments
///
/// * `fft_result` - A reference to the `FftResult` struct containing the real and imaginary components of the FFT.
/// * `sample_rate` - The sample rate of the input signal.
///
/// # Returns
///
/// A `Frequencies` struct containing the frequencies, amplitudes, total samples, sample rate, and start time.
/// You need to set the start time manually, as it is not calculated in this function.
pub fn get_frequenices(fft_result: &FftResult, sample_rate: u32) -> Frequencies {
    let N = fft_result.real.len();
    let mut frequencies = Vec::with_capacity(N / 2);
    let mut amplitudes = Vec::with_capacity(N / 2);
    let mut start_time = 0.0;
    let mut total_samples = 0;
    let sample_rate = sample_rate as f64;

    for i in 0..N / 2 {
        let frequency = (i as f64) * sample_rate / (N as f64);
        frequencies.push(frequency);
        let amplitude = ((fft_result.real[i] * fft_result.real[i])
            + (fft_result.imag[i] * fft_result.imag[i]))
            .sqrt();
        amplitudes.push(amplitude);
    }
    if N > 0 {
        start_time = 1.0 / sample_rate;
        total_samples = N;
    }

    Frequencies {
        frequencies,
        amplitudes,
        total_samples,
        sample_rate: sample_rate as u32,
        start_time,
    }
}

fn bit_reverse(n: u64, num_bits: u32) -> u64 {
    let mut reversed = 0;
    for i in 0..num_bits {
        if (n >> i) & 1 != 0 {
            // Kontrollera om den i-te biten är satt
            reversed |= 1 << (num_bits - 1 - i); // Sätt motsvarande bit i reversed
        }
    }
    reversed
}

/// Performs a Fast Fourier Transform (FFT) on the given input data.
///
/// # Arguments
///
/// * `in_data` - A slice of f64 values representing the input data.
///
/// # Returns
///
/// An `FftResult` struct containing the real and imaginary components of the FFT.
///
/// # Errors
///
/// This function will return an error if the input length is not a power of 2 or if the input length is 0.
pub fn fft(in_data: &[f64]) -> FftResult {
    let N = in_data.len();

    // N needs to be a power of 2
    if N == 0 || (N & (N - 1)) != 0 {
        panic!("Input length must be a power of 2 and greater than 0.");
    }

    let mut real: Vec<f64> = vec![0.0; N];
    let mut imag: Vec<f64> = vec![0.0; N];

    // Bit reversal
    for i in 0..in_data.len() {
        let j = bit_reverse(i as u64, (N as f64).log2() as u32);
        real[j as usize] = in_data[i];
    }

    // Butterfly
    let m = ((N as f64).log2() as u64) + 1;

    for s in 1..m {
        let n_step = 2_i32.pow(s as u32);
        let half_step = 2_i32.pow(s as u32 - 1);

        for i in (0..N).step_by(n_step as usize) {
            for j in 0..half_step {
                let a_real = real[i + j as usize];
                let a_imag = imag[i + j as usize];

                let b_real = real[i + j as usize + half_step as usize];
                let b_imag = imag[i + j as usize + half_step as usize];

                let (twiddle_real, twiddle_imag) = twiddle_factor(j as f64, n_step as usize);

                let temp_real = twiddle_real * b_real - twiddle_imag * b_imag;
                let temp_imag = twiddle_real * b_imag + twiddle_imag * b_real;

                real[i + j as usize] = a_real + temp_real;
                imag[i + j as usize] = a_imag + temp_imag;

                real[i + j as usize + half_step as usize] = a_real - temp_real;
                imag[i + j as usize + half_step as usize] = a_imag - temp_imag;
            }
        }
    }

    // Normalize the result by dividing by N
    for i in 0..N {
        real[i] /= N as f64;
        imag[i] /= N as f64;
    }

    FftResult { real, imag }
}

pub fn dft(in_data: &[f64]) -> FftResult {
    let N = in_data.len();
    let mut real = Vec::with_capacity(N);
    let mut imag = Vec::with_capacity(N);

    // DFT
    for k in 0..in_data.len() {
        let mut sum_real: f64 = 0.0;
        let mut sum_imag: f64 = 0.0;
        for n in 0..N {
            let angle = -2.0 * std::f64::consts::PI * (k as f64) * (n as f64) / (N as f64);
            sum_real += in_data[n] as f64 * angle.cos();
            sum_imag += in_data[n] as f64 * angle.sin();
        }
        // Normalize the result by dividing by N
        real.push(sum_real / (N as f64));
        imag.push(sum_imag / (N as f64));
    }

    FftResult { real, imag }
}

#[cfg(test)]
pub fn compare_speed() {
    let in_data: Vec<f64> = (0..1024).map(|x| (x as f64).sin()).collect();

    let start_dft = std::time::Instant::now();
    let _ = dft(&in_data);
    let duration_dft = start_dft.elapsed();
    println!("DFT took: {:?}", duration_dft);

    let start_fft = std::time::Instant::now();
    let _ = fft(&in_data);
    let duration_fft = start_fft.elapsed();
    println!("FFT took: {:?}", duration_fft);
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::assert_float_absolute_eq;

    fn assert_float_vec_eq(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len(), "Vectors have different lengths");
        for (i, (&x, &y)) in a.iter().zip(b.iter()).enumerate() {
            assert_float_absolute_eq!(x, y, 1e-4); // Tillräckligt noggrannt för tester
        }
    }

    #[test]
    fn constant_dc_curve() {
        let in_data: &[f64] = &[1.0, 1.0, 1.0, 1.0];
        let result = fft(&in_data);
        let expected_real: Vec<f64> = vec![1.0, 0.0, 0.0, 0.0];
        let expected_imag: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0];
        assert_float_vec_eq(&result.real, &expected_real);
        assert_float_vec_eq(&result.imag, &expected_imag);
    }

    #[test]
    fn simple_sinus_curve() {
        let in_data: &[f64] = &[0.0, 1.0, 0.0, -1.0];
        let result = fft(&in_data);
        let expected_real: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0];
        let expected_imag: Vec<f64> = vec![0.0, -0.5, 0.0, 0.5];
        assert_float_vec_eq(&result.real, &expected_real);
        assert_float_vec_eq(&result.imag, &expected_imag);
    }

    #[test]
    fn sum_of_sinusoids() {
        let in_data: &[f64] = &[
            0.0,
            1.0 + 0.707,
            1.0,
            0.707 - 1.0,
            0.0,
            -1.0 - 0.707,
            -1.0,
            0.707 + 1.0,
        ];
        let result = fft(&in_data);
        let expected_real: Vec<f64> = vec![
            0.1767, 0.4785, 0.0000, -0.4785, -0.1767, -0.4785, 0.0000, 0.4785,
        ];
        let expected_imag: Vec<f64> = vec![
            0.0000, -0.3750, 0.1768, 0.1250, 0.0000, -0.1250, -0.1768, 0.3750,
        ];
        assert_float_vec_eq(&result.real, &expected_real);
        assert_float_vec_eq(&result.imag, &expected_imag);
    }

    #[test]
    fn combination_of_two_frequencies() {
        let in_data: &[f64] = &[1.0, 2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0];
        let result = fft(&in_data);
        let expected_real: Vec<f64> = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let expected_imag: Vec<f64> = vec![0.0, 0.0, -0.5, 0.0, 0.0, 0.0, 0.5, 0.0];
        assert_float_vec_eq(&result.real, &expected_real);
        assert_float_vec_eq(&result.imag, &expected_imag);
    }
}
