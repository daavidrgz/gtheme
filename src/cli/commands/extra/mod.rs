use clap::ArgMatches;

use crate::cli::utils::Action;

pub mod edit;
pub mod list;
pub mod manage;

pub fn hanlde_subcommands(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("list", sub_sub_matches)) => list::run(sub_sub_matches),
        Some(("edit", sub_sub_matches)) => edit::run(sub_sub_matches),
        Some(("enable", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Enable),
        Some(("disable", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Disable),
        Some(("toggle", sub_sub_matches)) => manage::run(sub_sub_matches, Action::Toggle),
        _ => (),
    }
}
