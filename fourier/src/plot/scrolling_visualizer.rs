use piston_window::{
    color::{grey, BLACK},
    rectangle, Context, DrawState, G2d, G2dTexture, Graphics, ImageSize, Texture, TextureSettings,
    Transformed,
};

use super::Visualizer;
use crate::{utils::circular_vec::CircularVec, MAX_LOUDNESS};

pub struct ScrollingVisualizer {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    data: CircularVec<f64>,
    display_columns: usize,
}

impl ScrollingVisualizer {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        let display_columns = width as usize;

        Self {
            x,
            y,
            width,
            height,
            data: CircularVec::new(width as usize),
            display_columns,
        }
    }
}

impl Visualizer for ScrollingVisualizer {
    fn push(&mut self, data: Vec<f64>) {
        let height_multiplier = 1.0 / self.height;
        let sum = data.iter().sum::<f64>() / (height_multiplier * data.len() as f64);
        self.data.push(sum);
    }

    fn draw(&self, c: &Context, g: &mut G2d) {
        for (i, column) in self.data.iter().enumerate() {
            let height = column.min(1.0) * self.height;
            let rect = [
                self.x + self.width - self.data.len() as f64 + i as f64,
                self.y + self.height / 2.0 - height / 2.0,
                1.0,
                height,
            ];
            rectangle(BLACK, rect, c.transform, g);
        }
    }
}
