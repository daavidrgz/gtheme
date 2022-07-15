use clap::ArgMatches;
use colored::*;

use crate::cli::utils;
use crate::core::{
	postscript::PostScript,
	config::DesktopConfig
};

pub fn run(matches: &ArgMatches) {
	println!("");
	let desktop = match utils::get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let all_extras = PostScript::get_extras(&desktop);
	let desktop_config = DesktopConfig::new(&desktop);

	let enabled = desktop_config.get_actived();

	let desktop_title = format!("({})", desktop.get_name());

	println!("{} {}\n", "EXTRAS".bold().underline().red(), desktop_title.bold().cyan());

	for p in all_extras {
		print!("{} {:<20}", "•".red(), p.get_name());
		match enabled.get(p.get_name()) {
			Some(e) => if *e {
					print!(" {}\n", "ON".bold().green());
				} else {
					print!(" {}\n", "OFF".bold().red());
				},
			None => print!(" {}\n", "OFF".bold().red())
		}
	}
	println!("");
}