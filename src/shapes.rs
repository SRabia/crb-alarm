use std::{
    f32::consts::PI,
    ops::{Div, Sub},
};

use ratatui::{
    style::Color,
    symbols,
    widgets::canvas::{Painter, Shape},
};

#[derive(Debug)]
pub enum ShapeSelect {
    ArcSelect(Arc),
    SpiralSelect(Spiral),
    ZigZagSelect(ZigZag),
}

impl ShapeSelect {
    pub fn select_from(select: u32, c: Color) -> Self {
        match select {
            0 => Self::ArcSelect(Arc::new(8, c)),
            1 => Self::SpiralSelect(Spiral::new(c)),
            2 => Self::ZigZagSelect(ZigZag::new(5, c)),
            _ => Self::SpiralSelect(Spiral::new(c)), //TODO: have something more smart here
        }
    }
    pub fn get_marker(&self) -> symbols::Marker {
        match self {
            ShapeSelect::ArcSelect(_) => Arc::get_marker(),
            ShapeSelect::SpiralSelect(_) => Spiral::get_marker(),
            ShapeSelect::ZigZagSelect(_) => ZigZag::get_marker(),
        }
    }
    pub fn with_bgcolor(self, bg: Color) -> Self {
        match self {
            ShapeSelect::ArcSelect(s) => ShapeSelect::ArcSelect(s.with_bgcolor(bg)),
            ShapeSelect::SpiralSelect(s) => ShapeSelect::SpiralSelect(s.with_gbcolor(bg)),
            ShapeSelect::ZigZagSelect(s) => ShapeSelect::ZigZagSelect(s.with_gbcolor(bg)),
        }
    }
}

impl Shape for ShapeSelect {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        match self {
            ShapeSelect::ArcSelect(a) => a.draw(painter),
            ShapeSelect::SpiralSelect(s) => s.draw(painter),
            ShapeSelect::ZigZagSelect(z) => z.draw(painter),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Arc {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub thickness: usize,
    pub arc_perc: f64, // percentage arc 0 to 100
    pub color: Color,
    pub bgcolor: Option<Color>,
}

impl Arc {
    pub fn new(thickness: usize, color: Color) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
            thickness,
            arc_perc: 0.0,
            color,
            bgcolor: None,
        }
    }
    pub fn with_bgcolor(mut self, bg: Color) -> Self {
        self.bgcolor = Some(bg);
        self
    }

    pub fn get_marker() -> symbols::Marker {
        symbols::Marker::Dot
    }

    pub fn center(self, width: f64, height: f64, arc_perc: f64) -> Self {
        Self {
            x: width.div(2.0),
            y: height.div(2.0),
            radius: width.min(height).div(2.0),
            thickness: self.thickness,
            arc_perc,
            color: self.color,
            bgcolor: self.bgcolor,
        }
    }
}

impl Shape for Arc {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        if let Some(c) = self.bgcolor {
            for t in 0..self.thickness {
                let radius = self.radius.sub(t as f64);
                for angle in 0..360 {
                    let radians = f64::from(angle).to_radians();
                    let circle_x = radius.mul_add(radians.cos(), self.x);
                    let circle_y = radius.mul_add(radians.sin(), self.y);
                    if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                        painter.paint(x, y, c);
                    }
                }
            }
        }
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
    pub bgcolor: Option<Color>,
}

