use clap::ArgMatches;

use crate::cli::commands::utils::Action;

mod edit;
mod invert;
pub mod list;
mod manage;

pub fn handle_subcommands(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("list", sub_sub_matches)) => list::run(sub_sub_matches),
        Some(("edit", sub_sub_matches)) => edit::run(sub_sub_matches),
        Some(("enable", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Enable),
        Some(("disable", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Disable),
        Some(("toggle", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Toggle),
        Some(("invert", sub_sub_matches)) => invert::run(sub_sub_matches),
        _ =>  unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
