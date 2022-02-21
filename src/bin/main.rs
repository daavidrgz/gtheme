use clap::{Command, Arg};

fn main() {
// 	let mut app = Command::new("gtheme")
// 		.version("1.0")
// 		.about("A rust program that makes your theming life so much easier.")
// 		.author("David Rodríguez & Jorge Hermo");

// 	let apply_theme = Arg::new("theme")
// 		.short('t')
// 		.long("theme") // allow --name
// 		.takes_value(true)
// 		.help("Apply specified theme")
// 		.exclusive(true);

// let activate_patterns = Arg::new("activate")
// 		.short('a')
// 		.long("activate")
// 		.takes_value(true) // allow --name
// 		.multiple_values(true)
// 		.value_name("pattern")
// 		.help("Activate specified patterns in the current desktop")
// 		.exclusive(true);

// 	app = app.args([apply_theme, activate_patterns]);

// 	let matches = app.get_matches();

// 	match matches.value_of("theme") {
// 		Some(theme_name) => println!("{}", theme_name),
// 		None => println!("There is no theme specified!")
// 	};

  app::Ui::default().start_ui()
}
