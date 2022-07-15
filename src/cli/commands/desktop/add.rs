use clap::ArgMatches;
use std::path::Path;
use log::error;

use crate::cli::completions;
use crate::core::desktop::Desktop;

pub fn run(matches: &ArgMatches) {
	let desktops = matches.values_of("path").unwrap();
	for desktop in desktops {
		if let Err(desktop_opt) = Desktop::add(Path::new(desktop)) {
			if let Some(desktop_file) = desktop_opt {
				if matches.is_present("force") {
					desktop_file.remove();
					let _ = Desktop::add(Path::new(desktop));
				} else {
					error!("Desktop |{}| already exists", desktop_file.get_name())
				}
			} 
		}
	}
	completions::generate_completions()
}