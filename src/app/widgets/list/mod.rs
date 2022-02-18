use tui::{
	widgets::{Block, Borders, List, ListItem, ListState},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
};
use crate::core::config::GlobalConfig;
use crate::app::ScreenItem;

pub struct StatefulList{
	state: ListState,
	index: usize,
	items: Vec<ScreenItem>,
	color: Color,
	title: String
}
impl StatefulList {
	pub fn with_items(items: Vec<ScreenItem>, color: Color, title: String, selected: bool) -> StatefulList {
		let mut state = ListState::default();
		state.select(if selected {Some(0)} else {None});
		StatefulList {
			index: 0,
			color,
			title,
			state,
			items,
		}
	}
	
	pub fn add_fav(&mut self, item:&ScreenItem, global_config: &mut GlobalConfig) {
		match item {
			ScreenItem::Theme(t) => {
				let fav_themes = global_config.get_mut_fav_themes();
				let idx = fav_themes.iter().position(|item| item.get_name() == t.get_name());

				match idx {
					Some(_) => (),
					None => {
						fav_themes.push(t.clone());
						self.items.push(ScreenItem::Theme(t.clone()));
					}
				}
			}
			_ => {}
		}
	}

	pub fn remove_fav(&mut self, item:&ScreenItem, global_config: &mut GlobalConfig) {
		match item {
			ScreenItem::Theme(t) => {
				let fav_themes = global_config.get_mut_fav_themes();
				let idx = fav_themes.iter().position(|item| item.get_name() == t.get_name());
				
				match idx {
					Some(i) => {
						fav_themes.remove(i);
						self.items.remove(i);
					},
					None => ()
				}
			}
			_ => {}
		}
	}

	pub fn get_state(&self) -> &ListState {
		&self.state
	}
	pub fn get_state_mut(&mut self) -> &mut ListState {
		&mut self.state
	}
	pub fn get_items(&self) -> &Vec<ScreenItem> {
		&self.items
	}
	pub fn get_mut_items(&mut self) -> &mut Vec<ScreenItem> {
		&mut self.items
	}
	pub fn get_color(&self) -> &Color {
		&self.color
	}
	pub fn get_title(&self) -> &String {
		&self.title
	}
	pub fn get_length(&self) -> usize {
		self.items.len()
	}
	
	pub fn get_selected(&self) -> Option<&ScreenItem> {
		match self.state.selected() {
			Some(idx) => self.items.get(idx),
			None => None,
		}
	}

	pub fn is_selected(&self) -> bool {
		match self.state.selected() {
			Some(_) => true,
			None => false,
		}
	}

	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => if i >= self.items.len() - 1 {0} else {i + 1}
			None => self.index,
		};
		self.index = i;
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => if i == 0 {self.items.len() - 1} else {i - 1}
			None => self.index,
		};
		self.index = i;
		self.state.select(Some(i));
	}

	pub fn unselect(&mut self) {
		self.state.select(None);
	}
}

pub struct ListWidget<'a> {
	widget: List<'a>
}
impl<'a> ListWidget<'a> {
	pub fn new(stateful_list: &StatefulList, global_config: &GlobalConfig) -> ListWidget<'a> {

		let color = *stateful_list.get_color();
		let title = stateful_list.get_title();

		let items: Vec<ListItem> = stateful_list
			.items.iter().enumerate()
			.map(|(it, screen_item)| {
				let (name, active_text, arrows) = Self::get_item_text(it, screen_item, stateful_list, global_config);
				ListItem::new(Spans::from(vec![
					Span::from(name),
					Span::styled(active_text, Style::default().fg(color).add_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD).remove_modifier(Modifier::DIM)), 
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
			
		ListWidget { widget }
	}	

	fn get_item_text(it: usize, screen_item: &ScreenItem, stateful_list: &StatefulList, global_config: &GlobalConfig ) -> (String, String, String) {
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
			active_text = "• Active ".to_string();
		};

		(name, active_text, arrows)
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
 
