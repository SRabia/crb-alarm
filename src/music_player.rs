use crate::spoty;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
// const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

type ActionReturnType = ApiState;

#[derive(Default, PartialEq, Eq)]
enum ApiState {
    #[default]
    Connecting,
    Connected(String),
    Auth(String),
    Error(String),
}

// struct WidgetResult {
//     render_fn: Box<dyn FnMut(Rect, &mut Buffer)>,
// }

// impl WidgetResult {
//     fn new<W: Widget + 'static>(widget: W) -> Self {
//         Self {
//             render_fn: Box::new(move |area, buf| widget.render(area, buf)),
//         }
//     }
// }

// impl Widget for WidgetResult {
//     fn render(mut self, area: Rect, buf: &mut Buffer) {
//         (self.render_fn)(area, buf);
//     }
// }

// #[derive(Debug)]
pub struct MusicPlayer {
    spoty_api: spoty::SpotiApi,
    list_action: ActionList,
}

// #[derive(Debug)]
struct ActionList {
    items: Vec<ActionItem>,
    state: ListState,
}

struct ActionItem {
    action_name: String,
    result: ActionReturnType,
}

impl ActionItem {
    fn new(action_name: &str) -> Self {
        Self {
            action_name: action_name.to_string(),
            result: ApiState::default(),
        }
    }
}

impl MusicPlayer {
    async fn connect(&mut self) -> ActionReturnType {
        let info = self.spoty_api.try_auth().await;
        match info {
            Err(e) => ApiState::Error(e.to_string()),
            Ok(()) => ApiState::Connected("connection successful".to_string()),
        }
    }

    pub fn new() -> Self {
        let spoty_api = spoty::SpotiApi::default();
        let list_action_tuple = ["Connect to spotify"];
        Self {
            spoty_api,
            list_action: ActionList::from_iter(list_action_tuple),
        }
    }

    pub fn select_none(&mut self) {
        self.list_action.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.list_action.state.select_next();
    }
    pub fn select_previous(&mut self) {
        self.list_action.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.list_action.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.list_action.state.select_last();
    }

    pub async fn do_action(&mut self) {
        if let Some(i) = self.list_action.state.selected() {
            let r = &self.list_action.items[i].result;
            match r {
                ApiState::Connecting => self.list_action.items[i].result = self.connect().await,
                _ => {}
            };
        }
    }
}

impl FromIterator<&'static str> for ActionList {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        let items = iter.into_iter().map(|info| ActionItem::new(info)).collect();
        let state = ListState::default();
        Self { items, state }
    }
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
}
impl Widget for &mut MusicPlayer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        MusicPlayer::render_header(header_area, buf);
        MusicPlayer::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl MusicPlayer {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Actions")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Action List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .list_action
            .items
            .iter()
            .enumerate()
            .map(|(i, act_item)| {
                let color = alternate_colors(i);
                ListItem::from(act_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.list_action.state);
    }

    fn render_selected_item(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));
        if let Some(i) = self.list_action.state.selected() {
            let r = &self.list_action.items[i].result;
            match r {
                ApiState::Connected(info) => {
                    Paragraph::new(info.as_str())
                        .block(block)
                        .fg(TEXT_FG_COLOR)
                        .wrap(Wrap { trim: false })
                        .render(area, buf);
                }
                ApiState::Error(e) => {
                    Paragraph::new(e.as_str())
                        .block(block)
                        .fg(TEXT_FG_COLOR)
                        .wrap(Wrap { trim: false })
                        .render(area, buf);
                }
                _ => {}
            }
        }
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&ActionItem> for ListItem<'_> {
    fn from(value: &ActionItem) -> Self {
        let line = Line::styled(format!(" ☐ {}", value.action_name), TEXT_FG_COLOR);
        ListItem::new(line)
    }
}

// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use ratatui::{
//     buffer::Buffer,
//     layout::{Constraint, Layout, Offset, Rect},
//     style::Stylize,
//     text::Line,
//     widgets::Widget,
//     DefaultTerminal, Frame,
// };
// use serde::Serialize;

// fn main() -> Result<()> {
//     color_eyre::install()?;
//     let terminal = ratatui::init();
//     let result = App::default().run(terminal);
//     ratatui::restore();

//     // serialize the form to JSON if the user submitted it, otherwise print "Canceled"
//     match result {
//         Ok(Some(form)) => println!("{}", serde_json::to_string_pretty(&form)?),
//         Ok(None) => println!("Canceled"),
//         Err(err) => eprintln!("{err}"),
//     }
//     Ok(())
// }

