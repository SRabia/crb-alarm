use crate::spoty;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

#[derive(Debug)]
pub struct MusicPlayer {
    spoty_api: spoty::SpotiApi,
    list_action: ActionList,
}

#[derive(Debug)]
struct ActionList {
    items: Vec<ActionItem>,
    state: ListState,
}

#[derive(Debug)]
struct ActionItem {
    info: String,
}

impl ActionItem {
    fn new(info: &str) -> Self {
        Self {
            info: info.to_string(),
        }
    }
}

impl MusicPlayer {
    pub fn new() -> Self {
        let spoty_api = spoty::SpotiApi::default();
        Self {
            spoty_api,
            list_action: ActionList::from_iter([("Connect to spotify")]),
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

        let [list_area, _item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        MusicPlayer::render_header(header_area, buf);
        MusicPlayer::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
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

    // fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
    //     // We get the info depending on the item's state.
    //     let info = if let Some(i) = self.todo_list.state.selected() {
    //         match self.todo_list.items[i].cmd {
    //             Command::ConnectSpotify => format!("✓ DONE: {}", self.todo_list.items[i].info),
    //             Command::Quit => format!("☐ TODO: {}", self.todo_list.items[i].info),
    //         }
    //     } else {
    //         "Nothing selected...".to_string()
    //     };

    //     // We show the list item's info under the list in this paragraph
    //     let block = Block::new()
    //         .title(Line::raw("TODO Info").centered())
    //         .borders(Borders::TOP)
    //         .border_set(symbols::border::EMPTY)
    //         .border_style(TODO_HEADER_STYLE)
    //         .bg(NORMAL_ROW_BG)
    //         .padding(Padding::horizontal(1));

    //     // We can now render the item info
    //     Paragraph::new(info)
    //         .block(block)
    //         .fg(TEXT_FG_COLOR)
    //         .wrap(Wrap { trim: false })
    //         .render(area, buf);
    // }
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
        let line = Line::styled(format!(" ☐ {}", value.info), TEXT_FG_COLOR);
        // let line = match value.cmd {
        //     Command::ConnectSpotify => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
        //     // Command::Quit => Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR),
        // };
        ListItem::new(line)
    }
}
