use piston_window::{color::BLACK, rectangle, Context, Graphics};

use super::Visualizer;
use crate::{utils::circular_vec::CircularVec, MAX_LOUDNESS};

pub struct ScrollingVisualizer {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    data: CircularVec<Vec<f64>>,
    num_bars: usize,
}

impl ScrollingVisualizer {
    pub fn new(x: f64, y: f64, width: f64, height: f64, num_bars: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
            data: CircularVec::new(width as usize / 2),
            num_bars,
        }
    }
}

impl Visualizer for ScrollingVisualizer {
    fn draw<G: Graphics>(&mut self, data: &[f64], c: &Context, g: &mut G) {
        assert_eq!(data.len(), self.num_bars);
        self.data.push(data.to_owned());
        let bar_height = self.height / self.num_bars as f64;

        for (x, column) in self.data.iter().rev().enumerate() {
            for (y, loudness) in column.iter().enumerate() {
                let normalized_loudness = loudness / MAX_LOUDNESS;
                let color = [normalized_loudness as f32, 0.0, 0.0, 1.0];
                rectangle(color, [x as f64, y as f64, 1.0, bar_height], c.transform, g);
            }
        }

        rectangle(
            BLACK,
            [self.data.len() as f64, self.y, 2.0, self.height],
            c.transform,
            g,
        );
    }
}
