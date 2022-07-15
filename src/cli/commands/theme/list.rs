use clap::ArgMatches;
use colored::*;
use term_grid::{Grid, GridOptions, Direction, Filling};

use crate::cli::commands;
use crate::core::{
	theme::Theme,
	config::GlobalConfig
};

pub fn run(matches: &ArgMatches) {
	if matches.is_present("favs") {
		commands::fav::list::run();
		return
	}

	println!("");
	let all_themes = Theme::get_themes();
	let global_config = GlobalConfig::new();
	let current_theme = match global_config.get_current_theme() {
		Some(t) => t.get_name(),
		None => ""
	};

	println!("{}\n", "THEMES".bold().underline().yellow());

	let print_themes: Vec<String> = all_themes.into_iter().map(|t| {
		if t.get_name() == current_theme {
			format!("{} {} (Active)", "•".green(), t.get_name())
		} else {
			format!("{} {}", "•".yellow(), t.get_name())
		}
	}).collect();

	let mut grid = Grid::new(GridOptions {
		filling: Filling::Spaces(2),
		direction: Direction::TopToBottom,
	});
	
	for s in print_themes {
		grid.add(s.into());
	}

	// let term_width: usize = match terminal_size() {
	// 	Some((width, _)) => width.0.into(),
	// 	None => return 
	// };
	
	println!("{}", grid.fit_into_columns(3));
}