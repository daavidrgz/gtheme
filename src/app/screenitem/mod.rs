use crate::core::{
	desktop::DesktopFile,
	theme::{ThemeFile, Theme},
	pattern::{PatternFile},
	postscript::PostScript,
	config::{GlobalConfig, DesktopConfig}
};


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

	pub fn apply(&self, global_config: &mut GlobalConfig, desktop_config: &mut DesktopConfig) {
		match self {
			ScreenItem::Desktop(d) => Self::install_desktop(d.clone(), global_config, desktop_config),
			ScreenItem::Theme(t) => Self::apply_theme(t.clone(), global_config, desktop_config),
			ScreenItem::Pattern(p) => Self::toggle_pattern(p.clone(), desktop_config),
			ScreenItem::Extra(_) => {},
			ScreenItem::Help(_) => {}
		}
	}
	
	pub fn is_inverted(&self, desktop_config: &DesktopConfig) -> bool {
		match self {
			ScreenItem::Pattern(p) => *desktop_config.get_inverted().get(p.get_name()).unwrap_or(&false),
			_ => false
		}
	}

	pub fn is_active(&self, global_config: &GlobalConfig, desktop_config: &DesktopConfig) -> bool {
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
			ScreenItem::Pattern(p) => *desktop_config.get_actived().get(p.get_name()).unwrap_or(&true),
			ScreenItem::Extra(_) => true,
			ScreenItem::Help(_) => false
		}
	}

	fn toggle_pattern(pattern: PatternFile, desktop_config: &mut DesktopConfig) {
		let active_patterns = desktop_config.get_mut_actived();
		let current_status = *active_patterns.get(pattern.get_name()).unwrap_or(&true);

		active_patterns.insert(String::from(pattern.get_name()), !current_status);
		desktop_config.save()
	}

	fn apply_theme(theme: ThemeFile, global_config: &mut GlobalConfig, desktop_config: &mut DesktopConfig) {
		let current_desktop = global_config.get_current_desktop().as_ref()
			.expect("Can not apply a theme, there is no desktop installed").to_desktop();

		current_desktop.apply(&theme.to_theme(), desktop_config.get_actived(), desktop_config.get_inverted());

		*global_config.get_mut_current_theme() = Some(theme);
		global_config.save()
	}

	fn install_desktop(next_desktop: DesktopFile, global_config: &mut GlobalConfig, desktop_config: &mut DesktopConfig) {
		let current_desktop_opt = global_config.get_current_desktop().as_ref();
		let current_desktop = match current_desktop_opt {
			Some(d) => d.to_desktop(),
			None => next_desktop.to_desktop().clone()
		};

		let themes = Theme::get_themes();
		let aux_theme = themes.into_iter().find(|theme | theme.get_name() == "Nord").unwrap(); 

		let theme = desktop_config.get_default_theme().as_ref().unwrap_or(&aux_theme);

		*global_config.get_mut_current_desktop() = Some(next_desktop.clone());
		*global_config.get_mut_current_theme() = Some(theme.clone());
		global_config.save();

		next_desktop.to_desktop().install(&current_desktop, &theme.to_theme(), desktop_config.get_actived(), desktop_config.get_inverted())
	}
}