impl ZigZag {
    pub fn new(gap: usize, color: Color) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            size: 0.0,
            gap,
            fill_perc: 0.0,
            color,
            bgcolor: None,
        }
    }
    pub fn get_marker() -> symbols::Marker {
        symbols::Marker::HalfBlock
    }

    pub fn with_gbcolor(mut self, bg: Color) -> Self {
        self.bgcolor = Some(bg);
        self
    }

    pub fn center(self, width: f64, height: f64, fill_perc: f64) -> Self {
        let size = width.min(height);
        let x = width.div(2.0) - size.div(2.0);
        let x = x.max(0.0);
        let y = height.div(2.0) - size.div(2.0);
        let y = y.max(0.0);

        Self {
            x,
            y,
            size,
            gap: self.gap,
            fill_perc,
            color: self.color,
            bgcolor: self.bgcolor,
        }
    }

    fn drawfill(&self, painter: &mut Painter<'_, '_>, fill_per: f64, c: Color) {
        let fill_limit = (fill_per * (self.size + self.gap as f64).powi(2)) as usize;
        let mut filled = 0;

        // rust 101 for the noob! filled and painter variable are owned by this
        // closure now that's why you got error! .... just pure pain
        let mut draw_point = |x, y, inc| {
            filled += inc;
            if filled > fill_limit {
                return false;
            }
            if let Some((zx, zy)) =
                painter.get_point((x + self.x as usize) as f64, (y + self.y as usize) as f64)
            {
                painter.paint(zx, zy, c);
            }
            true
        };

        let size = self.size as usize;
        //let offset_x = self.x as usize;

        let mut going_up = true;
        for d in (0..size).step_by(self.gap) {
            let mut dy = d;
            going_up = !going_up;
            for dx in 0..d {
                let (x, y) = if going_up { (d - dx, dx) } else { (dx, dy) };
                if !draw_point(x, y, self.gap) {
                    return;
                }

                dy = dy.saturating_sub(1);
            }
            for g in 1..self.gap {
                let (x, y) = if going_up { (0, d + g) } else { (d + g, 0) };
                if !draw_point(x, y, 1) {
                    return;
                }
            }
        }

        for d in (0..size).step_by(self.gap) {
            let mut dy = size;
            going_up = !going_up;
            for dx in d..(size) {
                let (x, y) = if going_up {
                    (size - (dx - d), dx)
                } else {
                    (dx, dy)
                };
                if !draw_point(x, y, self.gap) {
                    return;
                }

                dy = dy.saturating_sub(1);
            }

            for g in 1..self.gap {
                let (x, y) = if going_up {
                    (d + g, size)
                } else {
                    (size, d + g)
                };
                if !draw_point(x, y, self.gap) {
                    return;
                }
            }
        }
    }
}

impl Shape for ZigZag {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        if let Some(c) = self.bgcolor {
            self.drawfill(painter, 1.0, c);
        }
        self.drawfill(painter, self.fill_perc, self.color);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Spiral {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub completion_perc: f64, // percentage arc 0 to 100
    pub color: Color,
    pub bgcolor: Option<Color>,
}

impl Spiral {
    pub fn new(color: Color) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            completion_perc: 0.0,
            color,
            bgcolor: None,
        }
    }

    pub fn center(self, width: f64, height: f64, arc_perc: f64) -> Self {
        Self {
            x: width.div(2.0),
            y: height.div(2.0),
            radius: width.min(height).div(2.0),
            completion_perc: arc_perc,
            color: self.color,
            bgcolor: self.bgcolor,
        }
    }

    pub fn with_gbcolor(mut self, bg: Color) -> Self {
        self.bgcolor = Some(bg);
        self
    }

    pub fn get_marker() -> symbols::Marker {
        symbols::Marker::HalfBlock
    }

    fn drawfill(&self, painter: &mut Painter<'_, '_>, fill: f64, c: Color) {
        // Archimedean spiral: r =  a + b*theta

        let a = -self.radius;
        let b = 1.0;
        let range = -a / (b * PI as f64);
        let range = (180.0 * range * fill) as u32;
        for angle in 0..range {
            let radians = f64::from(angle).to_radians();
            let radius = a + (b * radians);

            //convert to x,y coordinate
            let circle_x = radius.mul_add(radians.cos(), self.x);
            let circle_y = radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, c);
            }
        }
    }
}

impl Shape for Spiral {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        if let Some(c) = self.bgcolor {
            self.drawfill(painter, 1.0, c);
        }

        self.drawfill(painter, self.completion_perc, self.color);
    }
}
