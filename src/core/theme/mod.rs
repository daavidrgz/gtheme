use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::PathBuf;

use crate::core;

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub extras: BTreeMap<String, Vec<String>>,
    pub colors: BTreeMap<String, String>,
}

impl TryFrom<ThemeFile> for Theme {
    type Error = io::Error;

    fn try_from(theme_file: ThemeFile) -> Result<Self, Self::Error> {
        let mut file = File::open(theme_file.path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let theme: Theme = toml::from_str(&contents)?;
        Ok(theme)
    }
}
impl Default for Theme {
    //TODO: Return an usable theme, or return an skeleton? Then, new skeleton would call this method
    fn default() -> Self {
        // Nord theme colors by default

        //TODO: use lazy_static for defaul theme initialization
        let colors = [
            ("background", "2e3440"),
            ("foreground", "d8dee9"),
            ("cursor", "d8dee9"),
            ("selection-background", "e5e8f0"),
            ("selection-foreground", "2e3440"),
            ("black", "3b4252"),
            ("black-hg", "4c566a"),
            ("red", "bf616a"),
            ("red-hg", "bf616a"),
            ("green", "a3be8c"),
            ("green-hg", "a3be8c"),
            ("yellow", "ebcb8b"),
            ("yellow-hg", "ebcb8b"),
            ("blue", "81a1c1"),
            ("blue-hg", "81a1c1"),
            ("magenta", "b48ead"),
            ("magenta-hg", "b48ead"),
            ("cyan", "88c0d0"),
            ("cyan-hg", "8fbcbb"),
            ("white", "e5e8f0"),
            ("white-hg", "eceff4"),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();

        let extras = BTreeMap::new();
        let name = "DEFAULT_THEME".to_string();

        Theme {
            name,
            colors,
            extras,
        }
    }
}
impl Theme {
    pub fn get_by_name(name: &str) -> Option<ThemeFile> {
        let all_themes = Theme::get_themes();
        match all_themes
            .into_iter()
            .find(|item| item.get_name().to_lowercase() == name.to_lowercase())
        {
            None => {
                error!("Theme |{}| does not exist!", name);
                None
            }
            Some(theme) => Some(theme),
        }
    }
    pub fn exists(desktop: &str) -> bool {
        Theme::get_themes()
            .iter()
            .any(|desktop_file| desktop_file.get_name().to_lowercase() == desktop.to_lowercase())
    }

    fn save(&self) {
        let content = toml::to_string_pretty(self).unwrap();
        let path = format!(
            "{}/themes/{}.toml",
            core::utils::expand_path(core::GTHEME_HOME),
            self.get_name()
        );

        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
        {
            Ok(f) => f,
            Err(e) => {
                error!("Could not open |{}|: |{}|", &path, e);
                return;
            }
        };
        if let Err(e) = file.write_all(&content.as_bytes()) {
            error!("Could not save theme in |{}|: |{}|", &path, e);
        }
    }

    pub fn new_skeleton(theme_name: &str) {
        if Self::exists(theme_name) {
            error!("Theme |{theme_name}| already exists");
            return;
        }

        let theme_path = format!("{}/themes/", core::utils::expand_path(core::GTHEME_HOME));

        if let Err(reason) = fs::create_dir_all(&theme_path) {
            error!("Error while creating directory |{theme_path}|: |{reason}|");
            return;
        }

        let mut colors = BTreeMap::new();

        let pairs = vec![
            ("background", ""),
            ("foreground", ""),
            ("cursor", ""),
            ("selection-background", ""),
            ("selection-foreground", ""),
            ("black", ""),
            ("black-hg", ""),
            ("red", ""),
            ("red-hg", ""),
            ("green", ""),
            ("green-hg", ""),
            ("yellow", ""),
            ("yellow-hg", ""),
            ("blue", ""),
            ("blue-hg", ""),
            ("magenta", ""),
            ("magenta-hg", ""),
            ("cyan", ""),
            ("cyan-hg", ""),
            ("white", ""),
            ("white-hg", ""),
        ];

        colors.extend(
            pairs
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string())),
        );

        let extras = BTreeMap::new();
        let theme = Theme {
            name: theme_name.to_string(),
            colors,
            extras,
        };
        theme.save();
        info!("Successfully created theme |{}|", theme_name);
    }
}

#[derive(Debug, Clone)]
pub struct ThemeFile {
    pub name: String,
    pub path: PathBuf,
}

impl From<DirEntry> for ThemeFile {
    fn from(entry: DirEntry) -> Self {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        ThemeFile { name, path }
    }
}

impl ThemeFile {
    pub fn get_all() -> Vec<ThemeFile> {
        let gtheme_home = core::utils::expand_path(core::GTHEME_HOME);
        let themes_dir = gtheme_home.join("themes");

        let all_entries = match core::utils::read_dir(themes_dir) {
            Ok(themes) => themes,
            Err(e) => {
                error!("Could not read themes directory: |{e})");
                return Vec::new();
            }
        };

        for entry in all_entries {
            let path = entry.path();
            let name = entry.file_name().to_str().unwrap().to_string();
            let theme_file = ThemeFile { name, path };
            println!("{:?}", theme_file);
        }

        // let mut vec = Vec::new();
        // for entry in entries {
        //     let entry = match entry {
        //         Ok(entry) => entry,
        //         Err(e) => {
        //             error!(
        //                 "Error while reading entry from dir |{}|: |{}|",
        //                 &themes_dir, e
        //             );
        //             continue;
        //         }
        //     };

        //     let file_name = match entry.file_name().into_string() {
        //         Ok(file_name) => file_name,
        //         Err(_) => {
        //             error!("Error while converting OsString to String: |Invalid unicode data|");
        //             continue;
        //         }
        //     };

        //     let path = match entry.path().to_str() {
        //         Some(path) => String::from(path),
        //         None => {
        //             error!("Error while converting path to String: |Invalid UTF-8 data|");
        //             continue;
        //         }
        //     };

        //     let metadata = match entry.metadata() {
        //         Ok(metadata) => metadata,
        //         Err(err) => {
        //             error!("Could not read metadata from theme |{}|: |{}|", path, err);
        //             continue;
        //         }
        //     };

        //     if !metadata.is_file() || file_name.starts_with(".") {
        //         //if it isnt a file or is a hidden file
        //         continue;
        //     }

        //     let name = match file_name.rsplit_once(".") {
        //         None => file_name,
        //         Some((prefix, _)) => String::from(prefix),
        //     };
        //     vec.push(ThemeFile { name, path });
        // }
        // vec.sort_by(|a, b| {
        //     a.get_name()
        //         .to_lowercase()
        //         .cmp(&b.get_name().to_lowercase())
        // });
        // vec
        todo!()
    }

    pub fn remove(&self) {
        let path = self.path;
        let theme_name = self.name;
        info!("Removing theme |{theme_name}| from |{}|", path.display());

        if let Err(reason) = fs_extra::remove_items(&[path]) {
            error!(
                "Could not remove theme |{theme_name}| from |{}|: |{reason}|",
                path.display()
            );
            return;
        }

        info!("Successfully removed theme |{theme_name}|");
    }
}
