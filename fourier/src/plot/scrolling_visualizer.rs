use piston_window::{
    color::{grey, BLACK},
    rectangle, Context, G2dTexture, Graphics, ImageSize, Texture, TextureSettings, Transformed,
};

use super::Visualizer;
use crate::{utils::circular_vec::CircularVec, MAX_LOUDNESS};

// Use a buffer to store rectangle data for batch rendering
struct Buffer {
    vertices: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
}

impl Buffer {
    fn new(capacity: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(capacity * 6), // 6 vertices per rectangle (2 triangles)
            colors: Vec::with_capacity(capacity * 6),   // 1 color per vertex
        }
    }

    fn clear(&mut self) {
        self.vertices.clear();
        self.colors.clear();
    }

    // Add a rectangle to the buffer
    fn add_rect(&mut self, x: f64, y: f64, w: f64, h: f64, color: [f32; 4]) {
        // Convert to f32 for better performance
        let x = x as f32;
        let y = y as f32;
        let w = w as f32;
        let h = h as f32;

        // Vertex positions for the rectangle (2 triangles)
        let v0 = [x, y];
        let v1 = [x + w, y];
        let v2 = [x, y + h];
        let v3 = [x + w, y + h];

        // First triangle (v0, v1, v2)
        self.vertices.push(v0);
        self.vertices.push(v1);
        self.vertices.push(v2);

        // Second triangle (v2, v1, v3)
        self.vertices.push(v2);
        self.vertices.push(v1);
        self.vertices.push(v3);

        // Add colors for each vertex
        for _ in 0..6 {
            self.colors.push(color);
        }
    }
}

pub struct ScrollingVisualizer {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    data: CircularVec<Vec<f64>>,
    num_bars: usize,
    // Controls how many columns to display from the buffer
    display_columns: usize,
    // Controls how frequently to sample from the FFT data
    // Higher number = fewer bars = better performance
    frequency_sampling: usize,
    buffer: Buffer,
}

impl ScrollingVisualizer {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        num_bars: usize,
        sample_rate: usize,
    ) -> Self {
        let display_columns = width as usize / 2;

        Self {
            x,
            y,
            width,
            height,
            data: CircularVec::new(width as usize / 2),
            num_bars,
            display_columns,
            frequency_sampling,
            buffer: Buffer::new(display_columns * num_bars),
        }
    }
}

impl Visualizer for ScrollingVisualizer {
    fn push(&mut self, data: Vec<f64>) {
        self.data.push(data);
    }

    fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {
        // Calculate bar dimensions
        let bar_height = self.height / (self.num_bars / self.frequency_sampling) as f64;
        let bar_offset = self.height / (self.num_bars / self.frequency_sampling) as f64;

        let mut buffer = Buffer::new(self.display_columns * self.num_bars);
        buffer.clear();

        // Draw only a subset of the data to improve performance
        // Reduce the number of columns we render
        for (x, column) in self
            .data
            .iter()
            .rev()
            .take(self.display_columns)
            .enumerate()
        {
            // Sample only every Nth frequency to reduce the number of bars rendered
            for (y, loudness) in column.iter().step_by(self.frequency_sampling).enumerate() {
                let normalized_loudness = loudness / MAX_LOUDNESS;
                let color = grey((normalized_loudness as f32).min(1.0));

                // Make each bar wider to compensate for showing fewer columns
                // rectangle(
                //     color,
                //     [
                //         (self.width * 0.5) - x as f64 * 7.5 + 10.0, // Wider spacing
                //         y as f64 * bar_offset + 10.0,
                //         7.5, // Wider bars
                //         bar_height,
                //     ],
                //     c.transform,
                //     g,
                // );
            }
        }
    }
}
