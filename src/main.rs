use cbr_alarm::cli;
use cbr_alarm::fps;
use cbr_alarm::shapes;
use cbr_alarm::shapes::ShapeSelect;
use clap::Parser;
use color_eyre::Result;
use rand::Rng;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{self, Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text, ToSpan},
    widgets::{block::Title, canvas::Canvas, Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use rust_embed::RustEmbed;
use std::time::{Duration, Instant};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub fn format_duration(d: Duration) -> (u64, u64, u64) {
    let m = d.as_secs() / 60;
    let s = d.as_secs() % 60;
    let h = d.as_secs() / 3600;
    (h, m, s)
}

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
    fps: fps::Fps,
    shapes_selected: ShapeSelect,
}

impl App {
    fn new(timeout: Duration) -> Self {
        let rand_select = rand::thread_rng().gen_range(0..3);
        let s = shapes::ShapeSelect::select_from(rand_select);

        Self {
            timeout,
            remaining: timeout,
            fps: fps::Fps::default(),
            shapes_selected: s,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        let mut done = false;
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
                if !done && self.remaining.as_secs() == 0 {
                    done = true; // one shot
                    self.timeout_complete();
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

        let (h, m, s) = format_duration(self.remaining);

        let timeout = if h > 0 {
            format!("{}:{}:{}", h, m, s)
        } else {
            format!("{}:{}", m, s)
        };

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
                    "{}x{} -> {}::{:.3}",
                    area.width,
                    area.height,
                    f64::from(area.height).mul_add(2.0, -4.0),
                    1.0 - (self.remaining.as_millis() as f64 / self.timeout.as_millis() as f64)
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
        let complete_perc = self.remaining.as_millis() as f64 / self.timeout.as_millis() as f64;
        let complete_perc = 1.0 - complete_perc;
        let top = f64::from(area.height).mul_add(2.0, -4.0);

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
        let marker = self.shapes_selected.get_marker();
        Canvas::default()
            .block(Block::bordered())
            .marker(marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(move |ctx| {
                ctx.draw(&shape);
            })
    }

    fn timeout_complete(&self) {
        let nb_sound_files = Asset::iter().count();
        let rand_select = rand::thread_rng().gen_range(0..nb_sound_files);
        let sounds: Vec<_> = Asset::iter().collect();
        let sound_select = sounds.get(rand_select).unwrap().clone();

        std::thread::spawn(move || {
            let (_s, sh) = rodio::OutputStream::try_default().unwrap();
            let file_data = Asset::get(sound_select.as_ref()).unwrap();

            let cursor = std::io::Cursor::new(file_data.data);
            let reader = std::io::BufReader::new(cursor);
            let source = rodio::Decoder::new(reader).unwrap();
            let sink = rodio::Sink::try_new(&sh).unwrap();

            sink.append(source);
            sink.sleep_until_end();
        });
    }
}

fn get_center_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
