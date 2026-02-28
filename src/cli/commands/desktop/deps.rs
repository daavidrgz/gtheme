use clap::ArgMatches;

use crate::cli::commands::utils;
use crate::core::config::DesktopInfo;

pub fn run(matches: &ArgMatches) {
    let desktop = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };

    let desktop_info = DesktopInfo::new(&desktop);

    for dep in desktop_info.get_dependencies() {
        println!("{}", dep);
    }

    if matches.is_present("all") {
        for dep in desktop_info.get_optional_dependencies() {
            println!("{}", dep);
        }
    }
}
