use clap::ArgMatches;
use colored::*;

use crate::cli::commands::utils;
use crate::core::config::DesktopInfo;

pub fn run(matches: &ArgMatches) {
    let desktop = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };

    let desktop_info = DesktopInfo::new(&desktop);
    let dependencies = desktop_info.get_dependencies();
    let optional_dependencies = desktop_info.get_optional_dependencies();

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

    println!("{}", "Dependencies:".green().bold());
    for dep in dependencies {
        println!(" • {}", dep)
    }

    if !optional_dependencies.is_empty() {
        println!("{}", "Optional Dependencies:".green().bold());
        for dep in optional_dependencies {
            println!(" • {}", dep)
        }
    }

    println!("");
}
