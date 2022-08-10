use tui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

pub struct OptionsWidget<'a> {
    widget: Block<'a>,
}
impl<'a> OptionsWidget<'a> {
    pub fn default() -> Self {
        let title_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let border_style = Style::default().fg(Color::Yellow);

        let widget = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(Span::styled(String::from(" OPTIONS 襁"), title_style))
            .border_style(border_style);

        OptionsWidget { widget }
    }

    pub fn get_widget(self) -> Block<'a> {
        self.widget
    }
}
