use piston_window::{color::BLACK, rectangle, Context, Graphics};

use super::Visualizer;
use crate::MAX_LOUDNESS;
use fft_lib::Frequencies;

pub enum Rotation {
    Up,
    Down,
    Left,
    Right,
}

pub struct BarVisualizer {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    rotation: Rotation,
    bars: Vec<f64>,
}

impl BarVisualizer {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rotation: Rotation,
        num_bars: usize,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            rotation,
            bars: vec![0.0; num_bars],
        }
    }

    // New function to bin frequencies logarithmically
    pub fn bin_frequencies(&self, frequencies: &Frequencies) -> Vec<f64> {
        let num_bins = self.bars.len();
        let mut binned_amplitudes = vec![0.0; num_bins];
        let mut bin_counts = vec![0; num_bins];

        // Get the minimum and maximum frequencies
        let min_freq = frequencies.frequencies[0];
        let max_freq = frequencies.frequencies[frequencies.frequencies.len() - 1];

        // Create logarithmically spaced bin edges
        let bin_edges: Vec<f64> = (0..=num_bins)
            .map(|i| min_freq * (max_freq / min_freq).powf(i as f64 / num_bins as f64))
            .collect();

        // Assign amplitudes to bins
        for (freq, amp) in frequencies
            .frequencies
            .iter()
            .zip(frequencies.amplitudes.iter())
        {
            // Find the bin index for this frequency
            let bin_index = bin_edges
                .iter()
                .position(|&edge| edge > *freq)
                .unwrap_or(num_bins)
                - 1;

            if bin_index < num_bins {
                binned_amplitudes[bin_index] += amp;
                bin_counts[bin_index] += 1;
            }
        }

        // Normalize the binned amplitudes
        for i in 0..num_bins {
            if bin_counts[i] > 0 {
                binned_amplitudes[i] /= bin_counts[i] as f64;
            }
        }

        binned_amplitudes
    }
}

impl Visualizer for BarVisualizer {
    fn push(&mut self, data: Vec<f64>) {
        // Instead of directly using the input data, we'll create a Frequencies struct
        // and use bin_frequencies to process it
        let frequencies = Frequencies {
            frequencies: (0..data.len()).map(|i| i as f64).collect(),
            total_samples: data.len(),
            amplitudes: data,
            sample_rate: 44100, // This value doesn't matter for binning
            start_time: 0.0,    // This value doesn't matter for binning
        };

        let binned_data = self.bin_frequencies(&frequencies);
        assert_eq!(binned_data.len(), self.bars.len());

        for (new_height, height) in binned_data.iter().zip(self.bars.iter_mut()) {
            // Apply easing to the new height
            *height = *height * 0.9 + *new_height * 0.1;
        }
    }

    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        let (same_dir_len, other_dir_len) = match self.rotation {
            Rotation::Up | Rotation::Down => (self.height, self.width),
            Rotation::Left | Rotation::Right => (self.width, self.height),
        };
        let (same_dir_offset, other_dir_offset) = match self.rotation {
            Rotation::Up | Rotation::Down => (self.x, self.y),
            Rotation::Left | Rotation::Right => (self.y, self.x),
        };
        let bar_thickness = other_dir_len / self.bars.len() as f64 * 0.8;

        for (i, height) in self.bars.iter().enumerate() {
            let bar_same_dir_offset = (i as f64 / self.bars.len() as f64) * (other_dir_len - 10.0)
                + bar_thickness * 0.1
                + 5.0;

            let normalized_height = height.abs() / MAX_LOUDNESS;
            let bar_height = (normalized_height * same_dir_len as f64).min(same_dir_len as f64);

            let (x, y, width, height) = match self.rotation {
                Rotation::Up => (
                    self.x + bar_same_dir_offset,
                    self.y + self.height,
                    bar_thickness,
                    -bar_height,
                ),
                Rotation::Down => (
                    self.x + bar_same_dir_offset,
                    self.y,
                    bar_thickness,
                    bar_height,
                ),
                Rotation::Left => (
                    self.x,
                    self.y + bar_same_dir_offset,
                    bar_height,
                    bar_thickness,
                ),
                Rotation::Right => (
                    self.x + self.width,
                    self.y + bar_same_dir_offset,
                    -bar_height,
                    bar_thickness,
                ),
            };

            rectangle(BLACK, [x, y, width, height], c.transform, g);
        }
    }
}
