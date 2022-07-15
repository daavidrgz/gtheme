use clap::ArgMatches;
use std::collections::BTreeSet;
use log::error;
use tint::Color as TintColor;
use colored::*;

use crate::core::{
	theme::Theme,
	config::GlobalConfig
};

pub fn run(matches: &ArgMatches) {
	let theme_file = match matches.value_of("theme") {
		Some(t) => match Theme::get_by_name(t) {
			Some(t) => t,
			None => return
		},
		None => {
			let global_config = GlobalConfig::new();
			match global_config.get_current_theme() {
				Some(t) => t.clone(),
				None => {
					error!("|There is no theme installed!|, try specifing a theme");
					return
				}
			}
		}
	};

	let theme = theme_file.to_theme();
	let sorted_colors = theme.get_colors().into_iter().collect::<BTreeSet<_>>();

	println!("\n{} {}\n", "THEME".bold().underline().green(), theme.get_name().bold());
	for (color_key, color_value) in sorted_colors {
		let color_hex = format!("#{}", &color_value);
		let (r,g, b) = TintColor::from_hex(&color_hex).to_rgb255();
		println!("{color_hex}  {}  {}", "██".truecolor(r, g, b), color_key.bold().cyan())
	}
	println!();
}