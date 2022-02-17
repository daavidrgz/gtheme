use crate::core::{
	desktop::{DesktopFile, Desktop},
	theme::ThemeFile,
	pattern::{PatternFile, Pattern},
	config::GlobalConfig
};

use std::collections::HashMap;

#[derive(Clone)]
pub enum ScreenItem {
	Desktop(DesktopFile),
	Theme(ThemeFile),
	Pattern(PatternFile)
}
impl ScreenItem {
	pub fn get_name(&self) -> &str {
		match self {
			ScreenItem::Desktop(d) => d.get_name(),
			ScreenItem::Theme(t) => t.get_name(),
			ScreenItem::Pattern(p) => p.get_name()
		}
	}

	pub fn apply(&self, global_config: &mut GlobalConfig) {
		match self {
			ScreenItem::Desktop(d) => ScreenItem::install_desktop(d.to_desktop()),
			ScreenItem::Theme(t) => ScreenItem::apply_theme(t.clone(), global_config),
			ScreenItem::Pattern(_) => {}
		}
	}

	fn apply_theme(theme: ThemeFile, global_config: &mut GlobalConfig) {
		let current_desktop = global_config.get_current_desktop().as_ref()
			.expect("Can not apply a theme, there is no desktop installed").to_desktop();

		let patterns = Pattern::get_patterns(current_desktop.get_name());
		let mut actived = HashMap::new();
		for pattern in patterns{
			actived.insert(String::from(pattern.get_name()),true);
		}
		actived.insert(String::from("wallpaper"),true);

		current_desktop.apply(&theme.to_theme(), actived, HashMap::new());

		*global_config.get_mut_current_theme() = Some(theme);
		global_config.save()
	}

	fn install_desktop(desktop: Desktop) {
	}
}
