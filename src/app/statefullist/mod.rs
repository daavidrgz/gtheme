use tui::{
	widgets::ListState,
	style::Color,
};
use crate::core::config::GlobalConfig;
use crate::app::screenitem::ScreenItem;

pub struct StatefulList<T> {
	state: ListState,
	index: usize,
	infinite: bool,
	items: Vec<T>,
	color: Color,
	title: String
}
impl<T> StatefulList<T> {
	pub fn with_items(items: Vec<T>, color: Color, title: String, selected: bool, infinite: bool) -> StatefulList<T> {
		let mut state = ListState::default();
		state.select(if selected {Some(0)} else {None});
		StatefulList {
			index: 0,
			infinite,
			color,
			title,
			state,
			items,
		}
	}

	pub fn get_state(&self) -> &ListState {
		&self.state
	}
	pub fn get_mut_state(&mut self) -> &mut ListState {
		&mut self.state
	}
	pub fn get_items(&self) -> &Vec<T> {
		&self.items
	}
	pub fn get_mut_items(&mut self) -> &mut Vec<T> {
		&mut self.items
	}
	pub fn get_color(&self) -> &Color {
		&self.color
	}
	pub fn get_title(&self) -> &String {
		&self.title
	}
	pub fn get_infinite(&self) -> &bool {
		&self.infinite
	}
	pub fn get_length(&self) -> usize {
		self.items.len()
	}
	
	pub fn get_selected(&self) -> Option<&T> {
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
			Some(i) => {
				if i >= self.items.len() - 1 {
					if !self.get_infinite() {i} else {0}
				} else {
					i + 1
				}
			},
			None => self.index,
		};
		self.index = i;
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					if !self.get_infinite() {i} else {self.items.len() - 1}
				} else {
					i - 1
				}
			},
			None => self.index,
		};
		self.index = i;
		self.state.select(Some(i));
	}

	pub fn unselect(&mut self) {
		self.state.select(None);
	}
}


impl StatefulList<ScreenItem> {
	pub fn add_fav(&mut self, item: &ScreenItem, global_config: &mut GlobalConfig) {
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
}
