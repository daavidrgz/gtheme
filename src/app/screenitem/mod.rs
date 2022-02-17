use crate::core::{
	desktop::{DesktopFile, Desktop},
	theme::{ThemeFile, Theme},
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
			ScreenItem::Desktop(d) => ScreenItem::install_desktop(d.clone(), global_config),
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
		actived.insert(String::from("vscode"),true);

		current_desktop.apply(&theme.to_theme(), actived, HashMap::new());

		*global_config.get_mut_current_theme() = Some(theme);
		global_config.save()
	}

	fn install_desktop(next_desktop: DesktopFile, global_config: &mut GlobalConfig) {
		let previous_desktop_opt = global_config.get_current_desktop().as_ref();
		let current_desktop = match previous_desktop_opt {
			Some(d) => d.to_desktop(),
			None => next_desktop.to_desktop().clone()
		};

		let themes = Theme::get_themes();
		let theme = themes.into_iter().find(|theme |theme.get_name()=="Japan-Dark" ).unwrap(); 

		let patterns = Pattern::get_patterns(next_desktop.get_name());
		let mut actived = HashMap::new();
		for pattern in patterns{
			actived.insert(String::from(pattern.get_name()),true);
		}
		actived.insert(String::from("wallpaper"),true);
		actived.insert(String::from("vscode"),true);

		*global_config.get_mut_current_desktop() = Some(next_desktop.clone());
		*global_config.get_mut_current_theme() = Some(theme.clone());
		global_config.save();

		next_desktop.to_desktop().install(&current_desktop, &theme.to_theme(), actived, HashMap::new())
	}
}
