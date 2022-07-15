use clap::ArgMatches;

use crate::cli::utils;
use crate::core::{
	pattern::Pattern,
	config::DesktopConfig
};

pub fn run(matches: &ArgMatches, action: utils::Action) {
	let current_desktop_file = match utils::get_desktop( matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let mut desktop_config = DesktopConfig::new(&current_desktop_file);
	let patterns = matches.values_of("pattern").unwrap();
	for pattern_str in patterns {
		let pattern = match Pattern::get_by_name(&current_desktop_file, pattern_str) {
			Some(pattern) => pattern,
			None => continue
		};
		match action {
			utils::Action::Enable => desktop_config.enable_pattern(&pattern),
			utils::Action::Disable => desktop_config.disable_pattern(&pattern),
			utils::Action::Toggle => desktop_config.toggle_pattern(&pattern)
		}
	}
	desktop_config.save();
}