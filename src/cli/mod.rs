pub mod clilogger;
pub mod commands;
pub mod setup;
pub mod completions;

use std::collections::{HashMap, BTreeSet};
use std::env;
use clap::{ArgMatches, Values};
use log::{LevelFilter, error, warn, info, Level};
use colored::*;
use term_grid::{Grid, GridOptions, Direction, Filling};
// use terminal_size::terminal_size;
use std::process::{Command, Stdio};
use std::{fs, path::Path};
use tint::Color as TintColor;

use clilogger::CliLogger;
use crate::app;
use crate::core::{
	self,
	desktop::{Desktop, DesktopFile},
	theme::{Theme, ThemeFile},
	pattern::{Pattern, PatternFile},
	postscript::PostScript,
	config::{GlobalConfig, DesktopConfig, DesktopInfo, UserConfig}
};

enum Action {
	Enable,
	Disable,
	Toggle
}

pub fn start_cli() {
	let matches = commands::Cli::new(&vec![],&vec![], 
	&vec![], &vec![], &vec![])
		.get_app().get_matches();

	if matches.subcommand() == None {
		app::Ui::new().start_ui();
		return
	}

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

	match matches.subcommand() {
		Some(("config", sub_matches)) => match sub_matches.subcommand() {
			Some(("setup", _)) => setup::start(),
			Some(("list", _)) => show_settings(),
			Some(("edit", _)) => edit_settings(),
			Some(("set", sub_sub_matches)) => set_settings_prop(sub_sub_matches),
			Some(("unset", sub_sub_matches)) => unset_settings_prop(sub_sub_matches),
			_ => ()
		},

		Some(("desktop", sub_matches)) => match sub_matches.subcommand() {
			Some(("info", sub_sub_matches)) => show_desktop_info(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => edit_desktop(sub_sub_matches),
			Some(("status", sub_sub_matches)) => show_status(sub_sub_matches),
			Some(("new-skeleton", sub_sub_matches)) => create_desktop(sub_sub_matches),
			Some(("add", sub_sub_matches)) => add_desktop(sub_sub_matches),
			Some(("remove", sub_sub_matches)) => remove_desktop(sub_sub_matches),
			Some(("set-default-theme", sub_sub_matches)) => set_default_theme(sub_sub_matches),
			Some(("list", _)) => list_desktops(),
			Some(("apply", sub_sub_matches)) => apply_desktop(sub_sub_matches),
			_ => ()
		}

		Some(("theme", sub_matches)) => match sub_matches.subcommand() {
			Some(("colors", sub_sub_matches)) => show_colors(sub_sub_matches),
			Some(("list", sub_sub_matches)) => list_themes(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => edit_theme(sub_sub_matches),
			Some(("apply", sub_sub_matches)) => apply_theme(sub_sub_matches),
			_ => ()
		}

		Some(("pattern", sub_matches)) => match sub_matches.subcommand() {
			Some(("list", sub_sub_matches)) => list_patterns(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => edit_pattern(sub_sub_matches),
			Some(("enable", sub_sub_matches)) => manage_patterns(sub_sub_matches, Action::Enable),
			Some(("disable", sub_sub_matches)) => manage_patterns(sub_sub_matches, Action::Disable),
			Some(("toggle", sub_sub_matches)) => manage_patterns(sub_sub_matches, Action::Toggle),
			Some(("invert", sub_sub_matches)) => toggle_invert(sub_sub_matches),
			_ => ()
		}

		Some(("extra", sub_matches)) => match sub_matches.subcommand() {
			Some(("list", sub_sub_matches)) => list_extras(sub_sub_matches),
			Some(("edit", sub_sub_matches)) => edit_extra(sub_sub_matches),
			Some(("enable", sub_sub_matches)) => manage_extras(sub_sub_matches, Action::Enable),
			Some(("disable", sub_sub_matches)) => manage_extras(sub_sub_matches, Action::Disable),
			Some(("toggle", sub_sub_matches)) => manage_extras(sub_sub_matches, Action::Toggle),
			_ => ()
		}

		Some(("fav", sub_matches)) => match sub_matches.subcommand() {
			Some(("list", _)) => list_fav_themes(),
			Some(("add", sub_sub_matches)) => manage_fav(sub_sub_matches, Action::Enable),
			Some(("remove", sub_sub_matches)) => manage_fav(sub_sub_matches, Action::Disable),
			Some(("toggle", sub_sub_matches)) => manage_fav(sub_sub_matches, Action::Toggle),
			_ => ()
		}

		_ => ()
	}
}

// Aux functions
fn get_desktop(desktop_opt: Option<&str>) -> Option<DesktopFile> {
	match desktop_opt {
		Some(desktop_str) => {
			match Desktop::get_by_name(desktop_str) {
				Some(d) => Some(d),
				None => None
			}
		},
		None => {
			let global_config = GlobalConfig::new();
			match global_config.get_current_desktop() {
				Some(d) => Some(d.clone()),
				None => {
					error!("|There is no desktop installed!| Try with -d option instead");
					None
				}
			}
		}
	}
}

fn get_actived(values_opt: Option<Values>, current_desktop: &DesktopFile, desktop_config: &DesktopConfig) -> HashMap<String,bool> {
	let mut actived: HashMap<String,bool> = HashMap::new();
	match values_opt {
		Some(patterns) => {
			for p in patterns {
				match Pattern::get_by_name(current_desktop, p) {
					Some(_) => actived.insert(p.to_string(), true),
					None => continue
				};
			}
		},
		None => actived = desktop_config.get_actived().clone()
	}
	actived
}

fn get_inverted(values_opt: Option<Values>, current_desktop: &DesktopFile, desktop_config: &DesktopConfig) -> HashMap<String,bool> {
	let mut inverted: HashMap<String,bool> = HashMap::new();
	match values_opt {
		Some(patterns) => {
			for p in patterns {
				match Pattern::get_by_name(current_desktop, p) {
					Some(_) => inverted.insert(p.to_string(), true),
					None => continue
				};
			}
		},
		None => inverted = desktop_config.get_inverted().clone()
	}
	inverted
}

fn apply_theme(matches: &ArgMatches) {
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

	let actived = get_actived(
		matches.values_of("pattern"),
		current_desktop,
		&desktop_config
	);

	let inverted = get_inverted(
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

fn apply_desktop(matches: &ArgMatches) {
	let desktop_name = matches.value_of("desktop").unwrap();

	let current_desktop = match Desktop::get_by_name(desktop_name) {
		Some(d) => d,
		None => return
	};

	let mut global_config = GlobalConfig::new();
	let previous_desktop = match global_config.get_current_desktop() {
		Some(d) => Some(d.to_desktop()),
		None => None
	};

	let desktop_config = DesktopConfig::new(&current_desktop);

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
				None => {
					error!("There is no |default theme| specified in desktop |{}|. Try with -t option instead", current_desktop.get_name());
					return
				}
			}
		}
	};

	let actived = get_actived(
		matches.values_of("pattern"),
		&current_desktop,
		&desktop_config
	);

	let inverted = get_inverted(
		matches.values_of("invert"),
		&current_desktop,
		&desktop_config
	);

	let dry_run = matches.is_present("dry-run");

	if !dry_run {
		*global_config.get_mut_current_desktop() = Some(current_desktop.clone());
		*global_config.get_mut_current_theme() = Some(default_theme.clone());
		global_config.save();
		completions::generate_completions()
	}

	current_desktop.to_desktop().apply(&previous_desktop, &default_theme.to_theme(), &actived, &inverted, dry_run);
}

fn manage_patterns(matches: &ArgMatches, action:Action) {
	let current_desktop_file = match get_desktop( matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let mut desktop_config = DesktopConfig::new(&current_desktop_file);
	let patterns = matches.values_of("pattern").unwrap();
	for pattern_str in patterns {
		let pattern = match Pattern::get_by_name(&current_desktop_file,pattern_str) {
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
	let current_desktop_file = match get_desktop( matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};
	let mut desktop_config = DesktopConfig::new(&current_desktop_file);

	let patterns = matches.values_of("pattern").unwrap();
	for pattern_str in patterns {
		let pattern = match Pattern::get_by_name(&current_desktop_file,pattern_str) {
			Some(pattern) => pattern,
			None => continue
		};
		desktop_config.toggle_invert_pattern(&pattern);
	}
	desktop_config.save();
}

fn manage_extras(matches: &ArgMatches, action: Action) {
	let current_desktop_file = match get_desktop( matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let mut desktop_config = DesktopConfig::new(&current_desktop_file);

	let extras = matches.values_of("extra").unwrap();
	for extra_str in extras {
		let extra = match PostScript::get_extra_by_name(&current_desktop_file,extra_str) {
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
	global_config.save();
	completions::generate_completions()
}

fn show_status(matches: &ArgMatches) {
	println!("");
	let global_config = GlobalConfig::new();
	let desktop = match get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let desktop_config = DesktopConfig::new(&desktop);

	let default_theme_name = match desktop_config.get_default_theme() {
		Some(t) => t.get_name(),
		None => ""
	};

	println!("{}\n", desktop.get_name().to_uppercase().bold().underline().cyan());
	println!("{} {}", "• Default theme:".green().bold(), default_theme_name);

	if let Some(d) = global_config.get_current_desktop() {
		if d.get_name() == desktop.get_name() {
			let current_theme = match global_config.get_current_theme() {
				Some(t) => t.get_name(),
				None => ""
			};
			println!("{} {}", "• Current theme:".yellow().bold(), current_theme)
		}
	}
	println!("");

	list_patterns(matches);
	list_extras(matches);
}


fn list_desktops() {
	println!("");
	let all_desktops = Desktop::get_desktops();
	let global_config = GlobalConfig::new();
	let current_desktop = match global_config.get_current_desktop() {
		Some(d) => d.get_name(),
		None => ""
	};

	println!("{}\n", "DESKTOPS".bold().underline().cyan());

	for d in all_desktops {
		if d.get_name() == current_desktop {
			println!("{} {}", "•".green(), format!("{} (Active)", d.get_name()).bold().green());
		} else {
			println!("{} {}", "•".cyan(), d.get_name());
		};
	}
	println!("");
}

fn list_themes(matches: &ArgMatches) {
	if matches.is_present("favs") {
		list_fav_themes();
		return
	}

	println!("");
	let all_themes = Theme::get_themes();
	let global_config = GlobalConfig::new();
	let current_theme = match global_config.get_current_theme() {
		Some(t) => t.get_name(),
		None => ""
	};

	println!("{}\n", "THEMES".bold().underline().yellow());

	let print_themes: Vec<String> = all_themes.into_iter().map(|t| {
		if t.get_name() == current_theme {
			format!("{} {}", "•".green(), t.get_name())
		} else {
			format!("{} {}", "•".yellow(), t.get_name())
		}
	}).collect();

	let mut grid = Grid::new(GridOptions {
		filling: Filling::Spaces(2),
		direction: Direction::TopToBottom,
	});
	
	for s in print_themes {
		grid.add(s.into());
	}

	// let term_width: usize = match terminal_size() {
	// 	Some((width, _)) => width.0.into(),
	// 	None => return 
	// };
	
	println!("{}", grid.fit_into_columns(3));
}

fn list_fav_themes() {
	println!("");
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
	println!("");
	let desktop = match get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
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
				print!(" {}", "OFF".bold().red());
				Color::Red
			}
		};

		match inverted.get(p.get_name()) {
			Some(i) =>  if *i { print!(" {}", "(Inverted)".bold().color(color)) },
			None => ()
		}

		println!("");
		if matches.is_present("submodules") {
			list_pattern_submodules("  ".to_string(), p.to_pattern().get_submodules());
		}
	}
	println!("");
}

fn list_pattern_submodules(pre: String, submodules_opt: &Option<Vec<PatternFile>>) {
	if let Some(submodules) = submodules_opt {
		if submodules.len() == 0 { return }
		for s in submodules.iter().take(submodules.len()-1) {
			println!("{}{} {}", pre.magenta(), "├".magenta(), s.get_name());
			list_pattern_submodules(pre.clone() + "│ ", s.to_pattern().get_submodules());
		}
		let last = submodules.last().unwrap().to_pattern();
		println!("{}{} {}", pre.magenta(), "└".magenta(), last.get_name());
		list_pattern_submodules(pre.clone() + "  ", last.get_submodules());
	}
}

fn list_extras(matches: &ArgMatches) {
	println!("");
	let desktop = match get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let all_extras = PostScript::get_extras(&desktop);
	let desktop_config = DesktopConfig::new(&desktop);

	let enabled = desktop_config.get_actived();

	let desktop_title = format!("({})", desktop.get_name());

	println!("{} {}\n", "EXTRAS".bold().underline().red(), desktop_title.bold().cyan());

	for p in all_extras {
		print!("{} {:<20}", "•".red(), p.get_name());
		match enabled.get(p.get_name()) {
			Some(e) => if *e {
					print!(" {}\n", "ON".bold().green());
				} else {
					print!(" {}\n", "OFF".bold().red());
				},
			None => print!(" {}\n", "OFF".bold().red())
		}
	}
	println!("");
}

fn edit_file(path: &str) {
	match env::var("EDITOR") {
		Ok(value) => if value.is_empty() {
			warn!("Env var |$EDITOR| is empty, using |nano| instead")
		},
		Err(_) => warn!("Could not found env var |$EDITOR|, using |nano| instead")
	}
	
	info!("Editing |{}|...", path);

	match Command::new("sh")
	.arg("-c")
	.arg(format!("${{EDITOR:-nano}} {}", path))
	.stdin(Stdio::inherit())
	.stdout(Stdio::inherit())
	.output() {
		Ok(output) => {
			match output.status.success() {
				true => info!("File |{}| edited succesfully", path),
				false => error!("Could not edit |{}|, error: |{}|", path, String::from_utf8(output.stderr).unwrap())
			}
		},
		Err(e) => error!("Could not edit |{}|, error: |{}|", path, e)	
	}
}

fn explore_directory(path: &str) {
	match env::var("FILE_EXPLORER") {
		Ok(value) => if value.is_empty() {
			warn!("Env var |$FILE_EXPLORER| is empty, using |ranger| instead |(try exporting env var FILE_EXPLORER in your shell config)|")
		},
		Err(_) => warn!("Could not found env var |$FILE_EXPLORER|, using |ranger| instead |(try exporting env var FILE_EXPLORER in your shell config)|")
	}
	
	info!("Reading |{}|...", path);

	match Command::new("sh")
	.arg("-c")
	.arg(format!("${{FILE_EXPLORER:-ranger}} {}", path))
	.stdin(Stdio::inherit())
	.stdout(Stdio::inherit())
	.output() {
		Ok(output) => {
			match output.status.success() {
				true => info!("Directory |{}| readed succesfully", path),
				false => error!("Could not read |{}|, error: |{}|", path, String::from_utf8(output.stderr).unwrap())
			}
		},
		Err(e) => error!("Could not read |{}|, error: |{}|", path, e)	
	}
}

fn edit_theme(matches: &ArgMatches) {
	let theme = match Theme::get_by_name(matches.value_of("theme").unwrap()) {
		Some(t) => t,
		None => return
	};
	edit_file(theme.get_path());
}

fn edit_desktop(matches: &ArgMatches) {
	let desktop = match Desktop::get_by_name(matches.value_of("desktop").unwrap()) {
		Some(t) => t,
		None => return
	};
	explore_directory(desktop.get_path());
	completions::generate_completions()
}

fn edit_pattern(matches: &ArgMatches) {
	let desktop = match get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};
	let pattern = match Pattern::get_by_name(&desktop, matches.value_of("pattern").unwrap()) {
		Some(t) => t,
		None => return
	};

	if matches.is_present("postscript") {
		match PostScript::get_postscript_by_name(&desktop, pattern.get_name()) {
			Some(ps) => edit_file(ps.get_path()),
			None => error!("Pattern |{}| has no postscript", pattern.get_name())
		}
		return
	}

	if pattern.to_pattern().has_submodules() {
		explore_directory(pattern.get_path());
	} else {
		edit_file(pattern.get_path());
	}
}

fn edit_extra(matches: &ArgMatches) {
	let desktop = match get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};
	let extra = match PostScript::get_extra_by_name(&desktop, matches.value_of("extra").unwrap()) {
		Some(t) => t,
		None => return
	};
	edit_file(extra.get_path());
}

fn set_default_theme(matches: &ArgMatches) {
	let desktop = match get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let theme = match Theme::get_by_name(matches.value_of("theme").unwrap()) {
		Some(t) => t,
		None => return
	};

	let mut desktop_config = DesktopConfig::new(&desktop);
	desktop_config.set_default_theme(&theme);
	desktop_config.save()
}

fn create_desktop(matches: &ArgMatches) {
	let desktop_name = matches.value_of("name").unwrap();
	Desktop::new_skeleton(desktop_name);
	completions::generate_completions()
}

fn add_desktop(matches: &ArgMatches) {
	let desktops = matches.values_of("path").unwrap();
	for desktop in desktops {
		if let Err(desktop_opt) = Desktop::add(Path::new(desktop)) {
			if let Some(desktop_file) = desktop_opt {
				if matches.is_present("force") {
					desktop_file.remove();
					let _ = Desktop::add(Path::new(desktop));
				} else {
					error!("Desktop |{}| already exists", desktop_file.get_name())
				}
			} 
		}
	}
	completions::generate_completions()
}

fn remove_desktop(matches: &ArgMatches) {
	let desktops = matches.values_of("desktop").unwrap();

	for desktop in desktops{
		let desktop_file = match Desktop::get_by_name(desktop) {
			Some(d) => d,
			None => continue
		};
		desktop_file.remove();
	}
	completions::generate_completions()
}

fn show_desktop_info(matches: &ArgMatches) {
	let desktop = match get_desktop(matches.value_of("desktop")) {
		Some(d) => d,
		None => return
	};

	let desktop_info = DesktopInfo::new(&desktop);
	let dependencies = desktop_info.get_dependencies();

	if !matches.is_present("deps") {
		println!("");
		println!("{} {}", "Name:".green().bold(), desktop.get_name());
		println!("{} {}", "Author:".green().bold(), desktop_info.get_author());
		println!("{} {}", "Credits:".green().bold(), desktop_info.get_credits());
		println!("{} {}", "Description:".green().bold(), desktop_info.get_description());
		println!("{}", "Dependecies:".green().bold());

		for dep in dependencies {
			println!(" • {}", dep)
		}
		println!("");
	} else {
		for dep in dependencies { println!("{}", dep) }
	}
}

fn show_settings() {
	if !UserConfig::exists() {
		error!("|There is no global settings file|, run |gtheme config setup| first");
		return
	}
	let user_settings = UserConfig::new();

	let mut sorted_props = vec![];
	for p in user_settings.get_properties() {
		sorted_props.push(p)
	}
	sorted_props.sort_by(|(a,_),(b,_)| a.cmp(b));

	println!("\n{}\n", "GLOBAL SETTINGS".bold().underline().yellow());
	for (key, value) in sorted_props {
		println!("{} = '{}'", key.bold().green(), value)
	}
	println!("");
}

fn edit_settings() {
	if !UserConfig::exists() {
		error!("|There is no global settings file|, run |gtheme config setup| first");
		return
	}
	let user_settings = UserConfig::new();
	edit_file(&user_settings.get_path());
}

fn set_settings_prop(matches: &ArgMatches) {
	if !UserConfig::exists() {
		error!("|There is no global settings file|, run |gtheme config setup| first");
		return
	}

	let key = matches.value_of("key").unwrap();
	let value = matches.value_of("value").unwrap();

	let mut user_settings = UserConfig::new();
	user_settings.set_property(key, value);
	user_settings.save();
}

fn unset_settings_prop(matches: &ArgMatches) {
	if !UserConfig::exists() {
		error!("|There is no global settings file|, run |gtheme config setup| first");
		return
	}

	let key = matches.value_of("key").unwrap();

	let mut user_settings = UserConfig::new();
	user_settings.unset_property(key);
	user_settings.save();
}

fn show_colors(matches: &ArgMatches) {
	let theme_file = match matches.value_of("theme") {
		Some(t) => match Theme::get_by_name(t) {
			Some(t) => t,
			None => return
		},
		None => {
			let global_config = GlobalConfig::new();
			match global_config.get_current_theme() {
				Some(t) => t.clone(),
				None => {
					error!("|There is no theme installed!|, try specifing a theme");
					return
				}
			}
		}
	};

	let theme = theme_file.to_theme();
	let sorted_colors = theme.get_colors().into_iter().collect::<BTreeSet<_>>();

	println!("\n{} {}\n", "THEME".bold().underline().green(), theme.get_name().bold());
	for (color_key, color_value) in sorted_colors {
		let color_hex = format!("#{}", &color_value);
		let (r,g, b) = TintColor::from_hex(&color_hex).to_rgb255();
		println!("{color_hex}  {}  {}", "██".truecolor(r, g, b), color_key.bold().cyan())
	}
	println!();
}
