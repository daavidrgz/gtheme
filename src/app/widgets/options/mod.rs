use tui::{
	widgets::{Block, Borders, List, ListItem, ListState},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
};
use crate::core::config::GlobalConfig;

pub struct OptionsWidget<'a> {
	widget: Block<'a>
}
impl<'a> OptionsWidget<'a> {
	pub fn new() -> Self {
		let title_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
		let border_style = Style::default().fg(Color::Yellow);

		let widget =  Block::default()
			.borders(Borders::ALL)
			.title(Span::styled(String::from(" OPTIONS 襁"), title_style))
			.border_style(border_style);

		OptionsWidget { widget }
	}

	pub fn get_widget(self) -> Block<'a> {
		self.widget
	}
}
