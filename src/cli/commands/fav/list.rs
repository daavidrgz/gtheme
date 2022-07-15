use colored::*;
use crate::core::config::GlobalConfig;

pub fn run() {
	println!("");
	let global_config = GlobalConfig::new();
	let current_theme = match global_config.get_current_theme() {
		Some(t) => t.get_name(),
		None => ""
	};

	let fav_themes = global_config.get_fav_themes();

	println!("{}\n", "FAV THEMES".bold().underline().blue());

	for t in fav_themes {
		if t.get_name() == current_theme {
			println!("{} {}", "•".green(), format!("{} (Active)", t.get_name()).bold().green());
		} else {
			println!("{} {}", "•".blue(), t.get_name());
		};
	}
	println!("");
}