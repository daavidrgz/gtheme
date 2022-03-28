mod logo_content;

use tui::{
	widgets::Paragraph,
	layout::Alignment,
	style::{Color, Style},
	text::{Span, Spans},
};
use tint::Color as TintColor;

use logo_content::LOGO_CONTENT;
use crate::core::theme::Theme;

pub struct LogoWidget<'a> {
	widget: Paragraph<'a>,
}
impl<'a> LogoWidget<'a> {
	pub fn new(theme: Option<Theme>) -> LogoWidget<'a> {
		let colors = match theme {
			None => vec![Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan],
			Some(t) => {
				let color_keys = ["red", "green", "yellow", "blue", "magenta", "cyan"];
				let colors_map = t.get_colors();
				let mut colors: Vec<Color> = vec![];
				for key in color_keys {
					let hex_color = format!("#{}", colors_map.get(key).unwrap());
					let (r,g,b) = TintColor::from_hex(&hex_color).to_rgb255();
					colors.push(Color::Rgb(r,g,b));
				}
				colors
			}
		};

		LogoWidget {
			widget: Paragraph::new(Self::create_logo(colors))
				.alignment(Alignment::Left)
		}
	}

	pub fn get_widget(self) -> Paragraph<'a> {
		self.widget
	}

	fn create_logo(colors: Vec<Color>) -> Vec<Spans<'a>> {
		let mut spans: Vec<Spans> = vec![];
		for l in LOGO_CONTENT.lines() {
			let words: Vec<&str> = l.split('$').collect();

			let mut line_spans: Vec<Span> = vec![];
			for (idx, word) in words.into_iter().enumerate() {
				line_spans.push(Span::styled(String::from(word), Style::default().fg(colors[idx])));
			}
			spans.push(Spans::from(line_spans));
		}
		spans
	}
}
