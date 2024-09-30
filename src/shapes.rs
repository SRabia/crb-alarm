use std::ops::{Div, Sub};

use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Arc {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub thickness: usize,
    pub arc_perc: f64, // percentage arc 0 to 100
    pub color: Color,
}

impl Arc {
    pub fn centered(
        width: f64,
        height: f64,
        thickness: usize,
        arc_perc: f64,
        color: Color,
    ) -> Self {
        Self {
            x: width.div(2.0),
            y: height.div(2.0),
            radius: width.min(height).div(2.0),
            thickness,
            arc_perc,
            color,
        }
    }
}

impl Shape for Arc {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let arc_completion = (self.arc_perc) * 360.0;

        let arc_completion = arc_completion as u32;
        for t in 0..self.thickness {
            let radius = self.radius.sub(t as f64);
            for angle in 0..arc_completion {
                let radians = f64::from(angle).to_radians();
                let circle_x = radius.mul_add(radians.cos(), self.x);
                let circle_y = radius.mul_add(radians.sin(), self.y);
                if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                    painter.paint(x, y, self.color);
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ZigZag {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub gap: usize,
    pub fill_perc: f64, // this is a percentage
    pub color: Color,
}

impl ZigZag {
    pub fn centered(width: f64, height: f64, gap: usize, fill_perc: f64, color: Color) -> Self {
        let size = width.min(height);
        let x = width.div(2.0) - size.div(2.0);
        let x = x.max(0.0);
        let y = height.div(2.0) - size.div(2.0);
        let y = y.max(0.0);

        Self {
            x,
            y,
            size,
            gap,
            fill_perc,
            color,
        }
    }
}

impl Shape for ZigZag {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        let total = self.size * self.size;

        let fill = (self.fill_perc * total) as usize;
        let mut fill_count = 0;
        let size = self.size as usize;
        let offset_x = self.x as usize;
        let offset_y = self.y as usize;

        let mut change_dir = true;
        for d in (0..size).step_by(self.gap) {
            let mut dy = d;
            change_dir = !change_dir;
            for dx in 0..d {
                fill_count += self.gap;
                if fill_count > fill {
                    return;
                }

                let (px, py, c) = if change_dir {
                    (d - dx + offset_x, dx + offset_y, Color::Red)
                } else {
                    (dx + offset_x, dy + offset_y, Color::Blue)
                };
                //let (px, py, c) = (x, y, Color::Blue);
                if let Some((zx, zy)) = painter.get_point(px as f64, py as f64) {
                    painter.paint(zx, zy, c);
                }
                dy = dy.saturating_sub(1);
            }
        }

        for d in (0..size).step_by(self.gap) {
            let mut dy = size;
            change_dir = !change_dir;
            for dx in d..(size) {
                fill_count += self.gap;
                if fill_count > fill {
                    return;
                }

                let (px, py, c) = if change_dir {
                    (size - (dx - d) + offset_x, dx + offset_y, Color::Red)
                } else {
                    (dx + offset_x, dy + offset_y, Color::Blue)
                };
                if let Some((zx, zy)) = painter.get_point(px as f64, py as f64) {
                    painter.paint(zx, zy, c);
                }
                dy = dy.saturating_sub(1);
            }
        }
    }
}
