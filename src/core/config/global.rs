use crate::core::{
    self,
    desktop::{Desktop, DesktopFile},
    theme::{Theme, ThemeFile},
};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct GlobalConfigDto {
    current_desktop: Option<String>,
    current_theme: Option<String>,
    fav_themes: Vec<String>,
}

#[derive(Debug)]
pub struct GlobalConfig {
    current_desktop: Option<DesktopFile>,
    current_theme: Option<ThemeFile>,
    fav_themes: Vec<ThemeFile>,
}

impl GlobalConfigDto {
    fn new() -> Self {
        let path = format!(
            "{}/global_config.json",
            core::expand_path(core::GTHEME_HOME)
        );
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                warn!(
                    "Could not open global config, using default config: |{}|",
                    e
                );
                let config = Self::default();
                config.save();
                return config;
            }
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Could not read global config, using default config: |{}|",
                    e
                );
                let config = Self::default();
                config.save();
                return config;
            }
        };
        match serde_json::from_str(&content) {
            Ok(config) => {
                info!("Using global config |{}|", &path);
                config
            }
            Err(e) => {
                error!(
                    "Could not parse global config, using default config: |{}|",
                    e
                );
                return Self::default();
            }
        }
    }

    fn from(config: &GlobalConfig) -> Self {
        let current_desktop = match config.get_current_desktop() {
            None => None,
            Some(desktop) => Some(String::from(desktop.get_name())),
        };
        let current_theme = match config.get_current_theme() {
            None => None,
            Some(theme) => Some(String::from(theme.get_name())),
        };

        let fav_themes = config
            .get_fav_themes()
            .iter()
            .map(|theme| String::from(theme.get_name()))
            .collect();

        GlobalConfigDto {
            current_desktop,
            current_theme,
            fav_themes,
        }
    }

    fn save(&self) {
        let content = serde_json::to_string_pretty(self).unwrap();
        let path = format!(
            "{}/global_config.json",
            core::expand_path(core::GTHEME_HOME)
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
        match file.write_all(&content.as_bytes()) {
            Ok(_) => info!("Saving global config..."),
            Err(e) => error!("Could not write global config in |{}|: |{}|", &path, e),
        }
    }

    fn get_current_desktop(&self) -> &Option<String> {
        &self.current_desktop
    }
    fn get_current_theme(&self) -> &Option<String> {
        &self.current_theme
    }
    fn get_fav_themes(&self) -> &Vec<String> {
        &self.fav_themes
    }
}

impl Default for GlobalConfigDto {
    fn default() -> GlobalConfigDto {
        GlobalConfigDto {
            current_desktop: None,
            current_theme: None,
            fav_themes: Vec::new(),
        }
    }
}

impl GlobalConfig {
    pub fn new() -> Self {
        let dto = GlobalConfigDto::new();
        let desktops = Desktop::get_desktops();
        let themes = Theme::get_themes();

        let fav_themes_string = dto.get_fav_themes();

        let current_desktop = match dto.get_current_desktop() {
            None => None,
            Some(desktop) => desktops.into_iter().find(|item| item.get_name() == desktop),
        };
        let current_theme = match dto.get_current_theme() {
            None => None,
            Some(theme) => themes
                .clone()
                .into_iter()
                .find(|item| item.get_name() == theme),
        };

        let fav_themes = themes
            .into_iter()
            .filter(|item| fav_themes_string.contains(item.get_name()))
            .collect();

        GlobalConfig {
            current_desktop,
            current_theme,
            fav_themes,
        }
    }

    pub fn save(&self) {
        GlobalConfigDto::from(self).save()
    }
    pub fn get_current_desktop(&self) -> &Option<DesktopFile> {
        &self.current_desktop
    }
    pub fn get_mut_current_desktop(&mut self) -> &mut Option<DesktopFile> {
        &mut self.current_desktop
    }
    pub fn set_current_desktop(&mut self, desktop: DesktopFile) {
        self.current_desktop = Some(desktop)
    }

    pub fn get_current_theme(&self) -> &Option<ThemeFile> {
        &self.current_theme
    }
    pub fn get_mut_current_theme(&mut self) -> &mut Option<ThemeFile> {
        &mut self.current_theme
    }
    pub fn set_current_theme(&mut self, theme: ThemeFile) {
        self.current_theme = Some(theme);
    }

    pub fn get_fav_themes(&self) -> &Vec<ThemeFile> {
        &self.fav_themes
    }
    pub fn get_mut_fav_themes(&mut self) -> &mut Vec<ThemeFile> {
        &mut self.fav_themes
    }

    pub fn toggle_fav_theme(&mut self, theme: &ThemeFile) {
        let theme_name = theme.get_name().to_lowercase();
        match self
            .fav_themes
            .binary_search_by(|item| item.get_name().to_lowercase().cmp(&theme_name))
        {
            Ok(_) => self.remove_fav_theme(theme),
            Err(_) => self.add_fav_theme(theme),
        }
    }
    pub fn add_fav_theme(&mut self, theme: &ThemeFile) {
        let theme_name = theme.get_name().to_lowercase();
        match self
            .fav_themes
            .binary_search_by(|item| item.get_name().to_lowercase().cmp(&theme_name))
        {
            Ok(_) => warn!(
                "Theme |{}| was already on fav themes list",
                theme.get_name()
            ),
            Err(pos) => {
                self.fav_themes.insert(pos, theme.clone());
                info!(
                    "Theme |{}| successfuly added to the fav themes list!",
                    theme.get_name()
                );
            }
        }
    }
    pub fn remove_fav_theme(&mut self, theme: &ThemeFile) {
        let theme_name = theme.get_name().to_lowercase();
        match self
            .fav_themes
            .binary_search_by(|item| item.get_name().to_lowercase().cmp(&theme_name))
        {
            Ok(pos) => {
                self.fav_themes.remove(pos);
                info!(
                    "Theme |{}| successfuly removed from the fav themes list!",
                    theme.get_name()
                );
            }
            Err(_) => warn!(
                "Theme |{}| was not in the fav themes list!",
                theme.get_name()
            ),
        }
    }
}
