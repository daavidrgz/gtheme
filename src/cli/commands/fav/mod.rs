use clap::ArgMatches;

use crate::cli::commands::utils::Action;

pub mod list;
mod manage;

pub fn handle_subcommands(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("list", sub_sub_matches)) => list::run(sub_sub_matches),
        Some(("add", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Enable),
        Some(("remove", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Disable),
        Some(("toggle", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Toggle),
        _ =>  unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
