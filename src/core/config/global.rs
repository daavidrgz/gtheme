

use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use crate::core::{self,desktop::{Desktop,DesktopFile},theme::{Theme,ThemeFile}};

#[derive(Debug,Serialize,Deserialize)]
struct GlobalConfigDto{
	current_desktop:Option<String>,
	current_theme:Option<String>,
	fav_themes:Vec<String>,
}

#[derive(Debug)]
pub struct GlobalConfig{
	current_desktop:Option<DesktopFile>,
	current_theme:Option<ThemeFile>,
	fav_themes:Vec<ThemeFile>
}

impl GlobalConfigDto{
	fn new() -> Self {
		let mut file = File::open(format!("{}/global_config.json",core::expand_path(core::GTHEME_HOME))).unwrap();
		let mut content = String::new();
		file.read_to_string(&mut content).expect("Could not read theme file");
		serde_json::from_str(&content).expect("Error while deserializing theme file")
	}
	fn from(config:&GlobalConfig)->Self{

		let current_desktop = match config.get_current_desktop(){
			None=>None,
			Some(desktop)=>Some(String::from(desktop.get_name())),
		};

		let current_theme = match config.get_current_theme(){
			None=>None,
			Some(theme)=>Some(String::from(theme.get_name())),
		};

		let fav_themes = config.get_fav_themes().iter().map(|theme| String::from(theme.get_name())).collect();

		GlobalConfigDto{
			current_desktop,
			current_theme,
			fav_themes
		}
	}
	fn save(&self){
		let content = serde_json::to_string(self).unwrap();
		let mut file = File::open(format!("{}/global_config.json",core::expand_path(core::GTHEME_HOME))).unwrap();
		file.write_all(&content.as_bytes()).expect("Error while saving config file");
	}
	fn get_current_desktop(&self)->&Option<String>{
		&self.current_desktop
	}
	fn get_current_theme(&self)->&Option<String>{
		&self.current_theme
	}
	fn get_fav_themes(&self)->&Vec<String>{
		&self.fav_themes
	}
	
}

impl GlobalConfig{
	pub fn new()->Self{
		let dto = GlobalConfigDto::new();
		let desktops = Desktop::get_desktops();
		let themes = Theme::get_themes();

		let fav_themes_string = dto.get_fav_themes();

		let current_desktop = match dto.get_current_desktop(){
			None=>None,
			Some(desktop)=>desktops.into_iter().find(|item| item.get_name() == desktop)		
		};
		let current_theme = match dto.get_current_theme(){
			None=>None,
			Some(theme)=>themes.clone().into_iter().find(|item| item.get_name() == theme)		
		};

		let fav_themes = themes.into_iter().filter(|item| fav_themes_string.contains(item.get_name())).collect();

		GlobalConfig{
			current_desktop,
			current_theme,
			fav_themes
		}
	}
	pub fn save(&self){
		GlobalConfigDto::from(self).save()
	}

	pub fn get_current_desktop(&self)->&Option<DesktopFile>{
		&self.current_desktop
	}
	pub fn get_mut_current_desktop(&mut self)->&mut Option<DesktopFile>{
		&mut self.current_desktop
	}

	pub fn get_current_theme(&self)->&Option<ThemeFile>{
		&self.current_theme
	}
	pub fn get_mut_current_theme(&mut self)->&mut Option<ThemeFile>{
		&mut self.current_theme
	}

	pub fn get_fav_themes(&self)->&Vec<ThemeFile>{
		&self.fav_themes
	}
	pub fn get_mut_fav_themes(&mut self)->&mut Vec<ThemeFile>{
		&mut self.fav_themes
	}
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_config(){
		let mut config = GlobalConfig::new();

		println!("Before: {:?}",&config);
		*config.get_mut_current_desktop() = None;
		*config.get_mut_fav_themes()=vec![];

		println!("After: {:?}",&config);

	}
}