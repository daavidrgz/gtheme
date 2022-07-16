use clap::ArgMatches;

use crate::cli::completions;
use crate::core::theme::Theme;

pub fn run(matches: &ArgMatches) {
	let theme_name = matches.value_of("name").unwrap();
	Theme::new_skeleton(theme_name);
	completions::generate_completions()
}