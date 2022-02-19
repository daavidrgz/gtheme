use tui::{
	widgets::{Block, Borders, List, ListItem},
	style::{Modifier, Style, Color},
	text::{Span, Spans},
};

use crate::core::config::GlobalConfig;
use crate::app::{screenitem::ScreenItem, statefullist::StatefulList};

pub struct ExtrasWidget<'a> {
	widget: List<'a>
}
impl<'a> ExtrasWidget<'a> {
	pub fn new(stateful_list: &StatefulList<ScreenItem>, global_config: &GlobalConfig) -> ExtrasWidget<'a> {

		let color = *stateful_list.get_color();
		let title = stateful_list.get_title();

		let items: Vec<ListItem> = stateful_list
			.get_items().iter().enumerate()
			.map(|(it, screen_item)| {
				let (name, active_text, arrows) = Self::get_item_text(it, screen_item, stateful_list, global_config);
				ListItem::new(Spans::from(vec![
					Span::from(name),
					Span::styled(active_text, Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD).remove_modifier(Modifier::DIM)), 
					Span::from(arrows)
				])).style(Style::default().add_modifier(Modifier::DIM))
			}).collect();
		
		let mut border_style = Style::default().fg(color);
		border_style = if !stateful_list.is_selected() {
			border_style.add_modifier(Modifier::DIM)
		}	else {
			border_style
		};

		let mut title_style = Style::default().fg(color).add_modifier(Modifier::BOLD);
		title_style = if stateful_list.is_selected() {
			title_style.add_modifier(Modifier::REVERSED)
		}	else {
			title_style
		};

		let widget = List::new(items)
			.block(Block::default()
				.borders(Borders::ALL)
				.title(Span::styled(String::from(format!(" {} ", title)), title_style))
				.border_style(border_style))
			.highlight_symbol("")
			.highlight_style(Style::default().fg(color).add_modifier(Modifier::BOLD).remove_modifier(Modifier::DIM),
			);
			
		ExtrasWidget { widget }
	}	

	fn get_item_text(it: usize, screen_item: &ScreenItem, stateful_list: &StatefulList<ScreenItem>, global_config: &GlobalConfig ) -> (String, String, String) {
		let mut name = screen_item.get_name().to_string();
		let mut arrows = String::new();

		name = match stateful_list.get_state().selected() {
			Some(idx) => {
				if idx == it {
					arrows = if idx == 0 { "↓".to_string() }
					else if idx == stateful_list.get_length() - 1 { "↑".to_string() }
					else { "↓ ↑".to_string() };

					format!(" ‣ {} ", name)
				} else {
					format!("   {} ", name)
				}
			},
			None => format!("   {} ", name)
		};
 
		let mut active_text = String::new();
		if screen_item.is_active(global_config) {
			active_text = "• ON ".to_string();
		};

		(name, active_text, arrows)
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
 
