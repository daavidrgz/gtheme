use std::collections::HashMap;
use tui::style::Color;

use crate::core::{
	desktop::Desktop,
	theme::Theme,
	pattern::Pattern,
	postscript::PostScript,
	config::{GlobalConfig, DesktopConfig},
};
use crate::app::statefullist::StatefulList;
use crate::app::screenitem::ScreenItem;
use crate::app::widgets::HELP_CONTENT;

#[derive(Eq, PartialEq, Hash)]
pub enum Screen {
	Desktop,
	Theme
}
#[derive(Eq, PartialEq, Hash)]
pub enum Popup {
	Help,
	Extras,
	Info
}

pub struct AppState {
	current_screen: Screen,
	screens: HashMap<Screen, [StatefulList<ScreenItem>; 2]>,
	current_popup: Option<Popup>,
	popups: HashMap<Popup, StatefulList<ScreenItem>>,
	show_log: bool,
	global_config: GlobalConfig,
	desktop_config: Option<DesktopConfig>,
}
impl AppState {
	pub fn default(global_config: GlobalConfig) -> AppState {

		let desktop_config = match global_config.get_current_desktop() {
			Some(desktop_file) => Some(DesktopConfig::new(&desktop_file)),
			None => None
		};

		AppState {
			current_screen: Screen::Desktop,
			screens: Self::create_screens(&global_config),
			current_popup: None,
			popups: Self::create_popups(&global_config),
			show_log: false,
			global_config,
			desktop_config,
		}
	}
	
	pub fn get_mut_state(&mut self) -> (&mut Screen, &mut HashMap<Screen, [StatefulList<ScreenItem>; 2]>, &mut Option<Popup>,
		&mut HashMap<Popup, StatefulList<ScreenItem>>, &mut bool, &mut GlobalConfig, &mut Option<DesktopConfig>) {

		(&mut self.current_screen, &mut self.screens, &mut self.current_popup, &mut self.popups, &mut self.show_log, &mut self.global_config, &mut self.desktop_config)
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

	pub fn get_desktop_config(&self) -> &Option<DesktopConfig> {
		&self.desktop_config
	}
	pub fn get_mut_desktop_config(&mut self) ->&mut Option<DesktopConfig> {
		&mut self.desktop_config
	}

	fn create_screens(global_config: &GlobalConfig) -> HashMap<Screen, [StatefulList<ScreenItem>; 2]> {
		let desktops = Desktop::get_desktops().into_iter().map(|d| ScreenItem::Desktop(d)).collect();
		let desktops_list = StatefulList::with_items(desktops)
			.color(Color::Cyan)
			.title("DESKTOPS ")
			.selected(true);

		let patterns = match global_config.get_current_desktop() {
			None => vec![],
			Some(desktop) => Pattern::get_patterns(desktop).into_iter().map(|p| ScreenItem::Pattern(p)).collect()
		};
		let patterns_list = StatefulList::with_items(patterns)
			.color(Color::Magenta)
			.title("PATTERNS ")
			.active_text("• ON ")
			.inactive_text("• OFF ")
			.active_text_color(Color::Green)
			.inactive_text_color(Color::Red)
			.alignment(true);

		let fav_themes = global_config.get_fav_themes().into_iter().map(|f| ScreenItem::Theme(f.clone())).collect();
		let fav_themes_list = StatefulList::with_items(fav_themes)
			.color(Color::Blue)
			.title("FAV-THEMES ")
			.selected(true);

		let themes = Theme::get_themes().into_iter().map(|t| ScreenItem::Theme(t)).collect();
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
		popups.insert(Popup::Info, StatefulList::with_items(vec![]));
		popups
	}

	fn create_help_list() -> StatefulList<ScreenItem> {
		let mut lines: Vec<ScreenItem> = vec![];
		for l in HELP_CONTENT.lines() {
			lines.push(ScreenItem::Help(l.to_string()));
		}
		StatefulList::with_items(lines)
			.color(Color::Yellow)
			.title("HELP ")
	}

	fn create_extras_list(global_config: &GlobalConfig) -> StatefulList<ScreenItem> {
		let extras = match global_config.get_current_desktop() {
			None => vec![],
			Some(desktop) => PostScript::get_extras(desktop).into_iter().map(|e| ScreenItem::Extra(e)).collect()
		};

		StatefulList::with_items(extras)
			.color(Color::Magenta)
			.title("EXTRAS ")
			.active_text("• ON ")
			.inactive_text("• OFF ")
			.active_text_color(Color::Green)
			.inactive_text_color(Color::Red)
			.alignment(true)
	}
}
