pub mod clilogger;
pub mod commands;
pub mod completions;
pub mod setup;
pub mod specification;
pub mod utils;

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

    match matches.subcommand() {
        Some(("config", sub_matches)) => commands::config::handle_subcommands(sub_matches),
        Some(("desktop", sub_matches)) => commands::desktop::hanlde_subcommands(sub_matches),
        Some(("theme", sub_matches)) => commands::theme::handle_subcommands(sub_matches),
        Some(("pattern", sub_matches)) => commands::pattern::handle_subcommands(sub_matches),
        Some(("extra", sub_matches)) => commands::extra::handle_subcommands(sub_matches),
        Some(("fav", sub_matches)) => commands::fav::handle_subcommands(sub_matches),
        _ => (),
    }
}
