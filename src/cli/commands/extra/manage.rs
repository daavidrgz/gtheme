use clap::ArgMatches;

use crate::cli::utils;
use crate::core::{config::DesktopConfig, postscript::PostScript};

pub fn run(matches: &ArgMatches, action: utils::Action) {
    let current_desktop_file = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };

    let mut desktop_config = DesktopConfig::new(&current_desktop_file);

    let extras = matches.values_of("extra").unwrap();
    for extra_str in extras {
        let extra = match PostScript::get_extra_by_name(&current_desktop_file, extra_str) {
            Some(pattern) => pattern,
            None => continue,
        };
        match action {
            utils::Action::Enable => desktop_config.enable_extra(&extra),
            utils::Action::Disable => desktop_config.disable_extra(&extra),
            utils::Action::Toggle => desktop_config.toggle_extra(&extra),
        }
    }

    desktop_config.save();
}
