use crate::core::{
	desktop::DesktopFile,
	theme::{ThemeFile, Theme},
	pattern::{PatternFile, Pattern},
	postscript::PostScript,
	config::GlobalConfig
};

use std::collections::HashMap;

#[derive(Clone)]
pub enum ScreenItem {
	Desktop(DesktopFile),
	Theme(ThemeFile),
	Pattern(PatternFile),
	Extra(PostScript),
	Help(String)
}
impl ScreenItem {
	pub fn get_name(&self) -> &str {
		match self {
			ScreenItem::Desktop(d) => d.get_name(),
			ScreenItem::Theme(t) => t.get_name(),
			ScreenItem::Pattern(p) => p.get_name(),
			ScreenItem::Extra(e) => e.get_name(),
			ScreenItem::Help(s) => &s
		}
	}

	pub fn get_theme(&self) -> Option<&ThemeFile> {
		match self {
			ScreenItem::Theme(t) => Some(t),
			_ => None
		}
	}
	pub fn get_desktop(&self) -> Option<&DesktopFile> {
		match self {
			ScreenItem::Desktop(d) => Some(d),
			_ => None,
		}
	}
	pub fn get_pattern(&self) -> Option<&PatternFile> {
		match self {
			ScreenItem::Pattern(p) => Some(p),
			_ => None
		}
	}
	pub fn get_extra(&self) -> Option<&PostScript> {
		match self {
			ScreenItem::Extra(e) => Some(e),
			_ => None
		}
	}
	pub fn get_help(&self) -> Option<&String> {
		match self {
			ScreenItem::Help(s) => Some(s),
			_ => None
		}
	}

	pub fn apply(&self, global_config: &mut GlobalConfig) {
		match self {
			ScreenItem::Desktop(d) => ScreenItem::install_desktop(d.clone(), global_config),
			ScreenItem::Theme(t) => ScreenItem::apply_theme(t.clone(), global_config),
			ScreenItem::Pattern(_) => {},
			ScreenItem::Extra(e) => {},
			ScreenItem::Help(_) => {}
		}
	}

	pub fn is_active(&self, global_config: &GlobalConfig) -> bool {
		match self {
			ScreenItem::Desktop(d) => {
				match global_config.get_current_desktop() {
					Some(current_desktop) => d.get_name() == current_desktop.get_name(),
					None => false
				}
			},
			ScreenItem::Theme(t) => {
				match global_config.get_current_theme() {
					Some(current_theme) => t.get_name() == current_theme.get_name(),
					None => false
				}
			},
			ScreenItem::Pattern(_) => false,
			ScreenItem::Extra(e) => true,
			ScreenItem::Help(_) => false
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
		let theme = themes.into_iter().find(|theme |theme.get_name() == "Japan-Dark").unwrap(); 

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
