pub struct FftResult {
    pub real: Vec<u64>,
    pub imag: Vec<u64>,
}

pub fn fft(in_data: &[u64]) -> FftResult {
    let N = in_data.len();
    let mut real = Vec::with_capacity(N);
    let mut imag = Vec::with_capacity(N);

    for k in 0..in_data.len() {
        real.push(k as u64);
        imag.push(k as u64);
    }

    FftResult { real, imag }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fft(100);
        assert_eq!(result.len(), 100);
    }
}
