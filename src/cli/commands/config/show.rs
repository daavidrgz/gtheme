use crate::core::config::UserConfig;
use colored::*;

pub fn run() {
    let user_settings = UserConfig::new();

    let mut sorted_props = vec![];
    for p in user_settings.get_properties() {
        sorted_props.push(p)
    }
    sorted_props.sort_by(|(a, _), (b, _)| a.cmp(b));

    println!("\n{}\n", "GLOBAL SETTINGS".bold().underline().yellow());
    for (key, value) in sorted_props {
        println!("{} = '{}'", key.bold().green(), value)
    }
    println!();
}
