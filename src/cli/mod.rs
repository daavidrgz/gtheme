use clap::{Command, Arg, ArgMatches};
use std::collections::HashMap;

use crate::app;
use crate::core::{
	desktop::DesktopFile,
	theme::{ThemeFile, Theme},
	pattern::{PatternFile, Pattern},
	postscript::PostScript,
	config::{GlobalConfig, DesktopConfig}
};

pub struct Cli<'a> {
	app: Command<'a>,
}

impl<'a> Cli<'a> {
	pub fn new() -> Self {
		let mut app = Command::new("gtheme")
		.version("1.0")
		.about("A rust program that makes your theming life so much easier.")
		.author("David Rodríguez & Jorge Hermo");

		app = app.subcommand(Command::new("apply")
			.about("Apply specified theme")
			.args([
				Arg::new("theme")
					.required(true)
					.takes_value(true)
					.help("Theme to apply on all active patterns by default"),
				
				Arg::new("pattern")
					.short('p')
					.long("pattern")
					.takes_value(true)
					.multiple_values(true)
					.value_name("pattern")
					.help("Apply the theme only on selected patterns"),

				Arg::new("invert")
					.short('i')
					.long("invert")
					.takes_value(true)
					.multiple_values(true)
					.value_name("pattern")
					.help("Invert the foreground and background colors on selected patterns"),
			])
		);

		app = app.subcommand(Command::new("install")
			.about("Install specified desktop")
			.args([
				Arg::new("desktop")
					.required(true)
					.takes_value(true)
					.help("Desktop to install"),

				Arg::new("theme")
					.short('t')
					.long("theme")
					.takes_value(true)
					.help("Apply specified theme after installing the desktop")
			])
		);

		app = app.subcommand(Command::new("list")
			.about("List all installed themes, patterns, favourite themes or desktops")
			.args([
				Arg::new("element")
				.required(true)
				.takes_value(true)
				.exclusive(true)
				.possible_values(["themes","desktops","patterns","favs"])
			])
		);

		app = app.subcommand(Command::new("enable")
			.about("Enable specified patterns in the current desktop")
			.args([
				Arg::new("pattern")
				.required(true)
				.takes_value(true)
				.multiple_values(true)
				.exclusive(true)
				.help("Patterns to enable")
			])
		);

		app = app.subcommand(Command::new("disable")
			.about("Disable specified patterns in the current desktop")
			.args([
				Arg::new("pattern")
				.required(true)
				.takes_value(true)
				.multiple_values(true)
				.exclusive(true)
				.help("Patterns to disable")
			])
		);

		app = app.subcommand(Command::new("fav")
			.about("Add or remove selected themes from the favourite themes list")
			.subcommand_required(true)
			.subcommand(Command::new("add")
				.about("Add selected themes to the favourite themes list")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.exclusive(true)
					.help("Themes to add")
				)
			)
			.subcommand(Command::new("remove")
				.about("Remove selected themes to the favourite themes list")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.exclusive(true)
					.help("Themes to remove")
				)
			)
		);

		Cli { app }
	}

	pub fn start_cli(mut self) {
		let matches = self.app.get_matches();

		match matches.subcommand() {
			Some(("apply", sub_matches)) => Self::apply_theme(sub_matches),
			Some(("install", sub_matches)) => Self::install_desktop(sub_matches),
			Some(_) => (),
			None => {app::Ui::new().start_ui()}
		}
	}

	fn apply_theme(matches: &ArgMatches) {
		let theme_name = matches.value_of("theme").unwrap();

		let themes = Theme::get_themes();
		let theme = match themes.into_iter().find(|t| t.get_name().to_lowercase() == theme_name.to_lowercase()) {
			Some(t) => t.to_theme(),
			None => {
				println!("[!] Error, the theme {} does not exist!", theme_name);
				return
			}
		};

		let global_config = GlobalConfig::new();
		let current_desktop = match global_config.get_current_desktop() {
			Some(d) => d.to_desktop(),
			None => {
				println!("[!] Error, there is no desktop installed!");
				return
			}
		};
		
		let actived = if matches.is_present("pattern") {
			let mut all_patterns = Pattern::get_patterns(current_desktop.get_name());
			let patterns = matches.values_of("pattern").unwrap();
			for p in patterns {
				let size = all_patterns.len();
				all_patterns = all_patterns.into_iter().filter(|e| e.get_name() != p).collect();
				if size == all_patterns.len() {
					println!("[!] Error, the pattern {} does not exist!", p);
					return
				}
			}

			let mut map: HashMap<String,bool> = HashMap::new();
			all_patterns.into_iter().for_each(|p| {map.insert(p.get_name().clone(), false);});
			map
		} else {
			let desktop_config = DesktopConfig::new(current_desktop.get_name());
			desktop_config.get_actived().clone()
		};

		let inverted = if matches.is_present("invert") {
			let mut map: HashMap<String,bool> = HashMap::new();
			let all_patterns = Pattern::get_patterns(current_desktop.get_name());
			let patterns = matches.values_of("invert").unwrap();
			for p in patterns {
				match all_patterns.iter().find(|e| e.get_name() == p) {
					Some(_) => map.insert(p.to_string(), true),
					None => {
						println!("[!] Error, the pattern {} does not exist!", p);
						return
					}
				};
			}
			map
		} else {
			// let desktop_config = DesktopConfig::new(current_desktop.get_name());
			// desktop_config.get_inverted().clone()
			HashMap::new()
		};

		current_desktop.apply(&theme, &actived, &inverted)
	}

	fn install_desktop(matches: &ArgMatches) {

	}
}
