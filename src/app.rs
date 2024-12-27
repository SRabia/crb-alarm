use crate::anime;
use crate::fps;
use crate::shapes;
use color_eyre::Result;
use rand::Rng;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{self, Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Text, ToSpan},
    widgets::{block::Title, Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::spoty;
use rust_embed::RustEmbed;
use std::time::{Duration, Instant};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub struct App {
    fps: fps::Fps,
    spoty_api: spoty::SpotiApi,
    tm_animation: anime::AnimChrono,
}

impl App {
    //TODO: timeout should be an option, don't play animation of None
    //TODO: spoty should be music ui widget and create at init
    pub fn new(timeout: Duration, spoty_api: spoty::SpotiApi) -> Self {
        //TODO: move this to main should come from user config
        let rand_select = rand::thread_rng().gen_range(0..3);
        let s = shapes::ShapeSelect::select_from(rand_select, Color::LightRed);

        Self {
            fps: fps::Fps::default(),
            tm_animation: anime::AnimChrono::new(s, timeout),
            spoty_api,
        }
    }

    // /// Run the app until the user quits.
    // pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
    //     while self.is_running() {
    //         terminal
    //             .draw(|frame| self.draw(frame))
    //             .wrap_err("terminal.draw")?;
    //         self.handle_events()?;
    //     }
    //     Ok(())
    // }

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
                            self.tm_animation.decrease_timeout(60);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.tm_animation.increase_timeout(60);
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            self.tm_animation.increase_timeout(1);
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            self.tm_animation.decrease_timeout(1);
                        }
                        _ => {}
                    }
                }
            }

            let elapsed = last_tick.elapsed();
            if elapsed >= tick_rate {
                last_tick = Instant::now();
                self.tm_animation.update(elapsed, timeout_complete);
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

        let (h, m, s) = self.tm_animation.get_time_left_formated();

        let timeout = if h > 0 {
            format!("{}:{}:{}", h, m, s)
        } else {
            format!("{}:{}", m, s)
        };

        // let timeout = format!("sptoy {:?}", self.spoty_api.get_user_info());
        // TODO: temp high jack this to debug spoty
        let timeout_text = Text::from(vec![
            Line::from(" Time Left: ").centered(),
            Line::from(timeout.yellow()),
        ])
        .centered();

        Paragraph::new(timeout_text).centered().block(block)
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(&self.tm_animation, area);

        // frame.render_widget(self.get_tm_animation_widget(area), area);
        let area_chrono = get_center_area(area, 20, 10);
        frame.render_widget(self.get_tm_info_widget(), area_chrono);
        let message_fps = format!("{:.2} FPS", self.fps.fps());
        let title_fps = Title::from(message_fps.to_span().dim())
            .alignment(layout::Alignment::Left)
            .position(ratatui::widgets::block::Position::Top);

        let block_info = Block::bordered()
            .title(
                Title::from(format!(
                    "{}x{} -> {}",
                    area.width,
                    area.height,
                    f64::from(area.height).mul_add(2.0, -4.0)
                ))
                .alignment(ratatui::layout::Alignment::Right)
                .position(ratatui::widgets::block::Position::Bottom),
            )
            .title(title_fps)
            .border_set(border::THICK);
        frame.render_widget(block_info, area);
    }
}

fn get_center_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn timeout_complete() {
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
