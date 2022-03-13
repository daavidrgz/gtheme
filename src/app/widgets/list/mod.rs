use tui::{
	widgets::{Block, Borders, BorderType, List, ListItem},
	style::{Modifier, Style},
	text::{Span, Spans},
};

use crate::core::config::{GlobalConfig, DesktopConfig,};
use crate::app::{screenitem::ScreenItem, statefullist::StatefulList};

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

		let items: Vec<ListItem> = stateful_list
			.get_items().iter().enumerate()
			.map(|(it, screen_item)| {

				let state_color = match (stateful_list.get_active_text_color(), stateful_list.get_inactive_text_color()) {
					(Some(active), Some(inactive)) => if screen_item.is_active(global_config, desktop_config) { *active } else { *inactive },
					_ => color 
				};

				let (name_style, active_style) = match stateful_list.get_state().selected() {
					Some(idx) => if idx == it {
						(highlight_name_stlye, highlight_active_style.fg(state_color))
					} else {
						(default_name_style, default_active_style.fg(state_color))
					},
					None => (default_name_style, default_active_style.fg(state_color))
				};

				let (name, active_text, arrows) = Self::get_item_text(it, screen_item, stateful_list, global_config, desktop_config);

				ListItem::new(Spans::from(vec![
					Span::styled(name, name_style),
					Span::styled(active_text, active_style), 
					Span::styled(arrows, name_style),
				]))
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
				.border_style(border_style)
				.border_type(BorderType::Thick))
			.highlight_symbol("");
			
		ListWidget { widget }
	}	

	fn get_item_text(it: usize, screen_item: &ScreenItem, stateful_list: &StatefulList<ScreenItem>, global_config: &GlobalConfig, desktop_config: &Option<DesktopConfig>) -> (String, String, String) {
		let mut name = screen_item.get_name().to_string();
		let mut arrows = String::new();

		name = match stateful_list.get_state().selected() {
			Some(idx) => {
				if idx == it {
					arrows = if idx == 0 { "↓".to_string() }
					else if idx + 1 == stateful_list.get_length() { "↑".to_string() }
					else { "↓ ↑".to_string() };

					if *stateful_list.get_alignment() { format!(" ‣ {:<20} ", name) } else { format!(" ‣ {} ", name) }
				} else {
					if *stateful_list.get_alignment() { format!("   {:<20} ", name) } else { format!("   {} ", name) }
				}
			},
			None => if *stateful_list.get_alignment() { format!("   {:<20} ", name) } else { format!("   {} ", name) }
		};
	
		let mut active_text = if screen_item.is_active(global_config, desktop_config) {
			stateful_list.get_active_text().clone()
		} else {
			stateful_list.get_inactive_text().clone()
		};

		if screen_item.is_inverted(desktop_config) { 
			active_text = format!("{} (Inverted) ", active_text.trim());
		}

		(name, active_text, arrows)
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
 
