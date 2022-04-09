use clap::*;
use clap_complete::{generate_to, shells::Shell};
use std::{fs, io::Result, path::PathBuf};
use log::error;
use crate::core::{self, 
	theme::Theme,
	desktop::Desktop,
	pattern::Pattern,
	postscript::PostScript,
	config::GlobalConfig
};

pub fn get_themes() -> Vec<String> {
	let themes =  Theme::get_themes();
	let themes = themes.into_iter().map(|t| t.get_name().to_string().to_lowercase());
	let themes = themes.map(|s| s.replace("(","\\(").replace(")","\\)")).collect();
	themes
}

pub fn get_fav_themes(global_config: &GlobalConfig) -> Vec<String> {
	let fav_themes =  global_config.get_fav_themes();
	let fav_themes = fav_themes.into_iter().map(|t| t.get_name().to_string().to_lowercase());
	let fav_themes = fav_themes.map(|s| s.replace("(","\\(").replace(")","\\)")).collect();
	fav_themes
}

pub fn get_desktops() -> Vec<String> {
	let desktops =  Desktop::get_desktops();
	let desktops = desktops.into_iter().map(|d| d.get_name().to_string().to_lowercase());
	let desktops = desktops.map(|s| s.replace("(","\\(").replace(")","\\)")).collect();
	desktops
}

pub fn get_patterns(global_config: &GlobalConfig) -> Vec<String> {
	let desktop = match global_config.get_current_desktop() {
		None => return vec![],
		Some(desktop) => desktop
	};
	let patterns =  Pattern::get_patterns(desktop);
	let patterns = patterns.into_iter().map(|p| p.get_name().to_string().to_lowercase());
	let patterns = patterns.map(|s| s.replace("(","\\(").replace(")","\\)")).collect();
	patterns
}

pub fn get_extras(global_config: &GlobalConfig) -> Vec<String> {
	let desktop = match global_config.get_current_desktop() {
		None => return vec![],
		Some(desktop) => desktop
	};
	let extras =  PostScript::get_extras(desktop);
	let extras = extras.into_iter().map(|p| p.get_name().to_string().to_lowercase());
	let extras = extras.map(|s| s.replace("(","\\(").replace(")","\\)")).collect();
	extras
}

pub fn generate_completions() {
	let completions_dir = std::path::Path::new(&core::expand_path(core::GTHEME_HOME)).join("completions");
	let global_config = GlobalConfig::new();

	let themes_owned = get_themes();
	let desktops_owned = get_desktops();
	let patterns_owned = get_patterns(&global_config);
	let fav_themes_owned = get_fav_themes(&global_config);
	let extras_owned = get_extras(&global_config);

	let themes: Vec<&str> = themes_owned.iter().map(|s| s.as_str()).collect();
	let desktops: Vec<&str> = desktops_owned.iter().map(|s| s.as_str()).collect();
	let patterns: Vec<&str> = patterns_owned.iter().map(|s| s.as_str()).collect();
	let fav_themes: Vec<&str> = fav_themes_owned.iter().map(|s| s.as_str()).collect();
	let extras: Vec<&str> = extras_owned.iter().map(|s| s.as_str()).collect();

	let mut app = Cli::new(&themes, &desktops, &patterns, &fav_themes, &extras).get_app();
	
	// Generate completions
	if let Err(e) = fs::create_dir_all(&completions_dir) {
		error!("Error while creating completions directory: |{e}|");
		return
	}
	if let Err(e) = generate_completions_files(&mut app, &completions_dir) {
		error!("Error while generating completion scripts: |{e}|");
		return
	}
}

pub fn generate_completions_files(app: &mut Command, completions_dir: &PathBuf) -> Result<()> {
	generate_to(Shell::Bash, app, "gtheme", &completions_dir)?;
	generate_to(Shell::Zsh, app, "gtheme", &completions_dir)?;
	generate_to(Shell::Fish, app, "gtheme", &completions_dir)?;
	generate_to(Shell::PowerShell, app, "gtheme", &completions_dir)?;
	generate_to(Shell::Elvish, app, "gtheme", &completions_dir)?;
	Ok(())
}

pub struct Cli<'a>{
	app: Command<'a>,
}

