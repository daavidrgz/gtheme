use clap::ArgMatches;

use crate::cli::completions;
use crate::core::desktop::Desktop;

pub fn run(matches: &ArgMatches) {
	let desktops = matches.values_of("desktop").unwrap();

	for desktop in desktops{
		let desktop_file = match Desktop::get_by_name(desktop) {
			Some(d) => d,
			None => continue
		};
		desktop_file.remove();
	}
	completions::generate_completions()
}
