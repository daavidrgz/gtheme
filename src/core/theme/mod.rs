use std::collections::HashMap;
use std::fs::{self,File};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use log::{error};

use crate::core;

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
	name: String,
	extras: HashMap<String,Vec<String>>,
	colors: HashMap<String, String>
}

impl Theme {
	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_extras(&self) -> &HashMap<String,Vec<String>> {
		&self.extras
	}
	pub fn get_colors(&self) -> &HashMap<String, String> {
		&self.colors
	}

	//TODO: from str or from ThemeFile??
	pub fn from(theme: &ThemeFile) -> Self {
		let mut file = File::open(theme.get_path()).expect("Could not open theme file");
		let mut content = String::new();

		match file.read_to_string(&mut content) {
			Ok(_) => (),
			Err(e) => {
				error!("Error reading theme file |{}|: |{}|", theme.get_path(), e);
				panic!("Could not read theme file")
			}
		}

		match serde_json::from_str(&content) {
			Ok(t) => t,
			Err(e) => {
				error!("Error while deserializing theme file |{}|: |{}|", theme.get_path(), e);
				panic!("Could not deserialize theme file")
			}
		}
	}
	
	pub fn get_themes() -> Vec<ThemeFile> {
		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
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
			
			let path = match entry.path().to_str(){
				Some(path) => String::from(path),
				None => {
					error!("Error while converting path to String: |Invalid UTF-8 data|");
					continue;
				}
			};

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
	use super::*;
	#[test]
	fn test_get_themes() {
		let themes = Theme::get_themes();
		for theme in &themes {
			println!("Theme: {} in {}", theme.get_name(), theme.get_path())
		}

	}
}
