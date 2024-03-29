use clap::{Arg, Command};

pub fn init<'a>(app: Command<'a>, fav_themes: &'a [&'a str], themes: &'a [&'a str]) -> Command<'a> {
    let app = app.subcommand(
        Command::new("fav")
            .alias("f")
            .about("Manage fav themes")
            .subcommand_required(true)
            .subcommand(
                Command::new("list")
                    .alias("l")
                    .about("List favourite themes")
                    .arg(
                        Arg::new("quiet")
                            .short('q')
                            .long("quiet")
                            .help("Show only favourite theme's names"),
                    ),
            )
            .subcommand(
                Command::new("add")
                    .alias("a")
                    .about("Add selected themes to the favourite themes list")
                    .arg(
                        Arg::new("theme")
                            .required(true)
                            .takes_value(true)
                            .multiple_values(true)
                            .possible_values(themes)
                            .help("Themes to add"),
                    ),
            )
            .subcommand(
                Command::new("remove")
                    .alias("r")
                    .about("Remove selected themes from the favourite themes list")
                    .arg(
                        Arg::new("theme")
                            .required(true)
                            .takes_value(true)
                            .multiple_values(true)
                            .possible_values(fav_themes)
                            .help("Themes to remove"),
                    ),
            )
            .subcommand(
                Command::new("toggle")
                    .alias("t")
                    .about("Toggle selected themes from the favourite themes list")
                    .arg(
                        Arg::new("theme")
                            .required(true)
                            .takes_value(true)
                            .multiple_values(true)
                            .possible_values(themes)
                            .help("Themes to toggle"),
                    ),
            ),
    );

    return app;
}
