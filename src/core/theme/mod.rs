use std::collections::BTreeMap;
use std::fs::{self,File, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use log::{warn,error};

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
		let mut file = File::open(theme.get_path()).expect("Could not open theme file");
		let mut content = String::new();

		match file.read_to_string(&mut content) {
			Ok(_) => (),
			Err(e) => {
				error!("Error reading theme file |{}|: |{}|", theme.get_path(), e);
				warn!("Using default theme");
				return Theme::default(theme.get_name());
			}
		}

		match serde_json::from_str(&content) {
			Ok(t) => t,
			Err(e) => {
				error!("Error while deserializing theme file |{}|: |{}|", theme.get_path(), e);
				warn!("Using default theme colors");
				return Theme::default(theme.get_name());
			}
		}
	}
	pub fn save(&self) {
		let content = toml::to_string_pretty(self).unwrap();
		// let mut splitted:Vec<&str> = content.trim().split("\n").collect();

		// let mut to_order:Vec<&str> = splitted.drain(1..).collect();
		// to_order.sort_by(|a,b| a.cmp(b));

		// let content = splitted.into_iter().chain(to_order.into_iter())
		// 	.map(|e|e.to_string()).collect::<Vec<String>>().join("\n");

		let path = format!("{}/themes_tmp/{}.toml",core::expand_path(core::GTHEME_HOME),self.get_name());
		let mut file = match OpenOptions::new().create(true).write(true).truncate(true).open(&path) {
			Ok(f) => f,
			Err(e) => {
				error!("Could not open |{}|: |{}|", &path, e);
				return;
			}
		};
		match file.write_all(&content.as_bytes()) {
			Err(e) => error!("Could not write user settings in |{}|: |{}|", &path, e),
			_ => ()
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
		let themes = super::Theme::get_themes();
		themes.iter().map(|t|t.to_theme().save()).collect::<Vec<_>>();

	}
}