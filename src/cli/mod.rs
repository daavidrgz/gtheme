mod clilogger;
mod commands;
mod completions;
mod setup;
pub mod specification;

use clap::ArgMatches;

use self::clilogger::CliLogger;

pub fn start_cli(matches: ArgMatches) {
    CliLogger::init_logger(matches.occurrences_of("verbose"));
    commands::handle_command(&matches);
}