// #[derive(Default)]
// struct App {
//     state: AppState,
//     form: InputForm,
// }

// #[derive(Default, PartialEq, Eq)]
// enum AppState {
//     #[default]
//     Running,
//     Cancelled,
//     Submitted,
// }

// impl App {
//     fn run(mut self, mut terminal: DefaultTerminal) -> Result<Option<InputForm>> {
//         while self.state == AppState::Running {
//             terminal.draw(|frame| self.render(frame))?;
//             self.handle_events()?;
//         }
//         match self.state {
//             AppState::Cancelled => Ok(None),
//             AppState::Submitted => Ok(Some(self.form)),
//             AppState::Running => unreachable!(),
//         }
//     }

//     fn render(&self, frame: &mut Frame) {
//         self.form.render(frame);
//     }

//     fn handle_events(&mut self) -> Result<()> {
//         match event::read()? {
//             Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
//                 KeyCode::Esc => self.state = AppState::Cancelled,
//                 KeyCode::Enter => self.state = AppState::Submitted,
//                 _ => self.form.on_key_press(event),
//             },
//             _ => {}
//         }
//         Ok(())
//     }
// }

// struct InputForm {
//     focus: Focus,
//     first_name: StringField,
// }

// impl Default for InputForm {
//     fn default() -> Self {
//         Self {
//             focus: Focus::FirstName,
//             first_name: StringField::new("First Name"),
//             last_name: StringField::new("Last Name"),
//             age: AgeField::new("Age"),
//         }
//     }
// }

// impl InputForm {
//     // Handle focus navigation or pass the event to the focused field.
//     fn on_key_press(&mut self, event: KeyEvent) {
//         match event.code {
//             KeyCode::Tab => self.focus = self.focus.next(),
//             _ => match self.focus {
//                 Focus::FirstName => self.first_name.on_key_press(event),
//                 Focus::LastName => self.last_name.on_key_press(event),
//                 Focus::Age => self.age.on_key_press(event),
//             },
//         }
//     }

//     /// Render the form with the current focus.
//     ///
//     /// The cursor is placed at the end of the focused field.
//     fn render(&self, frame: &mut Frame) {
//         let [first_name_area, last_name_area, age_area] =
//             Layout::vertical(Constraint::from_lengths([1, 1, 1])).areas(frame.area());

//         frame.render_widget(&self.first_name, first_name_area);
//         frame.render_widget(&self.last_name, last_name_area);
//         frame.render_widget(&self.age, age_area);

//         let cursor_position = match self.focus {
//             Focus::FirstName => first_name_area.offset(self.first_name.cursor_offset()),
//             Focus::LastName => last_name_area.offset(self.last_name.cursor_offset()),
//             Focus::Age => age_area.offset(self.age.cursor_offset()),
//         };
//         frame.set_cursor_position(cursor_position);
//     }
// }

// #[derive(Default, PartialEq, Eq)]
// enum Focus {
//     #[default]
//     FirstName,
//     LastName,
//     Age,
// }

// impl Focus {
//     // Round-robin focus order.
//     const fn next(&self) -> Self {
//         match self {
//             Self::FirstName => Self::LastName,
//             Self::LastName => Self::Age,
//         }
//     }
// }

// /// A new-type representing a string field with a label.
// #[derive(Debug)]
// struct StringField {
//     label: &'static str,
//     value: String,
// }

// impl StringField {
//     const fn new(label: &'static str) -> Self {
//         Self {
//             label,
//             value: String::new(),
//         }
//     }

//     /// Handle input events for the string input.
//     fn on_key_press(&mut self, event: KeyEvent) {
//         match event.code {
//             KeyCode::Char(c) => self.value.push(c),
//             KeyCode::Backspace => {
//                 self.value.pop();
//             }
//             _ => {}
//         }
//     }

//     fn cursor_offset(&self) -> Offset {
//         let x = (self.label.len() + self.value.len() + 2) as i32;
//         Offset::new(x, 0)
//     }
// }

// impl Widget for &StringField {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         let constraints = [
//             Constraint::Length(self.label.len() as u16 + 2),
//             Constraint::Fill(1),
//         ];
//         let [label_area, value_area] = Layout::horizontal(constraints).areas(area);
//         let label = Line::from_iter([self.label, ": "]).bold();
//         label.render(label_area, buf);
//         self.value.clone().render(value_area, buf);
//     }
// }
