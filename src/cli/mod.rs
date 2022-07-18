pub mod clilogger;
pub mod specification;
pub mod setup;
pub mod completions;
pub mod commands;
pub mod utils;

use clap::ArgMatches;
use log::{LevelFilter, Level};
use std::{fs, path::Path};

use utils::Action;
use clilogger::CliLogger;
use crate::core;

fn init_logger(matches: &ArgMatches) {
	// Logger init
	let log_dir = Path::new(&core::expand_path(core::GTHEME_MISC)).join("logs");
	let _ = fs::create_dir_all(&log_dir);

	log::set_max_level(LevelFilter::Info);
	if matches.is_present("verbose") {
		static CLI_LOGGER: CliLogger = CliLogger{level: Level::Info};
		log::set_logger(&CLI_LOGGER).unwrap();
	} else {
		static CLI_LOGGER: CliLogger = CliLogger{level: Level::Warn};
		log::set_logger(&CLI_LOGGER).unwrap();
	}
}

pub fn start_cli(matches: ArgMatches) {
	init_logger(&matches);

	match matches.subcommand() {
		Some(("config", sub_matches)) => match sub_matches.subcommand() {
			Some(("show", _)) => commands::config::show::run(),
			Some(("setup", _)) => commands::config::setup::run(),
			Some(("edit", _)) => commands::config::edit::run(),
			Some(("set", sub_sub_matches)) => commands::config::set::run(sub_sub_matches),
			Some(("unset", sub_sub_matches)) => commands::config::unset::run(sub_sub_matches),
			_ => ()
		},

		Some(("desktop", sub_matches)) => match sub_matches.subcommand() {
			Some(("list",  sub_sub_matches)) => commands::desktop::list::run(sub_sub_matches),
			Some(("info", sub_sub_matches)) => commands::desktop::info::run(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => commands::desktop::edit::run(sub_sub_matches),
			Some(("status", sub_sub_matches)) => commands::desktop::status::run(sub_sub_matches),
			Some(("new-skeleton", sub_sub_matches)) => commands::desktop::newskeleton::run(sub_sub_matches),
			Some(("add", sub_sub_matches)) => commands::desktop::add::run(sub_sub_matches),
			Some(("remove", sub_sub_matches)) => commands::desktop::remove::run(sub_sub_matches),
			Some(("set-default-theme", sub_sub_matches)) => commands::desktop::setdefault::run(sub_sub_matches),
			Some(("apply", sub_sub_matches)) => commands::desktop::apply::run(sub_sub_matches),
			_ => ()
		}

		Some(("theme", sub_matches)) => match sub_matches.subcommand() {
			Some(("list", sub_sub_matches)) => commands::theme::list::run(sub_sub_matches),
			Some(("colors", sub_sub_matches)) => commands::theme::colors::run(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => commands::theme::edit::run(sub_sub_matches),
			Some(("new-skeleton", sub_sub_matches)) => commands::theme::newskeleton::run(sub_sub_matches),
			Some(("remove", sub_sub_matches)) => commands::theme::remove::run(sub_sub_matches),
			Some(("apply", sub_sub_matches)) => commands::theme::apply::run(sub_sub_matches),
			_ => ()
		}

		Some(("pattern", sub_matches)) => match sub_matches.subcommand() {
			Some(("list", sub_sub_matches)) => commands::pattern::list::run(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => commands::pattern::edit::run(sub_sub_matches),
			Some(("enable", sub_sub_matches)) => commands::pattern::manage::run(sub_sub_matches, Action::Enable),
			Some(("disable", sub_sub_matches)) => commands::pattern::manage::run(sub_sub_matches, Action::Disable),
			Some(("toggle", sub_sub_matches)) => commands::pattern::manage::run(sub_sub_matches, Action::Toggle),
			Some(("invert", sub_sub_matches)) => commands::pattern::invert::run(sub_sub_matches),
			_ => ()
		}

		Some(("extra", sub_matches)) => match sub_matches.subcommand() {
			Some(("list", sub_sub_matches)) => commands::extra::list::run(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => commands::extra::edit::run(sub_sub_matches),
			Some(("enable", sub_sub_matches)) => commands::extra::manage::run(sub_sub_matches, Action::Enable),
			Some(("disable", sub_sub_matches)) => commands::extra::manage::run(sub_sub_matches, Action::Disable),
			Some(("toggle", sub_sub_matches)) => commands::extra::manage::run(sub_sub_matches, Action::Toggle),
			_ => ()
		}

		Some(("fav", sub_matches)) => match sub_matches.subcommand() {
			Some(("list", sub_sub_matches)) => commands::fav::list::run(sub_sub_matches),
			Some(("add", sub_sub_matches)) => commands::fav::manage::run(sub_sub_matches, Action::Enable),
			Some(("remove", sub_sub_matches)) => commands::fav::manage::run(sub_sub_matches, Action::Disable),
			Some(("toggle", sub_sub_matches)) => commands::fav::manage::run(sub_sub_matches, Action::Toggle),
			_ => ()
		}

		_ => ()
	}
}
