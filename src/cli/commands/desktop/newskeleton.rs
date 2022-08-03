use clap::ArgMatches;

use crate::cli::completions;
use crate::core::desktop::Desktop;

pub fn run(matches: &ArgMatches) {
    let desktop_name = matches.value_of("name").unwrap();
    Desktop::new_skeleton(desktop_name);
    completions::generate_completions()
}
