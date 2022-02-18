use std::fs::File;
use std::io::{self, BufRead};

use tui::{
	widgets::{Block, Borders, Paragraph},
	layout::Alignment,
	style::{Color, Modifier, Style},
	text::{Span, Spans},
};

use crate::core;

pub struct HelpWidget<'a> {
	widget: Paragraph<'a>
}
impl<'a> HelpWidget<'a> {
	pub fn new() -> HelpWidget<'a> {
		let path = core::expand_path(&format!("{}/assets/help.txt", core::GTHEME_HOME));

		let spans = Self::create_help(path);

		let title_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD).add_modifier(Modifier::REVERSED);
		let block = Block::default()
			.title(Span::styled(" HELP  ", title_style))
			.borders(Borders::ALL)
			.border_style(Style::default().fg(Color::Yellow));

		let paragraph = Paragraph::new(spans)
			.block(block)
			.alignment(Alignment::Left);

		HelpWidget {
			widget: paragraph
		}
	}

	fn create_help(help_path: String) -> Vec<Spans<'a>> {
		let help_file = File::open(&help_path).expect(&format!("Error while opening logo file in {}", &help_path));
		let file_lines = io::BufReader::new(help_file).lines();

		let title_style = Style::default().fg(Color::Blue)
			.add_modifier(Modifier::BOLD)
			.add_modifier(Modifier::ITALIC);
		let entry_key_style = Style::default().fg(Color::Yellow)
			.add_modifier(Modifier::BOLD);
		let entry_value_style = Style::default().add_modifier(Modifier::BOLD);

		let mut spans: Vec<Spans> = vec![];
		for l in file_lines {
			let line = l.expect("Error while reading help file");
			let words: Vec<&str> = line.split('#').collect();
			
			if words.len() > 2 {
				panic!("Format error in help file")
			}

			// Its a section title
			if words.len() == 1 {
				spans.push(Spans::from(Span::styled(String::from(words[0]), title_style)));
				continue
			}

			// Its a section entry
			let entry_key = Span::styled(String::from(words[0]), entry_key_style);
			let entry_value = Span::styled(String::from(words[1]), entry_value_style);
			spans.push(Spans::from(vec![entry_key, entry_value]));
		}
		spans
	}

	pub fn get_widget(self) -> Paragraph<'a> {
		self.widget
	}
}
