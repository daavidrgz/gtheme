use clap::ArgMatches;

use crate::cli::commands::utils;
use crate::core::{config::DesktopConfig, pattern::Pattern};

pub fn run(matches: &ArgMatches) {
    let current_desktop_file = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };
    let mut desktop_config = DesktopConfig::new(&current_desktop_file);

    let patterns = matches.values_of("pattern").unwrap();
    for pattern_str in patterns {
        let pattern = match Pattern::get_by_name(&current_desktop_file, pattern_str) {
            Some(pattern) => pattern,
            None => continue,
        };
        desktop_config.toggle_invert_pattern(&pattern);
    }
    desktop_config.save();
}
