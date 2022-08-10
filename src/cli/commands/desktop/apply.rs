use clap::ArgMatches;
use log::{error, warn};

use crate::cli::commands::utils;
use crate::cli::completions;
use crate::core::{
    config::{DesktopConfig, GlobalConfig},
    desktop::Desktop,
    theme::{Theme, ThemeFile},
};

pub fn run(matches: &ArgMatches) {
    let desktop_name = matches.value_of("desktop").unwrap();

    let current_desktop = match Desktop::get_by_name(desktop_name) {
        Some(d) => d,
        None => return,
    };

    let mut global_config = GlobalConfig::new();
    let previous_desktop = global_config
        .get_current_desktop()
        .as_ref()
        .map(|d| d.to_desktop());

    let desktop_config = DesktopConfig::new(&current_desktop);

    let default_theme: ThemeFile = match matches.value_of("theme") {
        Some(theme_name) => match Theme::get_by_name(theme_name) {
            Some(t) => t,
            None => return,
        },
        None => match desktop_config.get_default_theme() {
            Some(t) => t.clone(),
            None => {
                error!("There is no |default theme| specified in desktop |{}|. Try with -t option instead", current_desktop.get_name());
                return;
            }
        },
    };

    let actived = utils::get_actived(
        matches.values_of("pattern"),
        &current_desktop,
        &desktop_config,
    );

    let inverted = utils::get_inverted(
        matches.values_of("invert"),
        &current_desktop,
        &desktop_config,
    );

    let dry_run = matches.is_present("dry-run");
    if !dry_run {
        *global_config.get_mut_current_desktop() = Some(current_desktop.clone());
        *global_config.get_mut_current_theme() = Some(default_theme.clone());
        global_config.save();
        completions::generate_completions()
    }

    current_desktop.to_desktop().apply(
        &previous_desktop,
        &default_theme.to_theme(),
        &actived,
        &inverted,
        dry_run,
    );

    if previous_desktop.is_none() {
        warn!("|Reboot your computer to see the changes!|")
    }
}
