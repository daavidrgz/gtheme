use std::{fs, path::Path};

use clap::ArgMatches;
use log::{Level, LevelFilter};

use crate::cli::clilogger::CliLogger;
use crate::core;

pub fn init_logger(matches: &ArgMatches) {
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
