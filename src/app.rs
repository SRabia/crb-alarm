use crate::anime;
use crate::fps;
use crate::shapes;
use crate::theme;
use color_eyre::Result;
use rand::Rng;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{self, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, ToSpan},
    widgets::{block::Title, Block, Widget},
    DefaultTerminal, Frame,
};

use crate::music_player;
use rust_embed::RustEmbed;
use std::time::{Duration, Instant};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

#[derive(Debug, PartialEq)]
enum AppState {
    Main,
    CmdSelect,
    Quit,
}

pub struct App {
    fps: fps::Fps,
    tm_animation: anime::AnimChrono,
    player: music_player::MusicPlayer,
    state: AppState,
}

const DARK_BLUE: Color = Color::Rgb(16, 24, 48);

impl App {
    //TODO: timeout should be an option, don't play animation of None
    pub fn new(timeout: Duration) -> Self {
        //TODO: move this to main should come from user config
        let rand_select = rand::thread_rng().gen_range(0..3);
        let s = shapes::ShapeSelect::select_from(rand_select, Color::LightRed);

        Self {
            fps: fps::Fps::default(),
            tm_animation: anime::AnimChrono::new(s, timeout),
            player: music_player::MusicPlayer::default(),
            state: AppState::Main,
        }
    }

    fn handle_event_main(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.state = AppState::Quit;
            }
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
            KeyCode::Char('p') => {
                if key.modifiers == KeyModifiers::CONTROL {
                    self.state = AppState::CmdSelect;
                }
            }
            _ => {}
        }
    }

    fn handle_event_player(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.state = AppState::Main,
            KeyCode::Char('h') | KeyCode::Left => self.player.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.player.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.player.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.player.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.player.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.player.do_action();
                // self.player.enter_command();
            }
            _ => {}
        }
    }

    fn handle_event(&mut self, key: KeyEvent) {
        match self.state {
            AppState::Main => {
                self.handle_event_main(key);
            }
            AppState::CmdSelect => {
                self.handle_event_player(key);
            }
            _ => {}
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();
        while self.state != AppState::Quit {
            self.fps.update();
            terminal.draw(|frame| self.draw(frame))?;
            // timeout = 16 - loop_interval
            let interval_tick = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(interval_tick)? {
                // block for 16ms
                if let Event::Key(key) = event::read()? {
                    self.handle_event(key);
                }
            }

            let elapsed = last_tick.elapsed();
            if elapsed >= tick_rate {
                last_tick = Instant::now();
                self.tm_animation.update(elapsed, timeout_complete);
            }
        }
        Ok(())
    }

    fn get_tm_info_widget(&self) -> impl Widget + '_ {
        let timeout_rem = get_time_left_formated(&self.tm_animation.remaining);
        let timeout_total = get_time_left_formated(&self.tm_animation.timeout);

        let complete_perc = self.tm_animation.remaining.as_millis() as f64
            / self.tm_animation.timeout.as_millis() as f64;
        let complete_perc = format!("{:.3}%", (1.0 - complete_perc) * 100.0);
        let keys = [
            ("Time Left", timeout_rem.as_str()),
            ("Total Duration", timeout_total.as_str()),
        ];
        let key_style = Style::new().fg(theme::BLACK).bg(theme::DARK_GRAY);
        let val_style = Style::new().fg(theme::DARK_GRAY).bg(theme::BLACK);
        let mut spans: Vec<Span> = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {key} "), key_style);
                let desc = Span::styled(format!(" {desc} "), val_style);
                [key, desc]
            })
            .collect();
        spans.push(Span::styled(
            complete_perc,
            Style::new().fg(theme::LIGHT_YELLOW).bg(theme::DARK_GRAY),
        ));
        Line::from(spans)
            .centered()
            .style((Color::Indexed(236), Color::Indexed(232)))
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let horizontal = Layout::horizontal([Constraint::Percentage(30), Constraint::Min(0)]);
        let [title_bar, main_area, bottom_bar] = vertical.areas(area);

        let main_frame = Block::new()
            .style(Style::new().bg(DARK_BLUE))
            .title(Title::from(format!("{}x{}", area.width, area.height)));
        frame.render_widget(main_frame, area);

        let message_fps = format!("{:.2} FPS", self.fps.fps());
        let title_fps = Title::from(message_fps.to_span().dim())
            .alignment(layout::Alignment::Left)
            .position(ratatui::widgets::block::Position::Top);
        let block_info = Block::bordered().title(title_fps).border_set(border::THICK);
        match self.state {
            AppState::Main => {
                frame.render_widget(&self.tm_animation, main_area);
                frame.render_widget(block_info, main_area);
            }
            AppState::CmdSelect => {
                let [list_area, animation_area] = horizontal.areas(main_area);
                frame.render_widget(&mut self.player, list_area);
                frame.render_widget(&self.tm_animation, animation_area);
                frame.render_widget(block_info, animation_area);
            }
            _ => {}
        }
        frame.render_widget(render_bottom_bar(), bottom_bar);
        frame.render_widget(self.get_tm_info_widget(), title_bar);
    }
}

fn render_bottom_bar() -> impl Widget + 'static {
    let keys = [
        ("h/←", "Sub 1s"),
        ("l/→", "Add 1s"),
        ("k/↑", "Add 1m"),
        ("j/↓", "Sub 1m"),
        ("r", "Reset time"),
        ("q", "Quit"),
    ];
    let key_style = Style::new().fg(theme::BLACK).bg(theme::DARK_GRAY);
    let desc_style = Style::new().fg(theme::DARK_GRAY).bg(theme::BLACK);
    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(format!(" {key} "), key_style);
            let desc = Span::styled(format!(" {desc} "), desc_style);
            [key, desc]
        })
        .collect();
    Line::from(spans)
        .centered()
        .style((Color::Indexed(236), Color::Indexed(232)))
}

// fn get_center_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
//     let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
//     let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
//     let [area] = vertical.areas(area);
//     let [area] = horizontal.areas(area);
//     area
// }

fn timeout_complete() {
    let nb_sound_files = Asset::iter().count();
    let rand_select = rand::thread_rng().gen_range(0..nb_sound_files);
    let sounds: Vec<_> = Asset::iter().collect();
    let sound_select = sounds.get(rand_select).unwrap().clone();
    // tokio::spawn(async move {
    //     let (_s, sh) = rodio::OutputStream::try_default().unwrap();
    //     let file_data = Asset::get(sound_select.as_ref()).unwrap();

    //     let cursor = std::io::Cursor::new(file_data.data);
    //     let reader = std::io::BufReader::new(cursor);
    //     let source = rodio::Decoder::new(reader).unwrap();
    //     let sink = rodio::Sink::try_new(&sh).unwrap();

    //     sink.append(source);
    //     sink.sleep_until_end();
    // });

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

pub fn get_time_left_formated(d: &Duration) -> String {
    let m = d.as_secs() / 60;
    let s = d.as_secs() % 60;
    let h = d.as_secs() / 3600;
    if h > 0 {
        format!("{}h {}m {}s", h, m, s)
    } else if m > 0 {
        format!("{}m {}s", m, s)
    } else {
        format!("{}s", s)
    }
}
