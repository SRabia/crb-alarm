use crate::shapes;
use crate::shapes::ShapeSelect;

use ratatui::{
    layout::Rect,
    prelude::Buffer,
    style::Color,
    widgets::{canvas::Canvas, Block, Widget},
};
use std::time::Duration;

#[derive(Debug)]
pub struct AnimChrono {
    shapes_selected: ShapeSelect,
    pub timeout: Duration, // todo: use u64 msecs
    pub remaining: Duration,
    complete: bool,
}

impl AnimChrono {
    pub fn new(shapes_selected: ShapeSelect, timeout: Duration) -> Self {
        Self {
            shapes_selected,
            timeout,
            remaining: timeout,
            complete: false,
        }
    }
    pub fn get_time_left_formated(&self) -> (u64, u64, u64) {
        let m = self.remaining.as_secs() / 60;
        let s = self.remaining.as_secs() % 60;
        let h = self.remaining.as_secs() / 3600;
        (h, m, s)
    }

    pub fn update<F>(&mut self, elapsed: Duration, cb_complete: F)
    where
        F: Fn(),
    {
        self.remaining = self.remaining.saturating_sub(elapsed);
        if !self.complete && self.remaining.as_secs() == 0 {
            self.complete = true;
            cb_complete();
        }
    }

    pub fn increase_timeout(&mut self, tm: u64) {
        self.timeout = self.timeout.saturating_add(Duration::new(tm, 0));
        self.remaining = self.remaining.saturating_add(Duration::new(tm, 0));
    }

    pub fn decrease_timeout(&mut self, tm: u64) {
        self.timeout = self.timeout.saturating_sub(Duration::new(tm, 0));
        self.remaining = self.remaining.saturating_sub(Duration::new(tm, 0));
    }
}

impl Widget for &AnimChrono {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let complete_perc = self.remaining.as_millis() as f64 / self.timeout.as_millis() as f64;
        let complete_perc = 1.0 - complete_perc;
        let top = f64::from(area.height).mul_add(2.0, -4.0);

        //TODO: can avoid the clone if we don't stop the shapes_selected in App
        // instead we create it in this function
        let shape = match &self.shapes_selected {
            shapes::ShapeSelect::ArcSelect(a) => {
                ShapeSelect::ArcSelect(a.clone().center(right, top, complete_perc))
            }
            shapes::ShapeSelect::SpiralSelect(s) => {
                ShapeSelect::SpiralSelect(s.clone().center(right, top, complete_perc))
            }
            shapes::ShapeSelect::ZigZagSelect(z) => {
                ShapeSelect::ZigZagSelect(z.clone().center(right, top, complete_perc))
            }
        };
        let shape = shape.with_bgcolor(Color::DarkGray);
        let marker = self.shapes_selected.get_marker();
        let canvas = Canvas::default()
            .block(Block::bordered())
            .marker(marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(move |ctx| {
                ctx.draw(&shape);
            });
        canvas.render(area, buf);
    }
}
