use piston_window::{color::BLACK, rectangle, Context, G2d, Graphics};

use super::Visualizer;
use crate::MAX_LOUDNESS;

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
}

impl Visualizer for BarVisualizer {
    fn push(&mut self, data: Vec<f64>) {
        assert!(data.len() >= self.bars.len());
        let data_per_bar = data.len() / self.bars.len();

        // Apply easing to smooth transitions
        const EASE_FACTOR: f64 = 0.95;
        for i in 0..self.bars.len() {
            let start_index = i * data_per_bar;
            let end_index = (start_index + data_per_bar).min(data.len());
            let sum = data[start_index..end_index].iter().sum::<f64>();

            let bar = &mut self.bars[i];
            *bar = *bar * EASE_FACTOR + sum / data_per_bar as f64 * (1.0 - EASE_FACTOR);
        }
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
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
