use std::{
    ops::{Div, Sub},
    time::{Duration, Instant},
};

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    symbols::{border, Marker},
    text::{Line, Text},
    widgets::{
        block::Title,
        canvas::{Canvas, Circle, Rectangle},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

const ZONE: u16 = 50;

struct App {
    x: f64,
    y: f64,
    ball: Circle,
    playground: Rect,
    vx: f64,
    vy: f64,
    tick_count: u64,
    marker: Marker,
}

impl App {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            ball: Circle {
                x: 20.0,
                y: 40.0,
                radius: 10.0,
                color: Color::Yellow,
            },
            playground: Rect::new(0, 0, ZONE, ZONE),
            vx: 1.0,
            vy: 1.0,
            tick_count: 0,
            marker: Marker::Braille,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => self.y += 1.0,
                        KeyCode::Up | KeyCode::Char('k') => self.y -= 1.0,
                        KeyCode::Right | KeyCode::Char('l') => self.x += 1.0,
                        KeyCode::Left | KeyCode::Char('h') => self.x -= 1.0,
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        // only change marker every 180 ticks (3s) to avoid stroboscopic effect
        // if (self.tick_count % 180) == 0 {
        //     // self.marker = match self.marker {
        //     //     Marker::Dot => Marker::Braille,
        //     //     Marker::Braille => Marker::Block,
        //     //     Marker::Block => Marker::HalfBlock,
        //     //     Marker::HalfBlock => Marker::Bar,
        //     //     Marker::Bar => Marker::Dot,
        //     // };
        // }
        // bounce the ball by flipping the velocity vector
        let ball = &self.ball;
        let playground = self.playground;
        if ball.x - ball.radius < f64::from(playground.left())
            || ball.x + ball.radius > f64::from(playground.right())
        {
            self.vx = -self.vx;
        }
        if ball.y - ball.radius < f64::from(playground.top())
            || ball.y + ball.radius > f64::from(playground.bottom())
        {
            self.vy = -self.vy;
        }

        self.ball.x += self.vx;
        self.ball.y += self.vy;
    }

    fn chrono_timeout(&self) -> impl Widget + '_ {
        let title = Title::from("Timeout Chrono".bold());
        let instructions = Title::from(Line::from(vec![
            "one more second ".into(),
            "<Left>".blue().bold(),
            " 1 less second ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<q>".red().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(ratatui::layout::Alignment::Center))
            .title(
                instructions
                    .alignment(ratatui::layout::Alignment::Center)
                    .position(ratatui::widgets::block::Position::Bottom),
            )
            .border_set(border::THICK);

        // let timeout_text = Text::from(vec![Line::from(vec![
        //     "Time Left: ".into()
        //     self.x.to_string().yellow(),
        // ])]);
        let timeout_text = Text::from(vec![
            Line::from(" Time Left: ").centered(),
            Line::from(self.x.to_string().yellow()),
        ])
        .centered();
        Paragraph::new(timeout_text).centered().block(block)
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        // let area_squared = Rect::new(0, 0, 50, 50);
        // fill(area_squared, frame.buffer_mut(), "█", Color::Red);

        // frame.render_widget(self.pong_canvas(), area_squared);
        frame.render_widget(self.boxes_canvas(area), area);
        let area_chrono = center_area(area, 30, 10);
        frame.render_widget(self.chrono_timeout(), area_chrono);
        let block_info = Block::bordered()
            .title(
                Title::from(format!(
                    "{}x{} -> {}x{}",
                    frame.area().width,
                    frame.area().height,
                    0,
                    0
                ))
                .alignment(ratatui::layout::Alignment::Right)
                .position(ratatui::widgets::block::Position::Bottom),
            )
            .border_set(border::THICK);
        frame.render_widget(block_info, area);
    }

    fn pong_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            // .block(Block::bordered().title("Pong"))
            .marker(self.marker)
            .paint(|ctx| {
                // let radius = self.playground.x.pow(2) + self.playground.y.pow(2);
                // let radius = f64::sqrt(radius as f64);
                let zone = ZONE as f64;
                // ctx.draw(&Circle {
                //     x: zone / 2.0,
                //     y: zone / 2.0,
                //     radius: (zone - 5.0) / 2.0,
                //     color: Color::Yellow,
                // });
                // ctx.draw(&Rectangle {
                //     x: zone / 2.0,
                //     y: zone / 2.0,
                //     width: (zone - 5.0) / 2.0,
                //     height: (zone - 5.0) / 2.0,
                //     color: Color::Yellow,
                // })
                for i in 0..=11 {
                    ctx.draw(&Rectangle {
                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                        y: 2.0,
                        width: f64::from(i),
                        height: f64::from(i),
                        color: Color::Red,
                    });
                    ctx.draw(&Rectangle {
                        x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                        y: 21.0,
                        width: f64::from(i),
                        height: f64::from(i),
                        color: Color::Blue,
                    });
                }

                // ctx.draw(&self.ball);
            })
            .x_bounds([0.0, ZONE as f64])
            .y_bounds([0.0, ZONE as f64])
    }

    fn boxes_canvas(&self, area: Rect) -> impl Widget {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        // this is the aspect ratio adjustement.. I don't know if will work for all screen ratio?
        let top = f64::from(area.height).mul_add(2.0, -4.0);
        Canvas::default()
            .block(Block::bordered().title("Rects"))
            .marker(self.marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(move |ctx| {
                // for i in 0..=11 {
                //     ctx.draw(&Rectangle {
                //         x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                //         y: 2.0,
                //         width: f64::from(i),
                //         height: f64::from(i),
                //         color: Color::Red,
                //     });
                //     ctx.draw(&Circle {
                //         x: f64::from(i * i + 3 * i) / 2.0 + 2.0,
                //         y: 21.0,
                //         // width: f64::from(i),
                //         radius: f64::from(i),
                //         color: Color::Blue,
                //     });
                // }
                // ctx.draw(&Rectangle {
                //     x: 3.0,
                //     y: 2.0,
                //     width: right - 5.0,
                //     height: top - 5.0,
                //     color: Color::Blue,
                // });
                for i in 0..10 {
                    ctx.draw(&Circle {
                        // x: right / 2.0,
                        // y: top / 2.0,
                        x: right.div(2.0),
                        y: top.div(2.0),
                        radius: right.div(4.0).sub(i as f64),
                        // radius: (right.powf(2.0) + top.powf(2.0)).sqrt().div(2.0),
                        // width: f64::from(area.width) - 5.0,
                        // height: f64::from(area.height) - 5.0,
                        color: Color::Red,
                    });
                }
            })
    }
}

fn center_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
