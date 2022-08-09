use clap::ArgMatches;

pub mod apply;
pub mod colors;
pub mod edit;
pub mod list;
pub mod newskeleton;
pub mod remove;

pub fn handle_subcommands(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("list", sub_sub_matches)) => list::run(sub_sub_matches),
        Some(("colors", sub_sub_matches)) => colors::run(sub_sub_matches),
        Some(("edit", sub_sub_matches)) => edit::run(sub_sub_matches),
        Some(("new-skeleton", sub_sub_matches)) => newskeleton::run(sub_sub_matches),
        Some(("remove", sub_sub_matches)) => remove::run(sub_sub_matches),
        Some(("apply", sub_sub_matches)) => apply::run(sub_sub_matches),
        _ =>  unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
