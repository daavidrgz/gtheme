use std::fs::File;
use std::io::{self, BufRead};

use tui::{
	widgets::Paragraph,
	layout::Alignment,
	style::{Color, Style},
	text::{Span, Spans},
};

use crate::core;

pub struct LogoWidget<'a> {
	widget: Paragraph<'a>,
}
impl<'a> LogoWidget<'a> {
	pub fn new() -> LogoWidget<'a> {
		let path = core::expand_path(&format!("{}/assets/logo.txt", core::GTHEME_HOME));
		LogoWidget {
			widget: Paragraph::new(LogoWidget::create_logo(path))
				.alignment(Alignment::Left)
		}
	}

	pub fn get_widget(self) -> Paragraph<'a> {
		self.widget
	}

	fn create_logo(logo_path: String) -> Vec<Spans<'a>> {
		let logo_file = File::open(&logo_path).expect(&format!("Error while opening logo file in {}", &logo_path));
		let file_lines = io::BufReader::new(logo_file).lines();

		let colors = vec![Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan];

		let mut spans: Vec<Spans> = vec![];

		for l in file_lines {
			let line = l.expect("Error while reading logo file");
			let words: Vec<&str> = line.split('$').collect();

			let mut line_spans: Vec<Span> = vec![];
			for (idx,word) in words.into_iter().enumerate() {
				line_spans.push(Span::styled(String::from(word), Style::default().fg(colors[idx])));
			}
			spans.push(Spans::from(line_spans));
		}
		spans
	}
}