impl <'a> Cli<'a> {
	pub fn get_app(self) -> Command<'a>{
		self.app
	}
	pub fn new(themes: &'a [&'a str], desktops: &'a [&'a str], patterns: &'a [&'a str], 
		fav_themes: &'a [&'a str], extras: &'a [&'a str]) -> Self {
		let mut app = Command::new("gtheme")
			.version("1.0")
			.about("A rust program that makes your theming life so much easier.")
			.author("David Rodriguez & Jorge Hermo")
			.arg(Arg::new("verbose")
				.short('v')
				.long("verbose")
				.global(true)
				.help("Show more information")
			);

		app = app.subcommand(Command::new("config")
			.alias("c")
			.about("Manage user settings")
			.subcommand_required(true)
			.subcommand(Command::new("setup")
				.about("Run an interactive setup to configure user settings")
			)
			.subcommand(Command::new("list")
				.alias("l")
				.about("Show current global settings")
			)
			.subcommand(Command::new("edit")
				.alias("ed")
				.about("Edit global settings")
			)
			.subcommand(Command::new("set")
				.alias("s")
				.about("Insert the specified key-value pair in the user settings file or update the value if the key is already in")
				.args([
					Arg::new("key")
						.required(true)
						.takes_value(true)
						.help("Attribute key"),
					Arg::new("value")
						.required(true)
						.takes_value(true)
						.help("Attribute value")
				])
			)
			.subcommand(Command::new("unset")
				.alias("u")
				.about("Remove the specified attribute from user settings file")
				.arg(Arg::new("key")
					.required(true)
					.takes_value(true)
					.help("Attribute key")
				)
			)
		);

		app = app.subcommand(Command::new("theme")
			.alias("t")
			.about("Manage themes")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.alias("l")
				.about("List all installed themes")
				.arg(Arg::new("favs")
					.short('f')
					.long("favs")
					.help("Show only favourite themes")
				)
			)
			.subcommand(Command::new("edit")
				.alias("ed")
				.about("Edit specified theme")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.possible_values(themes)
					.help("Theme to edit")
				)
			)
			.subcommand(Command::new("apply")
				.alias("a")
				.about("Apply specified theme")
				.args([
					Arg::new("theme")
						.required(true)
						.takes_value(true)
						.possible_values(themes)
						.help("Theme to apply on all active patterns by default"),

					Arg::new("pattern")
						.short('p')
						.long("pattern")
						.takes_value(true)
						.multiple_values(true)
						.possible_values(patterns)
						.value_name("pattern")
						.help("Apply the theme only on selected patterns"),

					Arg::new("invert")
						.short('i')
						.long("invert")
						.takes_value(true)
						.multiple_values(true)
						.possible_values(patterns)
						.value_name("pattern")
						.help("Invert the foreground and background colors on selected patterns"),

					Arg::new("dry-run")
						.long("dry-run")
						.takes_value(false)
						.help("See possible errors when applying the theme without changing disk files. It does not effectively change theme.")
				])
			)
		);

		app = app.subcommand(Command::new("desktop")
			.alias("d")
			.about("Manage desktops")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.alias("l")
				.about("List all installed desktops")
			)
			.subcommand(Command::new("edit")
				.alias("ed")
				.about("Edit specified desktop using the env var $FILE_EXPLORER")
				.arg(Arg::new("desktop")
					.required(true)
					.takes_value(true)
					.possible_values(desktops)
					.help("Desktop to edit")
				)
			)
			.subcommand(Command::new("info")
				.about("Show desktop information")
				.args([
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Desktop to show info"),
					Arg::new("deps")
						.long("deps")
						.help("Show only desktop dependencies")
				])
			)
			.subcommand(Command::new("add")
				.about("Add new desktop")
				.args([
					Arg::new("path")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.help("Path of desktops' directory"),
					Arg::new("force")
						.long("force")
						.help("Force adding specified desktop by previously removing it")
				])
			)
			.subcommand(Command::new("remove")
				.about("Remove desktop")
				.arg(Arg::new("desktop")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.possible_values(desktops)
					.help("Desktops to remove")
				)
			)
			.subcommand(Command::new("new-skeleton")
				.about("Create a new empty desktop structure")
				.arg(Arg::new("name")
					.required(true)
					.takes_value(true)
					.help("New desktop name")
				)
			)
			.subcommand(Command::new("set-default-theme")
				.alias("sdt")
				.about("Set default theme of current desktop by default")
				.args([
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Set default theme of specified desktop"),
					Arg::new("theme")
						.required(true)
						.takes_value(true)
						.possible_values(themes)
						.help("Theme to set as default"),
				])
			)
			.subcommand(Command::new("status")
				.alias("s")
				.about("Show desktop status of current desktop by default")
				.arg(Arg::new("desktop")
					.short('d')
					.long("desktop")
					.takes_value(true)
					.possible_values(desktops)
					.help("Show status of specified desktop")
				)
			)
			.subcommand(Command::new("apply")
				.alias("a")
				.about("Apply specified desktop")
				.args([
					Arg::new("desktop")
						.required(true)
						.takes_value(true)
						.possible_values(desktops)
						.help("Desktop to apply"),

					Arg::new("theme")
						.short('t')
						.long("theme")
						.takes_value(true)
						.possible_values(themes)
						.help("Apply specified theme after installing the desktop"),

					Arg::new("pattern")
						.short('p')
						.long("pattern")
						.takes_value(true)
						.multiple_values(true)
						.value_name("pattern")
						.help("Apply theme only in specified patterns after installing the desktop. As some patterns may be necessary for a desktop to work this option is not recommeded"),

					Arg::new("invert")
						.short('i')
						.long("invert")
						.takes_value(true)
						.multiple_values(true)
						.value_name("pattern")
						.help("Invert specified patterns when applying the theme after installing the desktop"),

					Arg::new("dry-run")
						.long("dry-run")
						.takes_value(false)
						.help("See possible errors when applying the desktop without changing disk files. It does not effectively change desktop")
				])
			)
		);

		app = app.subcommand(Command::new("pattern")
			.alias("p")
			.about("Manage patterns")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.alias("l")
				.about("List all patterns of current desktop by default")
				.args([
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("List patterns of specified desktop"),
					Arg::new("submodules")
						.short('s')
						.long("submodules")
						.help("Show also pattern submodules")
				])
			)
			.subcommand(Command::new("edit")
				.alias("ed")
				.about("Edit specified pattern in current desktop by default")
				.args([
					Arg::new("pattern")
						.required(true)
						.takes_value(true)
						.possible_values(patterns)
						.help("Pattern to edit"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Edit pattern in specified desktop"),
					Arg::new("postscript")
						.short('p')
						.long("postcript")
						.help("Edit pattern's postscript insted of the pattern file")
				])
			)
			.subcommand(Command::new("enable")
				.alias("e")
				.about("Enable specified patterns in current desktop")
				.args([
					Arg::new("pattern")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.possible_values(patterns)
						.help("Patterns to enable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Enable patterns in specified desktop")
				])
			)
			.subcommand(Command::new("disable")
				.alias("d")
				.about("Disable specified patterns in the current desktop")
				.args([
					Arg::new("pattern")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.possible_values(patterns)
						.help("Patterns to disable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Disable patterns in specified desktop")
				])
			)
			.subcommand(Command::new("toggle")
				.alias("t")
				.about("Toggle specified patterns in the current desktop")
				.args([
					Arg::new("pattern")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.possible_values(patterns)
						.help("Patterns to toggle"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Toggle patterns in specified desktop")
				])
			)
			.subcommand(Command::new("invert")
				.alias("i")
				.about("Invert specified patterns or return them to default if they are already inverted")
				.args([
					Arg::new("pattern")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.possible_values(patterns)
						.help("Patterns to invert"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Invert patterns in specified desktop")
				])
			)
		);

		app = app.subcommand(Command::new("extra")
			.alias("e")
			.about("Manage extras")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.alias("l")
				.about("List all extras of current desktop by default")
				.arg(Arg::new("desktop")
					.short('d')
					.long("desktop")
					.takes_value(true)
					.possible_values(desktops)
					.help("List extras of specified desktop")
				)
			)
			.subcommand(Command::new("edit")
				.alias("ed")
				.about("Edit specified extra")
				.args([
					Arg::new("extra")
						.required(true)
						.takes_value(true)
						.possible_values(extras)
						.help("Extra to edit"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Edit extra in specified desktop")
				])
			)
			.subcommand(Command::new("enable")
				.alias("e")
				.about("Enable specified extras in the current desktop")
				.args([
					Arg::new("extra")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.possible_values(extras)
						.help("Extras to enable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Enable extras in specified desktop")
				])
			)
			.subcommand(Command::new("disable")
				.alias("d")
				.about("Disable specified extras in the current desktop")
				.args([
					Arg::new("extra")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.possible_values(extras)
						.help("Extras to disable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Disable extras in specified desktop")
				])
			)
			.subcommand(Command::new("toggle")
				.alias("t")
				.about("Toggle specified extras in the current desktop")
				.args([
					Arg::new("extra")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.possible_values(extras)
						.help("Extras to toggle"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
						.possible_values(desktops)
						.help("Toggle extras in specified desktop")
				])
			)
		);

		app = app.subcommand(Command::new("fav")
			.alias("f")
			.about("Manage fav themes")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.alias("l")
				.about("List favourite themes")
			)
			.subcommand(Command::new("add")
				.alias("a")
				.about("Add selected themes to the favourite themes list")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.possible_values(themes)
					.help("Themes to add")
				)
			)
			.subcommand(Command::new("remove")
				.alias("r")
				.about("Remove selected themes from the favourite themes list")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.possible_values(fav_themes)
					.help("Themes to remove")
				)
			)
			.subcommand(Command::new("toggle")
				.alias("t")
				.about("Toggle selected themes from the favourite themes list")
				.arg(Arg::new("theme")
					.required(true)
					.takes_value(true)
					.multiple_values(true)
					.possible_values(themes)
					.help("Themes to toggle")
				)
			)
		);

		Cli { app }
	}
}
