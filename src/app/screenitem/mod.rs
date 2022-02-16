use crate::core::{
	desktop::{DesktopFile, Desktop},
	theme::{ThemeFile, Theme},
	pattern::{PatternFile, Pattern}
};

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

	pub fn apply(&self) {
		match self {
			ScreenItem::Desktop(d) => ScreenItem::install_desktop(d.to_desktop()),
			ScreenItem::Theme(t) => ScreenItem::apply_theme(t.to_theme()),
			ScreenItem::Pattern(p) => {}
		}
	}

	fn apply_theme(theme: Theme) {

	}

	fn install_desktop(desktop: Desktop) {
	}
}
