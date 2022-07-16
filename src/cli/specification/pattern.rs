use clap::{Arg, Command};

pub fn init<'a>(app: Command<'a>, patterns: &'a [&'a str], desktops: &'a [&'a str]) -> Command<'a> {
	let app = app.subcommand(Command::new("pattern")
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
					.help("Show also pattern submodules"),
				Arg::new("quiet")
					.short('q')
					.long("quiet")
					.help("Show only pattern's names")
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

	return app;
}