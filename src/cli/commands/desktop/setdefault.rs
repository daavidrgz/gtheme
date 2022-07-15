use clap::ArgMatches;

use crate::cli::utils;
use crate::core::{
	theme::Theme,
	config::DesktopConfig
};

pub fn run(matches: &ArgMatches) {
	let desktop = match utils::get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let theme = match Theme::get_by_name(matches.value_of("theme").unwrap()) {
		Some(t) => t,
		None => return
	};

	let mut desktop_config = DesktopConfig::new(&desktop);
	desktop_config.set_default_theme(&theme);
	desktop_config.save()
}