#![allow(unused)]

use piston_window::{
    clear, color::GRAY, line, rectangle, types::Color, Context, Graphics, Line, Rectangle,
    Transformed,
};

#[derive(Default)]
pub struct Plot {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    caption: Option<Label>,
    x_axis_label: Option<Label>,
    y_axis_label: Option<Label>,
}

struct Label {
    pub content: String,
    pub x: f64,
    pub y: f64,
}

impl Plot {
    pub fn new(pos: (f64, f64), size: (f64, f64)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            width: size.0,
            height: size.1,
            ..Default::default()
        }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn caption(mut self, content: &str) -> Self {
        let x = self.x + self.width / 2.0;
        let y = self.y - 10.0;

        self.caption = Some(Label {
            content: content.to_owned(),
            x,
            y,
        });

        self
    }

    pub fn x_axis_label(mut self, content: &str) -> Self {
        let x = self.x + self.width / 2.0;
        let y = self.y + self.height + 10.0;

        self.x_axis_label = Some(Label {
            content: content.to_owned(),
            x,
            y,
        });

        self
    }

    pub fn y_axis_label(mut self, content: &str) -> Self {
        let x = self.x - 10.0;
        let y = self.y - 10.0;

        self.y_axis_label = Some(Label {
            content: content.to_owned(),
            x,
            y,
        });

        self
    }

    pub fn draw_bg<G: Graphics>(&self, c: &Context, g: &mut G) {
        clear([1.0; 4], g);
        let line_color = [0.3, 0.3, 0.3, 1.0];

        line(
            line_color,
            1.0,
            [self.x, self.y, self.x, self.y + self.height],
            c.transform,
            g,
        );

        line(
            line_color,
            1.0,
            [
                self.x,
                self.y + self.height,
                self.x + self.width,
                self.y + self.height,
            ],
            c.transform,
            g,
        );
    }

    pub fn point<G: Graphics>(&self, color: Color, x: f64, y: f64, c: &Context, g: &mut G) {
        rectangle(
            color,
            [self.x + x, self.y + y, self.x + x, self.y + y],
            c.transform,
            g,
        );
    }

    pub fn rect<G: Graphics>(&self, color: Color, corners: [f64; 4], c: &Context, g: &mut G) {
        let transform = c.trans(self.x, self.y).transform;
        rectangle(color, corners, transform, g);
    }
}
