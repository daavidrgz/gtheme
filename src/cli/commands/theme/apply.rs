use clap::ArgMatches;
use log::error;

use crate::cli::utils;
use crate::core::{
	theme::Theme,
	config::{DesktopConfig, GlobalConfig}
};

pub fn run(matches: &ArgMatches) {
	let theme_name = matches.value_of("theme").unwrap();

	let theme = match Theme::get_by_name(theme_name) {
		Some(t) => t,
		None => return
	};

	let mut global_config = GlobalConfig::new();

	let current_desktop = match global_config.get_current_desktop() {
		Some(d) => d,
		None => {
			error!("|There is no desktop installed!|");
			return
		}
	};
	let desktop_config = DesktopConfig::new(current_desktop);

	let actived = utils::get_actived(
		matches.values_of("pattern"),
		current_desktop,
		&desktop_config
	);

	let inverted = utils::get_inverted(
		matches.values_of("invert"),
		current_desktop,
		&desktop_config
	);

	let dry_run = matches.is_present("dry-run");

	current_desktop.to_desktop().apply_theme(&theme.to_theme(), &actived, &inverted, dry_run);

	if !dry_run && !matches.is_present("pattern") {
		*global_config.get_mut_current_theme() = Some(theme);
		global_config.save()
	}
}