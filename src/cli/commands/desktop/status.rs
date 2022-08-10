use clap::ArgMatches;
use colored::*;

use crate::cli::commands;
use crate::cli::commands::utils;
use crate::core::config::{DesktopConfig, GlobalConfig};

pub fn run(matches: &ArgMatches) {
    println!();
    let global_config = GlobalConfig::new();
    let desktop = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };

    let desktop_config = DesktopConfig::new(&desktop);

    let default_theme_name = match desktop_config.get_default_theme() {
        Some(t) => t.get_name(),
        None => "",
    };

    println!(
        "{}\n",
        desktop.get_name().to_uppercase().bold().underline().cyan()
    );
    println!(
        "{} {}",
        "• Default theme:".green().bold(),
        default_theme_name
    );

    if let Some(d) = global_config.get_current_desktop() {
        if d.get_name() == desktop.get_name() {
            let current_theme = match global_config.get_current_theme() {
                Some(t) => t.get_name(),
                None => "",
            };
            println!("{} {}", "• Current theme:".yellow().bold(), current_theme)
        }
    }
    println!();

    commands::pattern::list::run(matches);
    commands::extra::list::run(matches);
}
