use clap::ArgMatches;
use colored::*;

use crate::cli::utils;
use crate::core::config::DesktopInfo;

pub fn run(matches: &ArgMatches) {
    let desktop = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };

    let desktop_info = DesktopInfo::new(&desktop);
    let dependencies = desktop_info.get_dependencies();

    if matches.is_present("deps") {
        dependencies.iter().for_each(|dep| println!("{}", dep));
        return;
    }

    println!("");
    println!("{} {}", "Name:".green().bold(), desktop.get_name());
    println!("{} {}", "Author:".green().bold(), desktop_info.get_author());
    println!(
        "{} {}",
        "Credits:".green().bold(),
        desktop_info.get_credits()
    );
    println!(
        "{} {}",
        "Description:".green().bold(),
        desktop_info.get_description()
    );
    println!("{}", "Dependecies:".green().bold());

    for dep in dependencies {
        println!(" • {}", dep)
    }
    println!("");
}
