use clap::ArgMatches;
use colored::*;
use term_grid::{Direction, Filling, Grid, GridOptions};

use crate::cli::commands;
use crate::core::{
    config::GlobalConfig,
    theme::{Theme, ThemeFile},
};

fn get_themes(all_themes: &[ThemeFile], current_theme: &str) -> Vec<String> {
    let themes = all_themes
        .iter()
        .map(|t| {
            if t.get_name() == current_theme {
                format!("{} {} (Active)", "•".green(), t.get_name())
            } else {
                format!("{} {}", "•".yellow(), t.get_name())
            }
        })
        .collect();

    themes
}

fn create_grid(items: Vec<String>, options: GridOptions) -> Grid {
    let mut grid = Grid::new(options);
    for s in items {
        grid.add(s.into());
    }
    grid
}

pub fn run(matches: &ArgMatches) {
    if matches.is_present("favs") {
        commands::fav::list::run(matches);
        return;
    }

    let all_themes = Theme::get_themes();
    let global_config = GlobalConfig::new();
    let current_theme = match global_config.get_current_theme() {
        Some(t) => t.get_name(),
        None => "",
    };

    if matches.is_present("quiet") {
        all_themes
            .iter()
            .for_each(|theme| println!("{}", theme.get_name()));
        return;
    }

    println!();
    println!("{}\n", "THEMES".bold().underline().yellow());
    let formatted_themes = get_themes(&all_themes, current_theme);
    let options = GridOptions {
        filling: Filling::Spaces(2),
        direction: Direction::TopToBottom,
    };
    let grid = create_grid(formatted_themes, options);
    println!("{}", grid.fit_into_columns(3));
}
