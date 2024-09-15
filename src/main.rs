use cbr_alarm::arc::Arc;
use cbr_alarm::cli;
use clap::Parser;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{self, Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    symbols::{border, Marker},
    text::{Line, Text, ToSpan},
    widgets::{block::Title, canvas::Canvas, Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{
    ops::{Div, Sub},
    process::Command,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let mut tm_s = Duration::from_secs(5);
    if let Some(cmd) = args.cmd {
        tm_s = match cmd {
            cli::Commands::Timeout(t) => t.parse().unwrap(),
        };
    }

    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new(tm_s).run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    timeout: Duration, // todo: use u64 msecs
    remaining: Duration,
    fps: Fps,
}

pub struct Fps {
    last_frame_update: Instant,
    frame_count: u32,
    fps: f64,
}

impl Fps {
    pub fn new() -> Self {
        Self {
            last_frame_update: Instant::now(),
            frame_count: 0,
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        let elapsed = (now - self.last_frame_update).as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f64 / elapsed;
            self.last_frame_update = now;
            self.frame_count = 0;
        }
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }
}

impl Default for Fps {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            remaining: timeout,
            fps: Fps::default(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        loop {
            self.fps.update();
            terminal.draw(|frame| self.draw(frame))?;
            // timeout = 16 - loop_interval
            let interval_tick = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(interval_tick)? {
                // block for 16ms
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.timeout = self.timeout.saturating_sub(Duration::new(60, 0));
                            self.remaining = self.remaining.saturating_sub(Duration::new(60, 0));
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.timeout = self.timeout.saturating_add(Duration::new(60, 0));
                            self.remaining = self.timeout.saturating_add(Duration::new(60, 0));
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            self.timeout = self.timeout.saturating_add(Duration::new(1, 0));
                            self.remaining = self.timeout.saturating_add(Duration::new(1, 0));
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            self.timeout = self.timeout.saturating_sub(Duration::new(1, 0));
                            self.remaining = self.timeout.saturating_sub(Duration::new(1, 0));
                        }
                        _ => {}
                    }
                }
            }

            let elapsed = last_tick.elapsed();
            if elapsed >= tick_rate {
                last_tick = Instant::now();
                self.remaining = self.remaining.saturating_sub(elapsed);
                if self.remaining.as_secs() == 0 {
                    self.on_timeout_complete();
                    break Ok(());
                }
            }
        }
    }

    fn get_tm_info_widget(&self) -> impl Widget + '_ {
        let title = Title::from("Timeout Chrono".bold());
        let instructions = Title::from(Line::from(vec![
            //todo: bad render here use mult line
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

        let timeout = self.remaining.as_secs().to_string();
        let timeout_text = Text::from(vec![
            Line::from(" Time Left: ").centered(),
            Line::from(timeout.yellow()),
        ])
        .centered();
        Paragraph::new(timeout_text).centered().block(block)
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        frame.render_widget(self.get_tm_animation_widget(area), area);
        let area_chrono = get_center_area(area, 20, 10);
        frame.render_widget(self.get_tm_info_widget(), area_chrono);
        let message_fps = format!("{:.2} FPS", self.fps.fps());
        let title_fps = Title::from(message_fps.to_span().dim())
            .alignment(layout::Alignment::Left)
            .position(ratatui::widgets::block::Position::Top);

        let block_info = Block::bordered()
            .title(
                Title::from(format!(
                    "{}x{} -> {:?}::{:?}",
                    self.remaining.as_millis() as f64,
                    self.timeout.as_millis() as f64,
                    self.timeout,
                    0
                ))
                .alignment(ratatui::layout::Alignment::Right)
                .position(ratatui::widgets::block::Position::Bottom),
            )
            .title(title_fps)
            .border_set(border::THICK);
        frame.render_widget(block_info, area);
    }

    fn get_tm_animation_widget(&self, area: Rect) -> impl Widget {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let arc_completion =
            360.0 - (self.remaining.as_millis() as f64 / self.timeout.as_millis() as f64) * 360.0;

        // this is the aspect ratio adjustement.. I don't know if will work for all screen ratio?
        let top = f64::from(area.height).mul_add(2.0, -4.0);
        Canvas::default()
            .block(Block::bordered().title("Rects"))
            .marker(Marker::Dot)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(move |ctx| {
                for i in 0..5 {
                    ctx.draw(&Arc {
                        x: right.div(2.0),
                        y: top.div(2.0),
                        radius: top.min(right).div(2.0).sub(i as f64),
                        arc: arc_completion as u32,
                        color: Color::Red,
                    });
                }
            })
    }

    fn on_timeout_complete(&self) {
        Command::new("wall")
            .arg("Task Compelte!")
            .spawn()
            .expect("Failed to send wall message");
    }
}

fn get_center_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
