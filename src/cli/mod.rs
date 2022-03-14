pub mod clilogger;

use std::collections::HashMap;
use clap::{Command, Arg, ArgMatches};
use log::{LevelFilter, error, warn};
use colored::*;

use clilogger::CliLogger;
use crate::app;
use crate::core::{
	desktop::Desktop,
	theme::{Theme, ThemeFile},
	pattern::Pattern,
	postscript::PostScript,
	config::{GlobalConfig, DesktopConfig}
};

const DEFAULT_THEME: &str = "Nord";
enum Action{
	Enable,
	Disable,
	Toggle
}
pub struct Cli<'a> {
	app: Command<'a>,
}

impl<'a> Cli<'a> {
	pub fn new() -> Self {
		let mut app = Command::new("gtheme")
		.version("1.0")
		.about("A rust program that makes your theming life so much easier.")
		.author("David Rodríguez & Jorge Hermo")
		.arg(Arg::new("verbose")
			.short('v')
			.long("verbose")
			.global(true)
			.help("Show more information")
		);

		app = app.subcommand(Command::new("theme")
			.about("Manage themes")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.about("List all installed themes")
			)
			.subcommand(Command::new("apply")
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
			)
		);

		app = app.subcommand(Command::new("desktop")
			.about("Manage desktops")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.about("List all installed desktops")
			)
			.subcommand(Command::new("install")
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
			)
		);

		app = app.subcommand(Command::new("pattern")
			.about("Manage patterns")
			.subcommand_required(true)
			.subcommand(Command::new("list")
			.about("List all patterns of the current desktop by default")
				.arg(Arg::new("desktop")
					.short('d')
					.long("desktop")
					.takes_value(true)
					.help("List patterns of the specified desktop")
				)
			)
			.subcommand(Command::new("enable")
				.about("Enable specified patterns in the current desktop")
				.arg(Arg::new("pattern")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.help("Patterns to enable")
				)
			)
			.subcommand(Command::new("disable")
				.about("Disable specified patterns in the current desktop")
				.arg(Arg::new("pattern")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.help("Patterns to disable")
				)
			)
			.subcommand(Command::new("toggle")
				.about("Toggle specified patterns in the current desktop")
				.arg(Arg::new("pattern")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.help("Patterns to toggle")
				)
			)
			.subcommand(Command::new("invert")
				.about("Invert specified patterns or return them to default if they are already inverted")
				.arg(Arg::new("pattern")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.help("Patterns to invert")
				)
			)
		);

		app = app.subcommand(Command::new("extra")
			.about("Manage extras")
			.subcommand_required(true)
			.subcommand(Command::new("enable")
				.about("Enable specified extras in the current desktop")
				.arg(Arg::new("extra")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.help("Extras to enable")
				)
			)
			.subcommand(Command::new("disable")
				.about("Disable specified extras in the current desktop")
				.arg(Arg::new("extra")
					.required(true)
					.takes_value(true)
					.help("Extras to disable")
				)
			)
			.subcommand(Command::new("toggle")
			.about("Toggle specified extras in the current desktop")
			.arg(Arg::new("extra")
				.required(true)
				.takes_value(true)
				.help("Extras to toggle")
			)
		)
		);

		app = app.subcommand(Command::new("fav")
			.about("Manage fav themes")
			.subcommand_required(true)
			.subcommand(Command::new("list")
			.about("List favourite themes")
			)
			.subcommand(Command::new("add")
				.about("Add selected themes to the favourite themes list")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.help("Themes to add")
				)
			)
			.subcommand(Command::new("remove")
				.about("Remove selected themes from the favourite themes list")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.help("Themes to remove")
				)
			).subcommand(Command::new("toggle")
			.about("Toggle selected themes from the favourite themes list")
			.arg(Arg::new("theme")
				.required(true)
				.takes_value(true)
				.multiple_values(true)
				.help("Themes to toggle")
			)
		)
		);

		Cli { app }
	}

	pub fn start_cli(self) {
		let matches = self.app.get_matches();

		if matches.subcommand() == None {
			app::Ui::new().start_ui();
			return
		}

		// Logger init
		static CLI_LOGGER: CliLogger = CliLogger;

		if matches.is_present("verbose") {
			log::set_max_level(LevelFilter::Info);
		} else {
			log::set_max_level(LevelFilter::Warn);
		}
    log::set_logger(&CLI_LOGGER).unwrap();

		println!("");
		match matches.subcommand() {

			Some(("desktop", sub_matches)) => match sub_matches.subcommand() {
				Some(("list", _)) => Self::list_desktops(),
				Some(("apply", sub_sub_matches)) => Self::install_desktop(sub_sub_matches),
				_ => ()
			}

			Some(("theme", sub_matches)) => match sub_matches.subcommand() {
				Some(("list", _)) => Self::list_themes(),
				Some(("apply", sub_sub_matches)) => Self::apply_theme(sub_sub_matches),
				_ => ()
			}

			Some(("pattern", sub_matches)) => match sub_matches.subcommand() {
				Some(("list", sub_sub_matches)) => Self::list_patterns(sub_sub_matches),
				Some(("enable", sub_sub_matches)) => Self::manage_patterns(sub_sub_matches, Action::Enable),
				Some(("disable", sub_sub_matches)) => Self::manage_patterns(sub_sub_matches, Action::Disable),
				Some(("toggle", sub_sub_matches)) => Self::manage_patterns(sub_sub_matches, Action::Toggle),
				Some(("invert", sub_sub_matches)) => Self::toggle_invert(sub_sub_matches),
				_ => ()
			}

			Some(("extra", sub_matches)) => match sub_matches.subcommand() {
				Some(("enable", sub_sub_matches)) => Self::manage_extras(sub_sub_matches, Action::Enable),
				Some(("disable", sub_sub_matches)) => Self::manage_extras(sub_sub_matches, Action::Disable),
				Some(("toggle", sub_sub_matches)) => Self::manage_extras(sub_sub_matches, Action::Toggle),
				_ => ()
			}

			Some(("fav", sub_matches)) => match sub_matches.subcommand() {
				Some(("list", _)) => Self::list_fav_themes(),
				Some(("add", sub_sub_matches)) => Self::manage_fav(sub_sub_matches, Action::Enable),
				Some(("remove", sub_sub_matches)) => Self::manage_fav(sub_sub_matches, Action::Disable),
				Some(("toggle", sub_sub_matches)) => Self::manage_fav(sub_sub_matches, Action::Toggle),
				_ => ()
			}

			_ => ()
		}
	}

	fn apply_theme(matches: &ArgMatches) {
		let theme_name = matches.value_of("theme").unwrap();

		let theme = match Theme::get_by_name(theme_name) {
			Some(t) => t,
			None => return
		};

		let mut global_config = GlobalConfig::new();

		let current_desktop_file = match global_config.get_current_desktop() {
			Some(d) => d,
			None => {
				error!("|There is no desktop installed!|");
				return
			}
		};
		let current_desktop = current_desktop_file.to_desktop();
		let desktop_config = DesktopConfig::new(current_desktop_file);

		let mut actived: HashMap<String,bool> = HashMap::new();
		if matches.is_present("pattern") {
			let patterns = matches.values_of("pattern").unwrap();
			for p in patterns {
				match Pattern::get_by_name(current_desktop_file, p) {
					Some(_) => actived.insert(p.to_string(), true),
					None => continue
				};
			}
		} else {
			actived = desktop_config.get_actived().clone()
		}

		let mut inverted: HashMap<String,bool> = HashMap::new();
		if matches.is_present("invert") {
			let patterns = matches.values_of("invert").unwrap();
			for p in patterns {
				match Pattern::get_by_name(current_desktop_file, p) {
					Some(_) => inverted.insert(p.to_string(), true),
					None => continue
				};
			}
		} else {
			inverted = desktop_config.get_inverted().clone()
		}

		current_desktop.apply(&theme.to_theme(), &actived, &inverted);

		*global_config.get_mut_current_theme() = Some(theme);
		global_config.save()
	}

	fn install_desktop(matches: &ArgMatches) {
		let desktop_name = matches.value_of("desktop").unwrap();

		let desktop = match Desktop::get_by_name(desktop_name) {
			Some(d) => d,
			None => return
		};

		let mut global_config = GlobalConfig::new();
		let previous = match global_config.get_current_desktop() {
			Some(d) => Some(d.to_desktop()),
			None => None
		};

		let desktop_config = DesktopConfig::new(&desktop);

		let default_theme: ThemeFile = match matches.value_of("theme") {
			Some(theme_name) => {
				match Theme::get_by_name(theme_name) {
					Some(t) => t,
					None => return
				}
			},
			None => {
				match desktop_config.get_default_theme() {
					Some(t) => t.clone(),
					None => Theme::get_by_name(DEFAULT_THEME).unwrap()
				}
			}
		};

		*global_config.get_mut_current_desktop() = Some(desktop.clone());
		*global_config.get_mut_current_theme() = Some(default_theme.clone());
		global_config.save();

		desktop.to_desktop().install(&previous, &default_theme.to_theme(), desktop_config.get_actived(), desktop_config.get_inverted())
	}

	fn manage_patterns(matches: &ArgMatches, action:Action) {
		let global_config = GlobalConfig::new();
		let current_desktop_file = match global_config.get_current_desktop() {
			Some(d) => d,
			None => {
				error!("|There is no desktop installed|, try using -d option");
				return
			}
		};

		let mut desktop_config = DesktopConfig::new(current_desktop_file);
		let patterns = matches.values_of("pattern").unwrap();
		for pattern_str in patterns {
			let pattern = match Pattern::get_by_name(current_desktop_file,pattern_str) {
				Some(pattern) => pattern,
				None => continue
			};
			match action {
				Action::Enable => desktop_config.enable_pattern(&pattern),
				Action::Disable => desktop_config.disable_pattern(&pattern),
				Action::Toggle => desktop_config.toggle_pattern(&pattern)
			}
		}
		desktop_config.save();
	}

	fn toggle_invert(matches : &ArgMatches) {
		let global_config = GlobalConfig::new();
		let current_desktop_file = match global_config.get_current_desktop() {
			Some(d) => d,
			None => {
				error!("|There is no desktop installed|, try using -d option");
				return
			}
		};
		let mut desktop_config = DesktopConfig::new(current_desktop_file);

		let patterns = matches.values_of("pattern").unwrap();
		for pattern_str in patterns {
			let pattern = match Pattern::get_by_name(current_desktop_file,pattern_str) {
				Some(pattern) => pattern,
				None => continue
			};
			desktop_config.toggle_invert_pattern(&pattern);
		}
		desktop_config.save();
	}

	fn manage_extras(matches: &ArgMatches, action: Action) {
		let global_config = GlobalConfig::new();
		let current_desktop_file = match global_config.get_current_desktop() {
			Some(d) => d,
			None => {
				error!("|There is no desktop installed|, try using -d option");
				return
			}
		};

		let mut desktop_config = DesktopConfig::new(current_desktop_file);

		let extras = matches.values_of("extra").unwrap();
		for extra_str in extras {
			let extra = match PostScript::get_extra_by_name(current_desktop_file,extra_str) {
				Some(pattern) => pattern,
				None => continue
			};
			match action {
				Action::Enable => desktop_config.enable_extra(&extra),
				Action::Disable => desktop_config.disable_extra(&extra),
				Action::Toggle => desktop_config.toggle_extra(&extra),
			}
		}

		desktop_config.save();
	}

	fn manage_fav(matches: &ArgMatches, action: Action) {
		let mut global_config = GlobalConfig::new();

		let themes = matches.values_of("theme").unwrap();
		for theme_name in themes {
			let theme = match Theme::get_by_name(theme_name){
				Some(t) => t,
				None => continue
			};
			match action {
				Action::Enable => global_config.add_fav_theme(&theme),
				Action::Disable => global_config.remove_fav_theme(&theme),
				Action::Toggle => global_config.toggle_fav_theme(&theme)
			}
		}
		global_config.save()
	}


	fn list_desktops() {
		let all_desktops = Desktop::get_desktops();
		let global_config = GlobalConfig::new();
		let current_desktop = match global_config.get_current_desktop() {
			Some(d) => d.get_name(),
			None => ""
		};

		println!("{}\n", "DESKTOPS".bold().underline().cyan());

		for d in all_desktops {
			if d.get_name() == current_desktop {
				println!("{} {}", "•".green(), d.get_name().bold().green());
			} else {
				println!("{} {}", "•".cyan(), d.get_name());
			};
		}
		println!("");
	}

	fn list_themes() {
		let all_themes = Theme::get_themes();
		let global_config = GlobalConfig::new();
		let current_theme = match global_config.get_current_theme() {
			Some(t) => t.get_name(),
			None => ""
		};

		println!("{}\n", "THEMES".bold().underline().yellow());

		for t in all_themes {
			if t.get_name() == current_theme {
				println!("{} {}", "•".green(), t.get_name().bold().green());
			} else {
				println!("{} {}", "•".yellow(), t.get_name());
			};
		}
		println!("");
	}

	fn list_fav_themes() {
		let global_config = GlobalConfig::new();
		let current_theme = match global_config.get_current_theme() {
			Some(t) => t.get_name(),
			None => ""
		};

		let fav_themes = global_config.get_fav_themes();

		println!("{}\n", "FAV THEMES".bold().underline().blue());

		for t in fav_themes {
			if t.get_name() == current_theme {
				println!("{} {}", "•".green(), t.get_name().bold().green());
			} else {
				println!("{} {}", "•".blue(), t.get_name());
			};
		}
		println!("");
	}

	fn list_patterns(matches: &ArgMatches) {
		let desktop = match matches.value_of("desktop") {
			Some(desktop_str) => {
				match Desktop::get_by_name(desktop_str) {
					Some(d) => d,
					None => return
				}
			},
			None => {
				let global_config = GlobalConfig::new();
				match global_config.get_current_desktop() {
					Some(d) => d.clone(),
					None => {
						warn!("|There is no desktop installed!| Try with -d option instead");
						return
					}
				}
			}
		};

		let all_patterns = Pattern::get_patterns(&desktop);
		let desktop_config = DesktopConfig::new(&desktop);

		let enabled = desktop_config.get_actived();
		let inverted = desktop_config.get_inverted();

		let desktop_title = format!("({})", desktop.get_name());

		println!("{} {}\n", "PATTERNS".bold().underline().magenta(), desktop_title.bold().cyan());

		for p in all_patterns {
			print!("{} {:<20}", "•".magenta(), p.get_name());
			let color = match enabled.get(p.get_name()) {
				Some(e) => if *e {
						print!(" {}", "ON".bold().green());
						Color::Green
					} else {
						print!(" {}", "OFF".bold().red());
						Color::Red
					},
				None => {
					print!(" {}", "ON".bold().green());
					Color::Green
				}
			};

			match inverted.get(p.get_name()) {
				Some(i) =>  if *i { print!(" {}", "(Inverted)".bold().color(color)) },
				None => ()
			}
			println!("");
		}
		println!("");
	}
}
