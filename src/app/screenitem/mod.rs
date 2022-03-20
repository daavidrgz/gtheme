use std::process::Command;
use log::warn;

use crate::core::{
	desktop::DesktopFile,
	theme::{ThemeFile, Theme},
	pattern::PatternFile,
	postscript::PostScript,
	config::{GlobalConfig, DesktopConfig}
};

#[derive(Clone)]
pub enum ScreenItem {
	Desktop(DesktopFile),
	Theme(ThemeFile),
	Pattern(PatternFile),
	Extra(PostScript),
	Help(String),
}
impl ScreenItem {
	pub fn get_name(&self) -> &str {
		match self {
			ScreenItem::Desktop(d) => d.get_name(),
			ScreenItem::Theme(t) => t.get_name(),
			ScreenItem::Pattern(p) => p.get_name(),
			ScreenItem::Extra(e) => e.get_name(),
			ScreenItem::Help(s) => &s,
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

	pub fn edit(&self) {
		match self {
			ScreenItem::Pattern(p) => {Command::new("nano").arg(p.get_path()).output()
				.expect(&format!("Could not edit file:{}", p.get_path()));},
			ScreenItem::Extra(e) => {Command::new("nano").arg(e.get_path()).output()
				.expect(&format!("Could not edit file:{}", e.get_path()));},
			_ => {}
		}
	}

	pub fn apply(&self, global_config: &mut GlobalConfig, desktop_config: &mut Option<DesktopConfig>) {
		match self {
			ScreenItem::Desktop(d) => Self::apply_desktop(d, global_config),
			ScreenItem::Theme(t) => Self::apply_theme(t, global_config, desktop_config),
			ScreenItem::Pattern(_) => Self::toggle_active(self, desktop_config),
			ScreenItem::Extra(_) => Self::toggle_active(self, desktop_config),
			ScreenItem::Help(_) => ()
		}
	}

	pub fn invert(&self, desktop_config: &mut DesktopConfig) {
		match self {
			ScreenItem::Pattern(p) => {
				desktop_config.toggle_invert_pattern(p);
				desktop_config.save()
			},
			_ => {}
		}
	}
	
	pub fn is_inverted(&self, desktop_config: &Option<DesktopConfig>) -> bool {
		match self {
			ScreenItem::Pattern(p) => {
				match desktop_config {
					Some(d_config) => *d_config.get_inverted().get(p.get_name()).unwrap_or(&false),
					None => false
				}
			},
			_ => false
		}
	}

	pub fn is_active(&self, global_config: &GlobalConfig, desktop_config: &Option<DesktopConfig>) -> bool {
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
			ScreenItem::Pattern(p) => { 
				match desktop_config {
					Some(d_config) => *d_config.get_actived().get(p.get_name()).unwrap_or(&false),
					None => true
				}
			},
			ScreenItem::Extra(e) => { 
				match desktop_config {
					Some(d_config) => *d_config.get_actived().get(e.get_name()).unwrap_or(&false),
					None => false
				}
			},
			ScreenItem::Help(_) => false
		}
	}

	fn toggle_active(item: &ScreenItem, desktop_config_opt: &mut Option<DesktopConfig>) {
		let desktop_config = match desktop_config_opt {
			None => {
				warn!("Cannot activate item, |there is no desktop installed!|");
				return
			}
			Some(config) => config,
		};

		match item {
			ScreenItem::Pattern(p) => desktop_config.toggle_pattern(p),
			ScreenItem::Extra(e) => desktop_config.toggle_extra(e),
			_ => ()
		}
		desktop_config.save()
	}

	fn apply_theme(theme: &ThemeFile, global_config: &mut GlobalConfig, desktop_config_opt: &mut Option<DesktopConfig>) {
		let current_desktop = match global_config.get_current_desktop() {
			Some(d) => d.to_desktop(),
			None => {
				warn!("Cannot apply a theme, |there is no desktop installed!|");
				return
			}
		};
		let desktop_config = match desktop_config_opt {
			Some(config) => config,
			None => {
				warn!("Cannot apply a theme, |there is no desktop installed!|");
				return
			}
		};

		current_desktop.apply(
			&theme.to_theme(),
			desktop_config.get_actived(),
			desktop_config.get_inverted(),
			false
		);

		*global_config.get_mut_current_theme() = Some(theme.clone());
		global_config.save()
	}

	fn apply_desktop(next_desktop: &DesktopFile, global_config: &mut GlobalConfig){
		let current_desktop = match global_config.get_current_desktop() {
			Some(d) => Some(d.to_desktop()),
			None => None
		};
		
		let aux_theme = Theme::get_by_name("Nord").unwrap(); 

		let next_desktop_config = DesktopConfig::new(&next_desktop);
		let theme = next_desktop_config.get_default_theme().as_ref().unwrap_or(&aux_theme);

		*global_config.get_mut_current_desktop() = Some(next_desktop.clone());
		*global_config.get_mut_current_theme() = Some(theme.clone());
		global_config.save();

		next_desktop.to_desktop().install(
			&current_desktop,
			&theme.to_theme(),
			next_desktop_config.get_actived(),
			next_desktop_config.get_inverted(),
			false
		)
	}
}
