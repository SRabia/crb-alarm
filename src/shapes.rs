use std::{
    f32::consts::PI,
    ops::{Div, Sub},
};

use ratatui::{
    style::Color,
    symbols,
    widgets::canvas::{self, Painter, Shape},
};

pub enum ShapeSelect {
    ArcSelect,
    SpiralSelect,
    ZigZagSelect,
}

pub trait AnimationShape {
    fn create(width: f64, height: f64, animation_completion: f64) -> Self;
    fn marker() -> symbols::Marker;
    fn draw_on_canvas(&self, ctx: &mut canvas::Context);
}

impl ShapeSelect {
    pub fn select_from(select: u32) -> Self {
        match select {
            0 => Self::ArcSelect,
            1 => Self::SpiralSelect,
            2 => Self::ZigZagSelect,
            _ => Self::SpiralSelect,
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
}

impl Arc {
    pub fn center(width: f64, height: f64, thickness: usize, arc_perc: f64, color: Color) -> Self {
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
        let fill_limit = (self.fill_perc * (self.size + self.gap as f64).powi(2)) as usize;
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
                painter.paint(zx, zy, self.color);
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

            // if d + offset_x + self.gap > size + offset_x {
            //     break;
            // }

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

pub struct Spiral {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub completion_perc: f64, // percentage arc 0 to 100
    pub color: Color,
}

impl Spiral {
    pub fn centered(width: f64, height: f64, arc_perc: f64, color: Color) -> Self {
        Self {
            x: width.div(2.0),
            y: height.div(2.0),
            radius: width.min(height).div(2.0),
            completion_perc: arc_perc,
            color,
        }
    }
}

impl Shape for Spiral {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        // Archimedean spiral: r =  a + b*theta

        let a = -self.radius;
        let b = 1.0;
        let range = -a / (b * PI as f64);
        let range = (180.0 * range * self.completion_perc) as u32;
        for angle in 0..range {
            let radians = f64::from(angle).to_radians();
            let radius = a + (b * radians);

            //convert to x,y coordinate
            let circle_x = radius.mul_add(radians.cos(), self.x);
            let circle_y = radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

impl AnimationShape for Arc {
    fn create(width: f64, height: f64, animation_completion: f64) -> Self {
        Self::center(width, height, 8, animation_completion, Color::Red)
    }
    fn marker() -> symbols::Marker {
        symbols::Marker::Dot
    }
    fn draw_on_canvas(&self, ctx: &mut canvas::Context) {
        ctx.draw(self);
    }
}

impl AnimationShape for ZigZag {
    fn create(width: f64, height: f64, animation_completion: f64) -> Self {
        Self::centered(width, height, 5, animation_completion, Color::Red)
    }
    fn marker() -> symbols::Marker {
        symbols::Marker::HalfBlock
    }
    fn draw_on_canvas(&self, ctx: &mut canvas::Context) {
        ctx.draw(self);
    }
}

impl AnimationShape for Spiral {
    fn create(width: f64, height: f64, animation_completion: f64) -> Self {
        Self::centered(width, height, animation_completion, Color::Red)
    }
    fn marker() -> symbols::Marker {
        symbols::Marker::HalfBlock
    }
    fn draw_on_canvas(&self, ctx: &mut canvas::Context) {
        ctx.draw(self);
    }
}

impl ShapeSelect {
    pub fn create_shape(
        &self,
        width: f64,
        height: f64,
        animation_completion: f64,
    ) -> Box<dyn AnimationShape> {
        match self {
            ShapeSelect::ArcSelect => Box::new(Arc::create(width, height, animation_completion)),
            ShapeSelect::SpiralSelect => {
                Box::new(Spiral::create(width, height, animation_completion))
            }
            ShapeSelect::ZigZagSelect => {
                Box::new(ZigZag::create(width, height, animation_completion))
            }
        }
    }
    pub fn get_marker(&self) -> symbols::Marker {
        match self {
            ShapeSelect::ArcSelect => Arc::marker(),
            ShapeSelect::SpiralSelect => Spiral::marker(),
            ShapeSelect::ZigZagSelect => ZigZag::marker(),
        }
    }
}
