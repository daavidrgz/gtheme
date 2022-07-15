use clap::{Arg, Command};

pub fn init<'a>(app: Command<'a>, themes: &'a [&'a str], patterns: &'a [&'a str]) -> Command<'a> {
	let app = app.subcommand(Command::new("theme")
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
		.subcommand(Command::new("colors")
			.alias("c")
			.about("Show colors for current theme by default")
			.arg(Arg::new("theme")
				.required(false)
				.takes_value(true)
				.possible_values(themes)
				.help("Theme to show colors")
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

	return app;
}