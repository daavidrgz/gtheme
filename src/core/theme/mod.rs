use std::collections::BTreeMap;
use std::fs::{self,File, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use log::{warn,error, info};

use crate::core;

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
	name: String,
	extras: BTreeMap<String,Vec<String>>,
	colors: BTreeMap<String, String>
}

impl Theme {
	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_extras(&self) -> &BTreeMap<String,Vec<String>> {
		&self.extras
	}
	pub fn get_colors(&self) -> &BTreeMap<String, String> {
		&self.colors
	}

	pub fn from(theme: &ThemeFile) -> Self {
		let mut file = match File::open(theme.get_path()) {
			Ok(file) => file,
			Err(e) => {
				warn!("Could not open theme file |{}|, using default theme: |{}|",theme.get_path(), e);
				return Self::default(theme.get_name());
			}
		};
		let mut content = String::new();

		if let Err(e) =  file.read_to_string(&mut content) {
			error!("Could not read theme file |{}|,using default theme: |{}|", theme.get_path(), e);
			return Self::default(theme.get_name());
		}

		match toml::from_str(&content) {
			Ok(theme) => {
				theme
			},
			Err(e) => {
				error!("Error while deserializing theme file |{}|: |{}|", theme.get_path(), e);
				warn!("Using default theme colors");
				Self::default(theme.get_name())
			}
		}
	}
	
	fn default(name:&str) -> Self{
		// Nord theme colors by default
		let mut colors = BTreeMap::new();

		let pairs = vec![
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
			("white-hg", "eceff4")
		];

		colors.extend(pairs.into_iter().map(|(key,value)| (key.to_string(), value.to_string())));

		let extras = BTreeMap::new();
		Theme {
			name: name.to_string(),
			colors,
			extras
		}
	}

	pub fn get_by_name(name: &str) -> Option<ThemeFile> {
		let all_themes = Theme::get_themes();
		match all_themes.into_iter().find(|item| item.get_name().to_lowercase() == name.to_lowercase()) {
			None => {
				error!("The theme |{}| does not exist!", name);
				None
			}
			Some(theme) => Some(theme)
		}
	}

	
	pub fn get_themes() -> Vec<ThemeFile> {
		let gtheme_home: String = core::expand_path(core::GTHEME_HOME);
		let themes_dir = gtheme_home + "/themes";

		let entries = match fs::read_dir(&themes_dir) {
			Ok(dir) => dir,
			Err(e) => {
				error!("Could not read directory |{}|: |{}|", &themes_dir, e);
				return vec![]
			}
		};

		let mut vec = Vec::new();
		for entry in entries {
			let entry = match entry{
				Ok(entry) => entry,
				Err(e) => {
					error!("Error while reading entry from dir |{}|: |{}|", &themes_dir, e);
					continue;
				}
			};

			let file_name = match entry.file_name().into_string() {
				Ok(file_name) => file_name,
				Err(_) => {
					error!("Error while converting OsString to String: |Invalid unicode data|");
					continue;
				}
			};
			
			let path = match entry.path().to_str() {
				Some(path) => String::from(path),
				None => {
					error!("Error while converting path to String: |Invalid UTF-8 data|");
					continue;
				}
			};

			let metadata = match entry.metadata(){
				Ok(metadata) => metadata,
				Err(err) => {
					error!("Could not read metadata from theme |{}|: |{}|",path,err);
					continue;
				}
			};

			if !metadata.is_file() || file_name.starts_with("."){
				//if it isnt a file or is a hidden file
				continue;
			}

			let name = match file_name.rsplit_once(".") {
				None => file_name,
				Some((prefix,_)) => String::from(prefix)
			};
			vec.push(ThemeFile { name, path });
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}

	fn save(&self) {
		let content = toml::to_string_pretty(self).unwrap();
		let path = format!("{}/themes/{}.toml",core::expand_path(core::GTHEME_HOME),self.get_name());
		let mut file = match OpenOptions::new().create(true).write(true).truncate(true).open(&path) {
			Ok(f) => f,
			Err(e) => {
				error!("Could not open |{}|: |{}|", &path, e);
				return;
			}
		};
		if let Err(e)=  file.write_all(&content.as_bytes()) {
			error!("Could not save theme in |{}|: |{}|", &path, e);
		}	
	}

	pub fn new_skeleton(theme_name: &str) {
		if let Some(_) = Self::get_by_name(theme_name) {
			error!("Theme |{}| already exists", theme_name);
			return;
		}

		let theme_path = format!("{}/themes/", core::expand_path(core::GTHEME_HOME));

		match fs::create_dir_all(&theme_path) {
			Ok(_) => info!("Created directory |{}|", &theme_path),
			Err(e) => {
				error!("Error while creating directory |{}|: |{}|", &theme_path, e);
				return;
			}
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
			("white-hg", "")
		];

		colors.extend(pairs.into_iter().map(|(key,value)| (key.to_string(), value.to_string())));

		let extras = BTreeMap::new();
		let theme =Theme {
			name: theme_name.to_string(),
			colors,
			extras
		};
		theme.save();		
		info!("Successfully created theme |{}|", theme_name);
	}
}

#[derive(Debug,Clone)]
pub struct ThemeFile {
	name: String,
	path: String,
}

impl ThemeFile {
	pub fn to_theme(&self) -> Theme {
		Theme::from(self)
	}
	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_path(&self) -> &String {
		&self.path
	}
}

#[cfg(test)]
mod tests{
	#[test]
	fn save(){
		super::Theme::new_skeleton("hola");

	}
}