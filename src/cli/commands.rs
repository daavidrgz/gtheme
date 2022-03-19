use clap::*;

pub fn get_matches() -> clap::ArgMatches {
	build_app().get_matches()
}

pub fn build_app() -> Command<'static> {
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
			.alias("t")
			.about("Manage themes")
			.subcommand_required(true)
			.subcommand(Command::new("list")
				.alias("l")
				.about("List all installed themes")
			)
			.subcommand(Command::new("apply")
				.alias("a")
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
			.subcommand(Command::new("apply")
				.alias("a")
				.about("Apply specified desktop")
				.args([
					Arg::new("desktop")
						.required(true)
						.takes_value(true)
						.help("Desktop to apply"),

					Arg::new("theme")
						.short('t')
						.long("theme")
						.takes_value(true)
						.help("Apply specified theme after installing the desktop"),

					Arg::new("dry-run")
						.long("dry-run")
						.takes_value(false)
						.help("See possible errors when applying the desktop without changing disk files. It does not effectively change desktop.")
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
				.arg(Arg::new("desktop")
					.short('d')
					.long("desktop")
					.takes_value(true)
					.help("List patterns of specified desktop")
				)
			)
			.subcommand(Command::new("enable")
				.alias("e")
				.about("Enable specified patterns in current desktop")
				.args([
					Arg::new("pattern")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.help("Patterns to enable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
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
						.help("Patterns to disable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
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
						.help("Patterns to toggle"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
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
						.help("Patterns to invert"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
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
					.help("List extras of specified desktop")
				)
			)
			.subcommand(Command::new("enable")
				.alias("e")
				.about("Enable specified extras in the current desktop")
				.args([
					Arg::new("extra")
						.required(true)
						.takes_value(true)
						.multiple_values(true)
						.help("Extras to enable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
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
						.help("Extras to disable"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
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
						.help("Extras to toggle"),
					Arg::new("desktop")
						.short('d')
						.long("desktop")
						.takes_value(true)
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
					.help("Themes to toggle")
				)
			)
		);

		app
}
