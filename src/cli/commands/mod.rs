use clap::ArgMatches;

mod config;
mod desktop;
mod extra;
mod fav;
mod pattern;
mod theme;
mod utils;

pub fn handle_command(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("config", sub_matches)) => config::handle_subcommands(sub_matches),
        Some(("desktop", sub_matches)) => desktop::handle_subcommands(sub_matches),
        Some(("extra", sub_matches)) => extra::handle_subcommands(sub_matches),
        Some(("fav", sub_matches)) => fav::handle_subcommands(sub_matches),
        Some(("pattern", sub_matches)) => pattern::handle_subcommands(sub_matches),
        Some(("theme", sub_matches)) => theme::handle_subcommands(sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
