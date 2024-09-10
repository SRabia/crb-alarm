use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Arc {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub arc: u32,
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
