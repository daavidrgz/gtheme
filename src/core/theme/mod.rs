use std::collections::HashMap;
use std::fs::{self,File};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};

use crate::core;


#[derive(Debug,Serialize,Deserialize)]
pub struct Theme {
	//TODO: getters
	name: String,
	vscode: String,
	wallpaper: String,
	colors: HashMap<String, String>
}

impl Theme {

	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_wallpaper(&self) -> &String {
		&self.wallpaper
	}
	pub fn get_colors(&self) -> &HashMap<String, String> {
		&self.colors
	}

	//TODO: from str or from ThemeFile??
	pub fn from(theme: &ThemeFile) -> Self {
		let mut file = File::open(theme.get_path()).expect("Could not open theme file");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("Could not read theme file");
		serde_json::from_str(&content).expect("Error while deserializing theme file")
	}
	
	pub fn get_themes() -> Vec<ThemeFile> {
		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
		let themes_dir = gtheme_home + "/themes";
		let entries = fs::read_dir(&themes_dir).expect(&format!("Could not read directory:{}", &themes_dir));

		let mut vec = Vec::new();
		for entry in entries {
			let entry = entry.expect(&format!("Error while reading entry from dir: {}", &themes_dir));
			let file_name = entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			let name = match file_name.rsplit_once(".") {
				None => file_name,
				Some((prefix,_)) => String::from(prefix)
			};
			vec.push(ThemeFile { name, path });
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}

	//TODO: inverted theme
}

#[derive(Debug,Clone)]
pub struct ThemeFile {
	name: String,
	path: String,
}
impl ThemeFile{
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
