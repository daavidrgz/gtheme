use clap_complete::{generate_to, shells::Shell};
use clap_mangen::Man;
use std::{env, fs, io::Result};

include!("src/cli/commands.rs");

fn main() -> Result<()> {
	let completions_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
	let manpage_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("manpage/");
	let _ = fs::create_dir(&completions_dir);
	let _ = fs::create_dir(&manpage_dir);

	// Generate completions
	let mut app = build_app();
	generate_to(Shell::Bash, &mut app, "gtheme", &completions_dir)?;
	generate_to(Shell::Zsh, &mut app, "gtheme", &completions_dir)?;
	generate_to(Shell::Fish, &mut app, "gtheme", &completions_dir)?;
	generate_to(Shell::PowerShell, &mut app, "gtheme", &completions_dir)?;
	generate_to(Shell::Elvish, &mut app, "gtheme", &completions_dir)?;

	// Generate manpage
	let app = app.name("gtheme");
	let man = Man::new(app);
	let mut buffer: Vec<u8> = Default::default();
	man.render(&mut buffer)?;
	std::fs::write(manpage_dir.join("gtheme.1"), buffer)?;

	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=./src/cli/commands.rs");
	println!("cargo:rerun-if-env-changed=GENERATE");

	Ok(())
}
