mod config;
mod desktop;
mod extra;
mod pattern;
mod theme;
mod fav;

use clap::{Arg, Command};

pub fn create_app<'a>(themes: &'a [&'a str], desktops: &'a [&'a str], patterns: &'a [&'a str], 
fav_themes: &'a [&'a str], extras: &'a [&'a str]) -> Command<'a> {
	let mut app = Command::new("gtheme")
		.version("1.0")
		.about("A rust program that makes your theming life so much easier.")
		.author("David Rodriguez & Jorge Hermo")
		.arg(Arg::new("verbose")
			.short('v')
			.long("verbose")
			.global(true)
			.help("Show more information")
		);

	app = config::init(app);
	app = desktop::init(app, desktops, themes); 
	app = theme::init(app, themes, patterns);
	app = pattern::init(app, patterns, desktops);
	app = extra::init(app, extras, desktops);
	app = fav::init(app, fav_themes, themes);
	
	return app;
}