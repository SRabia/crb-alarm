use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

/// A circle with a given center and radius and with a given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Arc {
    /// `x` coordinate of the circle's center
    pub x: f64,
    /// `y` coordinate of the circle's center
    pub y: f64,
    /// Radius of the circle
    pub radius: f64,
    // in angle degree
    pub arc: u32,
    /// Color of the circle
    pub color: Color,
}

impl Shape for Arc {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for angle in 0..self.arc {
            let radians = f64::from(angle).to_radians();
            let circle_x = self.radius.mul_add(radians.cos(), self.x);
            let circle_y = self.radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
