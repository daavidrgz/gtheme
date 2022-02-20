use tui_logger::*;
use tui::{
	widgets::{Block, Borders, List, ListItem},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
};

pub struct LoggerWidget<'a> {
	widget: TuiLoggerWidget<'a>
}

impl<'a> LoggerWidget<'a> {
	pub fn new() -> Self {
		let tui_lg = TuiLoggerWidget::default()
			.style_error(Style::default().fg(Color::Red))
			.style_debug(Style::default().fg(Color::Green))
			.style_warn(Style::default().fg(Color::Yellow))
			.style_trace(Style::default().fg(Color::Gray))
			.style_info(Style::default().fg(Color::Blue))
			.output_separator('|')
			.output_timestamp(Some("%F %H:%M:%S".to_string()))
			.block(Block::default()
				.title(" LOGS  ")
				.border_style(Style::default().fg(Color::White))
				.borders(Borders::ALL))
			.style(Style::default().fg(Color::White).bg(Color::Black));

		LoggerWidget {
			widget: tui_lg
		}
	}

	pub fn get_widget(self) -> TuiLoggerWidget<'a> {
		self.widget
	}
}
