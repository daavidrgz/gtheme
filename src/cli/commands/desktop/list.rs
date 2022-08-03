use clap::ArgMatches;
use colored::*;

use crate::core::{config::GlobalConfig, desktop::Desktop};

pub fn run(matches: &ArgMatches) {
    let all_desktops = Desktop::get_desktops();
    let global_config = GlobalConfig::new();
    let current_desktop = match global_config.get_current_desktop() {
        Some(d) => d.get_name(),
        None => "",
    };

    if matches.is_present("quiet") {
        all_desktops
            .iter()
            .for_each(|desktop| println!("{}", desktop.get_name()));
        return;
    }

    println!("");
    println!("{}\n", "DESKTOPS".bold().underline().cyan());

    for d in all_desktops {
        if d.get_name() == current_desktop {
            println!(
                "{} {}",
                "•".green(),
                format!("{} (Active)", d.get_name()).bold().green()
            );
        } else {
            println!("{} {}", "•".cyan(), d.get_name());
        };
    }
    println!("");
}
