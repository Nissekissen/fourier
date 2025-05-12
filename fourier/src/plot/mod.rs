pub mod bar_visualizer;
pub mod scrolling_visualizer;

use piston_window::{Context, Graphics};

pub trait Visualizer {
    fn push(&mut self, data: Vec<f64>);
    fn draw<G: Graphics>(&self, c: &Context, g: &mut G);
}

// pub fn draw_bg<G: Graphics>(&self, c: &Context, g: &mut G) {
//     clear([1.0; 4], g);
//     let line_color = [0.3, 0.3, 0.3, 1.0];

//     line(
//         line_color,
//         1.0,
//         [self.x, self.y, self.x, self.y + self.height],
//         c.transform,
//         g,
//     );

//     line(
//         line_color,
//         1.0,
//         [
//             self.x,
//             self.y + self.height,
//             self.x + self.width,
//             self.y + self.height,
//         ],
//         c.transform,
//         g,
//     );
// }

// pub fn point<G: Graphics>(&self, color: Color, x: f64, y: f64, c: &Context, g: &mut G) {
//     rectangle(
//         color,
//         [self.x + x, self.y + y, self.x + x, self.y + y],
//         c.transform,
//         g,
//     );
// }

// pub fn rect<G: Graphics>(&self, color: Color, corners: [f64; 4], c: &Context, g: &mut G) {
//     let transform = c.trans(self.x, self.y).transform;
//     rectangle(color, corners, transform, g);
// }

//     pub fn text<C, G>(
//         &self,
//         text: &str,
//         font_size: u32,
//         x: f64,
//         y: f64,
//         c: &Context,
//         g: &mut G,
//         cache: &mut C,
//     ) where
//         C: CharacterCache,
//         G: Graphics<Texture = <C as CharacterCache>::Texture>,
//     {
//         let transform = c.trans(self.x, self.y).transform;
//         piston_window::text(BLACK, font_size, text, cache, transform, g);
//     }
// }
