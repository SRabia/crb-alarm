use std::{ops::Div, vec};

use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Arc {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub arc_perc: f64, // percentage arc 0 to 100
    pub color: Color,
}

impl Shape for Arc {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let arc_completion = 360.0 - (self.arc_perc) * 360.0;

        let arc_completion = arc_completion as u32;
        for angle in 0..arc_completion {
            let radians = f64::from(angle).to_radians();
            let circle_x = self.radius.mul_add(radians.cos(), self.x);
            let circle_y = self.radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ZigZag {
    pub x: f64,
    pub y: f64,
    pub wid: f64,
    pub height: f64,
    pub fill_perc: f64, // this is a percentage
    pub color: Color,
}

// impl ZigZag {
//     pub fn new(x: f64, y: f64, wid: f64, height: u32, color: Color) -> Self {
//         Self {
//             x,
//             y,
//             wid,
//             height,
//             color,
//         }
//     }
// }

impl Shape for ZigZag {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let nb_lines = self.wid.min(self.height) as u32 + 1;
        let total = self.wid * self.height;
        let stop = (self.fill_perc * total) as u64;
        let mut increment = 0;

        for d in 0..nb_lines {
            if increment > stop {
                return;
            }
            let mut y = d;
            for x in 0..d {
                increment += 1;
                if let Some((x, y)) = painter.get_point(x as f64, y as f64) {
                    painter.paint(x, y, self.color);
                }
                y -= 1;
            }
        }
    }
}
