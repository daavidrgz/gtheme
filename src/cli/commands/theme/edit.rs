use clap::ArgMatches;

use crate::cli::utils;
use crate::core::theme::Theme;

pub fn run(matches: &ArgMatches) {
	let theme = match Theme::get_by_name(matches.value_of("theme").unwrap()) {
		Some(t) => t,
		None => return
	};
	utils::edit_file(theme.get_path());
}