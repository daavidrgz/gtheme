use tui::{
	widgets::{Block, Borders, BorderType, List, ListItem},
	style::{Modifier, Style, Color},
	text::{Span, Spans},
};

use crate::core::config::{GlobalConfig, DesktopConfig,};
use crate::tui::{screenitem::ScreenItem, statefullist::StatefulList};

pub struct ListWidget<'a> {
	widget: List<'a>
}
impl<'a> ListWidget<'a> {
	pub fn new(stateful_list: &StatefulList<ScreenItem>, global_config: &GlobalConfig, desktop_config: &Option<DesktopConfig>) -> ListWidget<'a> {

		let color = *stateful_list.get_color();
		let title = stateful_list.get_title();

		let default_name_style = Style::default().add_modifier(Modifier::DIM);
		let highlight_name_stlye = Style::default().fg(color).add_modifier(Modifier::BOLD);

		let default_active_style = Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::DIM);
		let highlight_active_style = Style::default().add_modifier(Modifier::BOLD);

		let default_theme_style = Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::DIM).fg(Color::Yellow);
		let highlight_default_theme_style = Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow);
		
		let mut items: Vec<ListItem> = vec![];
		for (it, screen_item) in stateful_list.get_items().iter().enumerate() {
			let state_color = match (stateful_list.get_active_text_color(), stateful_list.get_inactive_text_color()) {
				(Some(active), Some(inactive)) => 
					if screen_item.is_active(global_config, desktop_config) { *active } else { *inactive },
				_ => color
			};

			let (name_style, active_style, default_theme_style) = match stateful_list.get_state().selected() {
				Some(idx) => if idx == it {
					(highlight_name_stlye, highlight_active_style.fg(state_color), highlight_default_theme_style)
				} else {
					(default_name_style, default_active_style.fg(state_color), default_theme_style)
				},
				None => (default_name_style, default_active_style.fg(state_color), default_theme_style)
			};

			let (name, default_theme, active_text, arrows) = 
				Self::get_item_text(it, screen_item, stateful_list, global_config, desktop_config);

			let list_item = ListItem::new(Spans::from(vec![
				Span::styled(name, name_style),
				Span::styled(default_theme, default_theme_style),
				Span::styled(active_text, active_style), 
				Span::styled(arrows, name_style),
			]));
			items.push(list_item)
		}
		
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
				.border_style(border_style)
				.border_type(BorderType::Thick))
			.highlight_symbol("");
			
		ListWidget { widget }
	}	

	fn get_item_text(it: usize, screen_item: &ScreenItem, stateful_list: &StatefulList<ScreenItem>, global_config: &GlobalConfig, desktop_config: &Option<DesktopConfig>) -> (String, String, String, String) {
		let name_str = screen_item.get_name();

		let mut arrows = String::new();
		let mut name = if *stateful_list.get_alignment() { format!("   {:<20} ", name_str) } else { format!("   {} ", name_str) };
		match stateful_list.get_state().selected() {
			Some(idx) => {
				if idx == it {
					arrows = if idx == 0 { "↓".to_string() }
					else if idx + 1 == stateful_list.get_length() { "↑".to_string() }
					else { "↓ ↑".to_string() };

					name = if *stateful_list.get_alignment() { format!(" ‣ {:<20} ", name_str) } else { format!(" ‣ {} ", name_str) }
				}
			},
			None => ()
		};

		let mut default_theme = String::new();
		if screen_item.is_default_theme(desktop_config) {
			default_theme = "• Default ".to_string();
		}
	
		let mut active_text = if screen_item.is_active(global_config, desktop_config) {
			stateful_list.get_active_text().clone()
		} else {
			stateful_list.get_inactive_text().clone()
		};

		if screen_item.is_inverted(desktop_config) { 
			active_text = format!("{} (Inverted) ", active_text.trim());
		}

		(name, default_theme, active_text, arrows)
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
 
