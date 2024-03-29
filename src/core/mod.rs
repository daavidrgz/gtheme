use log::error;
use std::{
    fs::{self, DirEntry, File},
    io::{self, Write},
    path::Path,
};
pub mod config;
pub mod desktop;
pub mod pattern;
pub mod postscript;
pub mod theme;

pub const GTHEME_HOME: &str = "~/.config/gtheme";
pub const GTHEME_MISC: &str = "~/.gtheme";
pub const CONFIG_HOME: &str = "~/.config";

pub fn expand_path(origin_path: &str) -> String {
    let mut path = String::from(origin_path);
    path = shellexpand::tilde(&path).to_string();
    path = shellexpand::env(&path).unwrap().to_string();
    path
}

//TODO: rework of functions that read directories to use this function
fn get_files(path: &Path) -> Vec<DirEntry> {
    if !path.exists() {
        return vec![];
    }

    let entries = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(e) => {
            error!(
                "Could not read directory |{}|: |{}|",
                &path.as_os_str().to_string_lossy(),
                e
            );
            return vec![];
        }
    };

    let mut vec = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!(
                    "Error while reading entry from dir |{}|: |{}|",
                    &path.as_os_str().to_string_lossy(),
                    e
                );
                continue;
            }
        };
        vec.push(entry);
    }
    vec
}
// TODO:
fn write_content_to(content: &String, path: &Path) -> io::Result<()> {
    let prefix = path.parent().unwrap_or(Path::new("/"));
    fs::create_dir_all(prefix)?;
    // Check config save functions to use this instead.
    let mut output_file = File::create(path)?;
    return output_file.write_all(content.as_bytes());
}
//TODO: return result?
fn copy(from: &[&Path], to: &Path) {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;
    let to_display = to.display();
    if let Err(e) = fs::create_dir_all(to) {
        error!("Could not create |{to_display}|: |{e}|");
        return;
    }
    if let Err(e) = fs_extra::copy_items(from, &to, &options) {
        error!("Error while copying to |{to_display}|: |{e}|");
        return;
    }
}

#[cfg(test)]
mod tests {
    // use super::pattern::Pattern;
    use super::desktop::Desktop;
    use super::theme::Theme;
    use std::collections::BTreeMap;

    // #[test]
    // fn test_fill_pattern(){
    // 	let patterns = Pattern::get_patterns("jorge");
    // 	let pattern = patterns.into_iter().find(|pattern| pattern.get_name()=="kitty").unwrap().to_pattern();

    // 	let themes = Theme::get_themes();
    // 	let theme = themes.into_iter().find(|theme |theme.get_name()=="Nord" ).unwrap().to_theme();

    // 	// pattern.fill(&theme)
    // }
    // #[test]

    // #[test]
    // fn test_search_and_fill_pattern(){
    // 	let themes = Theme::get_themes();
    // 	let selected_file = themes.get(0).unwrap();
    // 	let selected_theme = selected_file.to_theme();

    // 	let patterns = Pattern::get_patterns("jorge");
    // 	let pattern = patterns.iter().find(|pattern| pattern.get_name()=="kitty").unwrap().to_pattern();

    // 	// pattern.fill(&selected_theme);
    // }

    #[test]
    fn test_apply_theme() {
        let desktops = Desktop::get_desktops();
        let desktop = desktops
            .into_iter()
            .find(|desktop| desktop.get_name() == "jorge")
            .unwrap();

        let themes = Theme::get_themes();
        let theme = themes
            .into_iter()
            .find(|theme| theme.get_name() == "Nord")
            .unwrap()
            .to_theme();

        let desktop = desktop.to_desktop();
        let patterns = desktop.get_patterns();

        let mut active = BTreeMap::new();
        for pattern in patterns {
            active.insert(String::from(pattern.get_name()), true);
        }
        active.insert(String::from("wallpaper"), true);

        let mut inverted = BTreeMap::new();
        inverted.insert(String::from("polybar"), true);

        desktop.apply_theme(&theme, &active, &inverted, false);
    }
}
