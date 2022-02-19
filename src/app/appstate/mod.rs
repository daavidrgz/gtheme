use std::collections::HashMap;
use tui::style::Color;
use std::fs::File;
use std::io::{self, BufRead};

use crate::core::{
	self,
	desktop::Desktop,
	theme::Theme,
	pattern::Pattern,
	postscript::PostScript,
	config::GlobalConfig
};
use crate::app::statefullist::StatefulList;
use crate::app::screenitem::ScreenItem;

#[derive(Eq, PartialEq, Hash)]
pub enum Screen {
	Desktop,
	Theme
}
#[derive(Eq, PartialEq, Hash)]
pub enum Popup {
	Help,
	Extras
}

pub struct AppState {
	current_screen: Screen,
	screens: HashMap<Screen, [StatefulList<ScreenItem>; 2]>,
	global_config: GlobalConfig, 
	current_popup: Option<Popup>,
	popups: HashMap<Popup, StatefulList<ScreenItem>>,
}
impl AppState {
	pub fn default(global_config: GlobalConfig) -> AppState {
		AppState {
			current_screen: Screen::Desktop,
			screens: Self::create_screens(&global_config),
			current_popup: None,
			popups: Self::create_popups(&global_config),
			global_config,
		}
	}
	
	pub fn get_mut_state(&mut self) -> (&mut Screen, &mut HashMap<Screen, [StatefulList<ScreenItem>; 2]>, &mut Option<Popup>, &mut HashMap<Popup, StatefulList<ScreenItem>>, &mut GlobalConfig) {
		(&mut self.current_screen, &mut self.screens, &mut self.current_popup, &mut self.popups, &mut self.global_config)
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

	fn create_screens(global_config: &GlobalConfig) -> HashMap<Screen, [StatefulList<ScreenItem>; 2]> {
		let current_desktop = global_config.get_current_desktop();
		let desktop_str = match current_desktop {
			Some(d) => d.get_name(),
			None => ""
		};

		let desktops = Desktop::get_desktops().into_iter().map(|d|ScreenItem::Desktop(d)).collect();
		let desktops_list = StatefulList::with_items(desktops)
			.color(Color::Cyan)
			.title("DESKTOPS ")
			.selected(true);

		let patterns = Pattern::get_patterns(desktop_str).into_iter().map(|p|ScreenItem::Pattern(p)).collect();
		let patterns_list = StatefulList::with_items(patterns)
			.color(Color::Magenta)
			.title("PATTERNS ")
			.inactive_text("• Inactive ");

		let fav_themes = global_config.get_fav_themes().into_iter().map(|f|ScreenItem::Theme(f.clone())).collect();
		let fav_themes_list = StatefulList::with_items(fav_themes)
			.color(Color::Blue)
			.title("FAV-THEMES ")
			.selected(true);

		let themes = Theme::get_themes().into_iter().map(|t|ScreenItem::Theme(t)).collect();
		let themes_list = StatefulList::with_items(themes)
			.color(Color::Green)
			.title("THEMES ")
			.infinite(true);

		let mut map = HashMap::new();
		map.insert(Screen::Desktop, [desktops_list, patterns_list]);
		map.insert(Screen::Theme, [fav_themes_list, themes_list]);
		map
	}

	fn create_popups(global_config: &GlobalConfig) -> HashMap<Popup, StatefulList<ScreenItem>> {
		let mut popups = HashMap::new();
		popups.insert(Popup::Extras, Self::create_extras_list(global_config));
		popups.insert(Popup::Help, Self::create_help_list());
		popups
	}

	fn create_help_list() -> StatefulList<ScreenItem> {
		let help_path = core::expand_path(&format!("{}/assets/help.txt", core::GTHEME_HOME));
		let help_file = File::open(&help_path).expect(&format!("Error while opening logo file in {}", &help_path));
		let file_lines = io::BufReader::new(help_file).lines();

		let lines = file_lines.into_iter().map(|line| ScreenItem::Help(line.unwrap())).collect();
		StatefulList::with_items(lines)
			.color(Color::Yellow)
			.title("HELP ")
	}

	fn create_extras_list(global_config: &GlobalConfig) -> StatefulList<ScreenItem> {
		let current_desktop = global_config.get_current_desktop();
		let desktop_str = match current_desktop {
			Some(d) => d.get_name(),
			None => ""
		};

		let extras = PostScript::get_extras(desktop_str).into_iter().map(|e|ScreenItem::Extra(e)).collect();
		StatefulList::with_items(extras)
			.color(Color::Red)
			.title("EXTRAS ")
			.active_text("• ON ")
			.inactive_text("• OFF ")
	}
}
