use clap::{Arg, Command};

pub fn init<'a>(app: Command<'a>, extras: &'a [&'a str], desktops: &'a [&'a str]) -> Command<'a> {
    let app = app.subcommand(
        Command::new("extra")
            .alias("e")
            .about("Manage extras")
            .subcommand_required(true)
            .subcommand(
                Command::new("list")
                    .alias("l")
                    .about("List all extras of current desktop by default")
                    .args([
                        Arg::new("desktop")
                            .short('d')
                            .long("desktop")
                            .takes_value(true)
                            .possible_values(desktops)
                            .help("List extras of specified desktop"),
                        Arg::new("quiet")
                            .short('q')
                            .long("quiet")
                            .help("Show only extra's names"),
                    ]),
            )
            .subcommand(
                Command::new("edit")
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
                            .help("Edit extra in specified desktop"),
                    ]),
            )
            .subcommand(
                Command::new("enable")
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
                            .help("Enable extras in specified desktop"),
                    ]),
            )
            .subcommand(
                Command::new("disable")
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
                            .help("Disable extras in specified desktop"),
                    ]),
            )
            .subcommand(
                Command::new("toggle")
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
                            .help("Toggle extras in specified desktop"),
                    ]),
            ),
    );

    return app;
}
