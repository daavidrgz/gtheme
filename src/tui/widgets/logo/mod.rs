use log::error;
use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::core::theme::Theme;
use crate::utils;

const LOGO_CONTENT: &str = include_str!("logo.txt");
pub struct LogoWidget<'a> {
    widget: Paragraph<'a>,
}
impl<'a> LogoWidget<'a> {
    fn get_colors(theme_opt: Option<Theme>) -> Vec<Color> {
        let default_colors = vec![
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
        ];

        match theme_opt {
            None => default_colors,
            Some(t) => {
                let color_keys = ["red", "green", "yellow", "blue", "magenta", "cyan"];
                let colors_map = t.get_colors();
                let mut colors: Vec<Color> = vec![];
                for key in color_keys {
                    if colors_map.get(key) == None {
                        error!("The property |{}| does not exist!", key);
                        return default_colors;
                    }

                    let hex_color = format!("#{}", colors_map.get(key).unwrap());
                    match utils::hex_to_rgb(&hex_color) {
                        Some((r, g, b)) => colors.push(Color::Rgb(r, g, b)),
                        None => {
                            error!(
                                "Invalid hexadcimal color '|{}|' in property |{}|",
                                hex_color, key
                            );
                            return default_colors;
                        }
                    }
                }
                colors
            }
        }
    }

    fn create_logo(colors: Vec<Color>) -> Vec<Spans<'a>> {
        let mut spans: Vec<Spans> = vec![];
        for l in LOGO_CONTENT.lines() {
            let words: Vec<&str> = l.split('$').collect();

            let mut line_spans: Vec<Span> = vec![];
            for (idx, word) in words.into_iter().enumerate() {
                line_spans.push(Span::styled(
                    String::from(word),
                    Style::default().fg(colors[idx]),
                ));
            }
            spans.push(Spans::from(line_spans));
        }
        spans
    }

    pub fn new(theme: Option<Theme>) -> LogoWidget<'a> {
        let colors = Self::get_colors(theme);

        LogoWidget {
            widget: Paragraph::new(Self::create_logo(colors)).alignment(Alignment::Left),
        }
    }

    pub fn get_widget(self) -> Paragraph<'a> {
        self.widget
    }
}
