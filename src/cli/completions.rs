use clap_complete::{generate_to, shells::Shell};
use log::error;
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

use crate::cli::specification;
use crate::core::{
    self, config::GlobalConfig, desktop::Desktop, pattern::Pattern, postscript::PostScript,
    theme::Theme,
};

fn escape_string(s: String) -> String {
    s.replace('(', "\\(").replace(')', "\\)")
}

pub fn get_themes() -> Vec<String> {
    let themes = Theme::get_themes();
    let themes = themes
        .into_iter()
        .map(|t| t.get_name().to_string().to_lowercase());
    themes.map(escape_string).collect()
}

pub fn get_fav_themes(global_config: &GlobalConfig) -> Vec<String> {
    let fav_themes = global_config.get_fav_themes();
    let fav_themes = fav_themes
        .iter()
        .map(|t| t.get_name().to_string().to_lowercase());
    fav_themes.map(escape_string).collect()
}

pub fn get_desktops() -> Vec<String> {
    let desktops = Desktop::get_desktops();
    let desktops = desktops
        .into_iter()
        .map(|d| d.get_name().to_string().to_lowercase());
    desktops.map(escape_string).collect()
}

pub fn get_patterns(global_config: &GlobalConfig) -> Vec<String> {
    let desktop = match global_config.get_current_desktop() {
        None => return vec![],
        Some(desktop) => desktop,
    };
    let patterns = Pattern::get_patterns(desktop);
    let patterns = patterns
        .into_iter()
        .map(|p| p.get_name().to_string().to_lowercase());
    patterns.map(escape_string).collect()
}

pub fn get_extras(global_config: &GlobalConfig) -> Vec<String> {
    let desktop = match global_config.get_current_desktop() {
        None => return vec![],
        Some(desktop) => desktop,
    };
    let extras = PostScript::get_extras(desktop);
    let extras = extras
        .into_iter()
        .map(|p| p.get_name().to_string().to_lowercase());
    extras.map(escape_string).collect()
}

pub fn generate_completion_files(app: &mut clap::Command, completions_dir: &PathBuf) -> Result<()> {
    generate_to(Shell::Bash, app, "gtheme", &completions_dir)?;
    generate_to(Shell::Zsh, app, "gtheme", &completions_dir)?;

    let fish_dir = Path::new(&core::expand_path(core::CONFIG_HOME)).join("fish/completions");
    if fish_dir.exists() {
        generate_to(Shell::Fish, app, "gtheme", fish_dir)?;
    }

    Ok(())
}

pub fn generate_completions() {
    let completions_dir = Path::new(&core::expand_path(core::GTHEME_MISC)).join("completions");
    let global_config = GlobalConfig::new();

    let themes_owned = get_themes();
    let desktops_owned = get_desktops();
    let patterns_owned = get_patterns(&global_config);
    let fav_themes_owned = get_fav_themes(&global_config);
    let extras_owned = get_extras(&global_config);

    let themes: Vec<&str> = themes_owned.iter().map(|s| s.as_str()).collect();
    let desktops: Vec<&str> = desktops_owned.iter().map(|s| s.as_str()).collect();
    let patterns: Vec<&str> = patterns_owned.iter().map(|s| s.as_str()).collect();
    let fav_themes: Vec<&str> = fav_themes_owned.iter().map(|s| s.as_str()).collect();
    let extras: Vec<&str> = extras_owned.iter().map(|s| s.as_str()).collect();

    let mut app = specification::create_app(&themes, &desktops, &patterns, &fav_themes, &extras);

    // Generate completions
    if let Err(e) = fs::create_dir_all(&completions_dir) {
        error!("Error while creating completions directory: |{e}|");
        return;
    }
    if let Err(e) = generate_completion_files(&mut app, &completions_dir) {
        error!("Error while generating completion scripts: |{e}|");
    }
}
