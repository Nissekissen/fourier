pub mod bar_visualizer;
pub mod scrolling_visualizer;

use piston_window::{Context, G2d, Graphics};

pub trait Visualizer {
    fn push(&mut self, data: Vec<f64>);
    fn draw(&self, c: &Context, g: &mut G2d);
}
