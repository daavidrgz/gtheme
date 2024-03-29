use clap::ArgMatches;
use colored::*;
use log::error;
use std::collections::BTreeSet;

use crate::core::{config::GlobalConfig, theme::Theme};
use crate::utils;

pub fn run(matches: &ArgMatches) {
    let theme_file = match matches.value_of("theme") {
        Some(t) => match Theme::get_by_name(t) {
            Some(t) => t,
            None => return,
        },
        None => {
            let global_config = GlobalConfig::new();
            match global_config.get_current_theme() {
                Some(t) => t.clone(),
                None => {
                    error!("|There is no theme installed!|, try specifing a theme");
                    return;
                }
            }
        }
    };

    let theme = theme_file.to_theme();
    let sorted_colors = theme.get_colors().into_iter().collect::<BTreeSet<_>>();

    println!(
        "\n{} {}\n",
        "THEME".bold().underline().green(),
        theme.get_name().bold()
    );
    for (color_key, color_value) in sorted_colors {
        let hex_color = format!("#{}", &color_value);
        match utils::hex_to_rgb(&hex_color) {
            Some((r, g, b)) => println!(
                "{hex_color}  {}  {}",
                "██".truecolor(r, g, b),
                color_key.bold().cyan()
            ),
            None => error!(
                "Invalid hexadcimal color '|{}|' in property |{}|",
                color_value, color_key
            ),
        }
    }
    println!();
}
