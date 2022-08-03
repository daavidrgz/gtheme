use clap::{Arg, Command};

pub fn init(app: Command) -> Command {
    let app = app.subcommand(Command::new("config")
		.alias("c")
		.about("Manage user settings")
		.subcommand_required(true)
		.subcommand(Command::new("setup")
			.about("Run an interactive setup to configure user settings")
		)
		.subcommand(Command::new("show")
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

    return app;
}
