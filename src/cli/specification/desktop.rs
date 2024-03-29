use clap::{Arg, Command};

pub fn init<'a>(app: Command<'a>, desktops: &'a [&'a str], themes: &'a [&'a str]) -> Command<'a> {
    let app = app.subcommand(Command::new("desktop")
		.alias("d")
		.about("Manage desktops")
		.subcommand_required(true)
		.subcommand(Command::new("list")
			.alias("l")
			.about("List all installed desktops")
			.arg(Arg::new("quiet")
				.short('q')
				.long("quiet")
				.help("Show only desktop's names"))
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
			.about("Remove desktops")
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

    return app;
}
