use clap::ArgMatches;

use crate::cli::{completions, utils};
use crate::core::desktop::Desktop;

pub fn run(matches: &ArgMatches) {
    let desktop = match Desktop::get_by_name(matches.value_of("desktop").unwrap()) {
        Some(t) => t,
        None => return,
    };
    utils::explore_directory(desktop.get_path());
    completions::generate_completions()
}
