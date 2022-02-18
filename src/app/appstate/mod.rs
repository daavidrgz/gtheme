use std::collections::HashMap;
use crate::app::widgets::StatefulList;
use tui::style::Color;

use crate::core::{
	desktop::Desktop,
	theme::Theme,
	pattern::Pattern,
	config::GlobalConfig
};

use crate::app::screenitem::ScreenItem;

#[derive(Eq, PartialEq, Hash)]
pub enum Screen {
	Desktop,
	Theme
}

pub struct AppState {
	current_screen: Screen,
	map: HashMap<Screen, [StatefulList; 2]>,
	global_config: GlobalConfig, 
	show_popup: bool
}
impl AppState {
	pub fn default(global_config: GlobalConfig) -> AppState {
		AppState {
			current_screen: Screen::Desktop,
			map: AppState::create_lists(&global_config),
			global_config: GlobalConfig::new(),
			show_popup: false
		}
	}
	
	pub fn get_mut_state(&mut self) -> (&mut Screen, &mut HashMap<Screen, [StatefulList; 2]>, &mut GlobalConfig, &mut bool) {
		(&mut self.current_screen, &mut self.map, &mut self.global_config, &mut self.show_popup)
	}
	pub fn get_mut_screen(&mut self) -> &mut Screen {
		&mut self.current_screen
	}

	pub fn get_global_config(&self) -> &GlobalConfig {
		&self.global_config
	}
	pub fn get_mut_global_config(&mut self) ->&mut GlobalConfig {
		&mut self.global_config
	}

	fn create_lists(global_config: &GlobalConfig) -> HashMap<Screen, [StatefulList; 2]> {
		let desktops = Desktop::get_desktops().into_iter().map(|e|ScreenItem::Desktop(e)).collect();
		let desktops_list = StatefulList::with_items(desktops, Color::Cyan, "DESKTOPS ".to_string(), true);

		let patterns = Pattern::get_patterns("simple").into_iter().map(|e|ScreenItem::Pattern(e)).collect();
		let patterns_list = StatefulList::with_items(patterns, Color::Magenta,  "PATTERNS ".to_string(), false);

		let fav_themes = global_config.get_fav_themes().into_iter().map(|e|ScreenItem::Theme(e.clone())).collect();
		let fav_themes_list = StatefulList::with_items(fav_themes, Color::Blue, "FAV-THEMES ".to_string(), true);

		let themes = Theme::get_themes().into_iter().map(|e|ScreenItem::Theme(e)).collect();
		let themes_list = StatefulList::with_items(themes, Color::Green, "THEMES ".to_string(), false);

		let mut map = HashMap::new();
		map.insert(Screen::Desktop, [desktops_list, patterns_list]);
		map.insert(Screen::Theme, [fav_themes_list, themes_list]);
		map
	}
}
