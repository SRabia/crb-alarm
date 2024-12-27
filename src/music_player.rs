use crate::spoty;
use ratatui::{
    layout::Rect,
    prelude::Buffer,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
};
#[derive(Debug)]
pub struct MusicPlayer {
    spoty_api: spoty::SpotiApi,
}

impl MusicPlayer {
    pub fn new() -> Self {
        let spoty_api = spoty::SpotiApi::default();

        Self { spoty_api }
    }
}

impl Default for MusicPlayer {
    fn default() -> Self {
        Self::new()
    }
}
impl Widget for &MusicPlayer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("".bold());
        let instructions = Title::from(Line::from(vec![
            //todo: bad render here use mult line
            "Connect".into(),
            "<C>".blue().bold(),
            "Disconnect".into(),
            "<D>".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(ratatui::layout::Alignment::Center))
            .title(
                instructions
                    .alignment(ratatui::layout::Alignment::Center)
                    .position(ratatui::widgets::block::Position::Bottom),
            )
            .border_set(border::THICK);

        let timeout = self.spoty_api.testing_shit();

        // let timeout = format!("sptoy {:?}", self.spoty_api.get_user_info());
        // TODO: temp high jack this to debug spoty
        let timeout_text = Text::from(vec![
            Line::from(" Time Left: ").centered(),
            Line::from(timeout.yellow()),
        ])
        .centered();

        Paragraph::new(timeout_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
