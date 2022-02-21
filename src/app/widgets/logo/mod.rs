use std::fs::File;
use std::io::{self, BufRead};

use log::error;

use tui::{
	widgets::Paragraph,
	layout::Alignment,
	style::{Color, Style},
	text::{Span, Spans},
};

use crate::core;
use crate::core::theme::Theme;

pub struct LogoWidget<'a> {
	widget: Paragraph<'a>,
}
impl<'a> LogoWidget<'a> {
	pub fn new(theme: Option<Theme>) -> LogoWidget<'a> {
		let path = core::expand_path(&format!("{}/assets/logo.txt", core::GTHEME_HOME));

		let colors = match theme {
			None => vec![Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan],
			Some(t) => {
				let color_keys = ["red", "green", "yellow", "blue", "magenta", "cyan"];
				let colors_map = t.get_colors();
				let mut colors: Vec<Color> = vec![];
				for key in color_keys {
					let hex_color = format!("#{}", colors_map.get(key).unwrap());
					let (r,g,b) = tint::Color::from_hex(&hex_color).to_rgb255();
					colors.push(Color::Rgb(r,g,b));
				}
				colors
			}
		};

		LogoWidget {
			widget: Paragraph::new(Self::create_logo(path,colors))
				.alignment(Alignment::Left)
		}
	}

	pub fn get_widget(self) -> Paragraph<'a> {
		self.widget
	}

	fn create_logo(logo_path: String, colors: Vec<Color>) -> Vec<Spans<'a>> {
		let logo_file = match File::open(&logo_path) {
			Ok(f) => f,
			Err(e) => {
				error!("Error while opening logo file in {}: {}", &logo_path, e);
				return vec![];
			}
		};
		let file_lines = io::BufReader::new(logo_file).lines();

		let mut spans: Vec<Spans> = vec![];
		for l in file_lines {
			let line = match l {
				Ok(s) => s,
				Err(e) => {
					error!("Error while reading logo file in {}: {}", &logo_path, e);
					return vec![]
				}
			};
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
