use clap::ArgMatches;
use colored::*;

use crate::cli::utils;
use crate::core::{
    config::DesktopConfig,
    pattern::{Pattern, PatternFile},
};

pub fn run(matches: &ArgMatches) {
    let desktop = match utils::get_desktop(matches.value_of("desktop")) {
        Some(d) => d,
        None => return,
    };

    let all_patterns = Pattern::get_patterns(&desktop);
    let desktop_config = DesktopConfig::new(&desktop);

    let enabled = desktop_config.get_actived();
    let inverted = desktop_config.get_inverted();

    if matches.is_present("quiet") {
        all_patterns
            .iter()
            .for_each(|pattern| println!("{}", pattern.get_name()));
        return;
    }

    let desktop_title = format!("({})", desktop.get_name());

    println!("");
    println!(
        "{} {}\n",
        "PATTERNS".bold().underline().magenta(),
        desktop_title.bold().cyan()
    );

    for p in all_patterns {
        print!("{} {:<20}", "•".magenta(), p.get_name());
        let color = match enabled.get(p.get_name()) {
            Some(e) => {
                if *e {
                    print!(" {}", "ON".bold().green());
                    Color::Green
                } else {
                    print!(" {}", "OFF".bold().red());
                    Color::Red
                }
            }
            None => {
                print!(" {}", "OFF".bold().red());
                Color::Red
            }
        };

        match inverted.get(p.get_name()) {
            Some(i) => {
                if *i {
                    print!(" {}", "(Inverted)".bold().color(color))
                }
            }
            None => (),
        }

        println!("");
        if matches.is_present("submodules") {
            list_pattern_submodules("  ".to_string(), p.to_pattern().get_submodules());
        }
    }
    println!("");
}

fn list_pattern_submodules(pre: String, submodules_opt: &Option<Vec<PatternFile>>) {
    if let Some(submodules) = submodules_opt {
        if submodules.len() == 0 {
            return;
        }
        for s in submodules.iter().take(submodules.len() - 1) {
            println!("{}{} {}", pre.magenta(), "├".magenta(), s.get_name());
            list_pattern_submodules(pre.clone() + "│ ", s.to_pattern().get_submodules());
        }
        let last = submodules.last().unwrap().to_pattern();
        println!("{}{} {}", pre.magenta(), "└".magenta(), last.get_name());
        list_pattern_submodules(pre.clone() + "  ", last.get_submodules());
    }
}
