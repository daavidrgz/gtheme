

mod clilogger;
mod commands;
mod completions;
mod setup;
pub mod specification;
mod utils;

use clap::ArgMatches;
use log::{Level, LevelFilter};
use std::{fs, path::Path};

use crate::core;
use clilogger::CliLogger;

fn init_logger(matches: &ArgMatches) {
    // Logger init
    let log_dir = Path::new(&core::expand_path(core::GTHEME_MISC)).join("logs");
    let _ = fs::create_dir_all(&log_dir);

    log::set_max_level(LevelFilter::Info);
    if matches.is_present("verbose") {
        static CLI_LOGGER: CliLogger = CliLogger { level: Level::Info };
        log::set_logger(&CLI_LOGGER).unwrap();
    } else {
        static CLI_LOGGER: CliLogger = CliLogger { level: Level::Warn };
        log::set_logger(&CLI_LOGGER).unwrap();
    }
}

pub fn start_cli(matches: ArgMatches) {
    init_logger(&matches);
    commands::handle_command(&matches);
}
