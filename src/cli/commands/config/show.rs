use log::error;
use colored::*;
use crate::core::config::UserConfig;

pub fn run() {
	if !UserConfig::exists() {
		error!("|There is no global settings file|, run |gtheme config setup| first");
		return
	}
	let user_settings = UserConfig::new();

	let mut sorted_props = vec![];
	for p in user_settings.get_properties() {
		sorted_props.push(p)
	}
	sorted_props.sort_by(|(a,_),(b,_)| a.cmp(b));

	println!("\n{}\n", "GLOBAL SETTINGS".bold().underline().yellow());
	for (key, value) in sorted_props {
		println!("{} = '{}'", key.bold().green(), value)
	}
	println!("");
}