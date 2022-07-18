use clap::ArgMatches;

use crate::cli::completions;
use crate::core::theme::Theme;

pub fn run(matches: &ArgMatches) {
	let themes = matches.values_of("theme").unwrap();

	for theme in themes {
		let theme_file = match Theme::get_by_name(theme) {
			Some(t) => t,
			None => continue
		};
	}

	completions::generate_completions()
}