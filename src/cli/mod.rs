mod clilogger;
mod commands;
mod completions;
mod setup;
pub mod specification;
mod utils;

use clap::ArgMatches;

pub fn start_cli(matches: ArgMatches) {
    utils::init_logger(&matches);
    commands::handle_command(&matches);
}
