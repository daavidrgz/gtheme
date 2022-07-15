use clap::ArgMatches;

use crate::cli::utils;
use crate::core::postscript::PostScript;

pub fn run(matches: &ArgMatches) {
	let desktop = match utils::get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};
	let extra = match PostScript::get_extra_by_name(&desktop, matches.value_of("extra").unwrap()) {
		Some(t) => t,
		None => return
	};
	utils::edit_file(extra.get_path());
}