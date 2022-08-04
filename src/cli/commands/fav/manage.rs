use clap::ArgMatches;

use crate::cli::commands::utils::Action;
use crate::cli::completions;
use crate::core::{config::GlobalConfig, theme::Theme};

pub fn run(matches: &ArgMatches, action: Action) {
    let mut global_config = GlobalConfig::new();

    let themes = matches.values_of("theme").unwrap();
    for theme_name in themes {
        let theme = match Theme::get_by_name(theme_name) {
            Some(t) => t,
            None => continue,
        };
        match action {
            Action::Enable => global_config.add_fav_theme(&theme),
            Action::Disable => global_config.remove_fav_theme(&theme),
            Action::Toggle => global_config.toggle_fav_theme(&theme),
        }
    }
    global_config.save();
    completions::generate_completions()
}
