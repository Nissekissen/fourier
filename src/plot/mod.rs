#![allow(unused)]

#[derive(Default)]
pub struct Plot {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    top_label: Option<Label>,
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
}
