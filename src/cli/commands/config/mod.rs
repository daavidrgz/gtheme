use clap::ArgMatches;

pub mod edit;
pub mod set;
pub mod setup;
pub mod show;
pub mod unset;

pub fn handle_subcommands(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("show", _)) => show::run(),
        Some(("setup", _)) => setup::run(),
        Some(("edit", _)) => edit::run(),
        Some(("set", sub_sub_matches)) => set::run(sub_sub_matches),
        Some(("unset", sub_sub_matches)) => unset::run(sub_sub_matches),
        _ => (),
    }
}
