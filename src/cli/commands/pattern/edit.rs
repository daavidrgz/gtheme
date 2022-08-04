use clap::ArgMatches;
use log::error;

use crate::cli::commands::utils;
use crate::core::{pattern::Pattern, postscript::PostScript};

pub fn run(matches: &ArgMatches) {
    let desktop = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };
    let pattern = match Pattern::get_by_name(&desktop, matches.value_of("pattern").unwrap()) {
        Some(t) => t,
        None => return,
    };

    if matches.is_present("postscript") {
        match PostScript::get_postscript_by_name(&desktop, pattern.get_name()) {
            Some(ps) => utils::edit_file(ps.get_path()),
            None => error!("Pattern |{}| has no postscript", pattern.get_name()),
        }
        return;
    }

    if pattern.to_pattern().has_submodules() {
        utils::explore_directory(pattern.get_path());
    } else {
        utils::edit_file(pattern.get_path());
    }
}
