use clap_complete::{generate_to, shells::Shell};
use std::{
    env, fs,
    io::Result,
    path::{Path, PathBuf},
};

include!("src/cli/commands.rs");

fn main() -> Result<()> {
			let out_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");

			// Generate completions
			let mut app = build_app();
			generate_to(Shell::Bash, &mut app, "gtheme", &out_dir)?;
			generate_to(Shell::Zsh, &mut app, "gtheme", &out_dir)?;
			generate_to(Shell::Fish, &mut app, "gtheme", &out_dir)?;
			generate_to(Shell::PowerShell, &mut app, "gtheme", &out_dir)?;
			generate_to(Shell::Elvish, &mut app, "gtheme", &out_dir)?;

			// Generate manpage
			// let app = app.name("gtheme");
			// let man = clap_mangen::Man::new(app);
			// let mut buffer: Vec<u8> = Default::default();
			// man.render(&mut buffer)?;
			// std::fs::write(manpage_out_dir.join("gtheme.1"), buffer)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./src/cli/commands.rs");
    println!("cargo:rerun-if-env-changed=GENERATE");

    Ok(())
	}
