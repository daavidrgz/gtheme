use std::collections::HashMap;
use crate::app::widgets::StatefulList;
use tui::style::Color;

use crate::core::{
	desktop::Desktop,
	theme::Theme,
	pattern::Pattern
};

use crate::app::screenitem::ScreenItem;

#[derive(Eq, PartialEq, Hash)]
pub enum Screen {
	Desktop,
	Theme
}

pub struct AppState {
	current_screen: Screen,
	lists: HashMap<Screen, [StatefulList<ScreenItem>; 2]>
}
impl AppState {
	pub fn default() -> AppState {
		AppState {
			current_screen: Screen::Desktop,
			lists: AppState::create_lists()
		}
	}
	
	pub fn get_state(&mut self) -> (&mut Screen, &mut HashMap<Screen, [StatefulList<ScreenItem>; 2]>) {
		(&mut self.current_screen, &mut self.lists)
	}

	pub fn get_screen(&mut self) -> &mut Screen {
		&mut self.current_screen
	}

	pub fn set_screen(&mut self, screen: Screen) {
		self.current_screen = screen;
	}

	fn create_lists() -> HashMap<Screen, [StatefulList<ScreenItem>; 2]> {
		let desktops = Desktop::get_desktops().into_iter().map(|e|ScreenItem::Desktop(e)).collect();
		let desktops_list = StatefulList::with_items(desktops, Color::Cyan, "DESKTOPS ".to_string(), true);

		let patterns = Pattern::get_patterns("simple").into_iter().map(|e|ScreenItem::Pattern(e)).collect();
		let patterns_list = StatefulList::with_items(patterns, Color::Magenta,  "PATTERNS ".to_string(), false);

		let fav_themes = Theme::get_themes().into_iter().map(|e|ScreenItem::Theme(e)).collect();
		let fav_themes_list = StatefulList::with_items(fav_themes, Color::Blue, "FAV-THEMES ".to_string(), true);

		let themes = Theme::get_themes().into_iter().map(|e|ScreenItem::Theme(e)).collect();
		let themes_list = StatefulList::with_items(themes, Color::Green, "THEMES ".to_string(), false);

		let mut map = HashMap::new();
		map.insert(Screen::Desktop, [desktops_list, patterns_list]);
		map.insert(Screen::Theme, [fav_themes_list, themes_list]);
		map
	}
}
