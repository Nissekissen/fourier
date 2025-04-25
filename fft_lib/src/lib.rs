pub struct FftResult {
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

pub fn fft(in_data: &[f64]) -> FftResult {
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
        let in_data: &[f64] = &[0.0, 1.0 + 0.707, 1.0, 0.707 - 1.0, 0.0, -1.0 - 0.707, -1.0, 0.707 + 1.0];
        let result = fft(&in_data);
        let expected_real: Vec<f64> = vec![0.1767, 0.4785, 0.0000, -0.4785, -0.1767, -0.4785, 0.0000, 0.4785];
        let expected_imag: Vec<f64> = vec![0.0000, -0.3750, 0.1768, 0.1250, 0.0000, -0.1250, -0.1768, 0.3750];
        println!("Result Real: {:?}", result.real);
        println!("Result Imag: {:?}", result.imag);
        assert_float_vec_eq(&result.real, &expected_real);
        assert_float_vec_eq(&result.imag, &expected_imag);
    }
}
