use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use crate::core::{self,theme::{Theme,ThemeFile}};

#[derive(Debug, Serialize, Deserialize)]
struct DesktopConfigDto {
	default_theme:Option<String>,
	actived: HashMap<String,bool>,
	inverted: HashMap<String,bool>,
}

#[derive(Debug,Clone)]
pub struct DesktopConfig {
	desktop:String,
	default_theme:Option<ThemeFile>,
	actived: HashMap<String,bool>,
	inverted: HashMap<String,bool>,
}

impl DesktopConfigDto {
	fn new(desktop:&str) -> DesktopConfigDto {
		let path = format!("{}/desktops/{}/desktop_config.json",core::expand_path(core::GTHEME_HOME),desktop);
		let mut file = match File::open(path){
			Ok(file)=>file,
			_ => return Self::default()
		};
		let mut content = String::new();
		match  file.read_to_string(&mut content){
			Ok(_)=>(),
			_ => return Self::default()
		};
		match serde_json::from_str(&content){
			Ok(config) => config,
			_ => Self::default()
		}
	}
	fn from(config:&DesktopConfig)->Self{
		let default_theme = match config.get_default_theme(){
			None=>None,
			Some(theme)=>Some(String::from(theme.get_name()))
		};

		DesktopConfigDto{
			default_theme,
			actived:config.get_actived().clone(),
			inverted:config.get_inverted().clone()
		}
	}
	fn save(&self,desktop:&str){
		if desktop=="" {return}
		let content = serde_json::to_string(self).unwrap();
		let path = format!("{}/desktops/{}/desktop_config.json",core::expand_path(core::GTHEME_HOME),desktop);
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path).expect("Could not open desktop config file with write permissions");
        file.write_all(&content.as_bytes()).expect("Error while saving config file");
	}
}

impl Default for DesktopConfigDto{
	fn default() ->DesktopConfigDto{
		DesktopConfigDto{
			default_theme:None,
			actived:HashMap::new(),
			inverted: HashMap::new()
		}
	}
}

impl DesktopConfig{
	pub fn new(desktop:&str)->Self{
		let dto = DesktopConfigDto::new(desktop);
		let themes = Theme::get_themes();
		let default_theme = match dto.default_theme{
			None=>None,
			Some(theme_name)=> themes.into_iter().find(|theme| *theme.get_name() ==theme_name)
		};
		DesktopConfig{
			desktop:String::from(desktop),
			default_theme,
			actived:dto.actived,
			inverted:dto.inverted
		}

	}
	pub fn get_default_theme(&self)->&Option<ThemeFile>{
		&self.default_theme
	}
	pub fn get_mut_default_theme(&mut self)->&mut Option<ThemeFile>{
		&mut self.default_theme
	}
	pub fn get_actived(&self)->&HashMap<String, bool>{
		&self.actived
	}
	pub fn get_mut_actived(&mut self)->&mut HashMap<String, bool>{
		&mut self.actived
	}
	pub fn get_inverted(&self)->&HashMap<String, bool>{
		&self.inverted
	}
	pub fn get_mut_inverted(&mut self)->&mut HashMap<String, bool>{
		&mut self.inverted
	}
	pub fn save(&self){
		DesktopConfigDto::from(self).save(&self.desktop)
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn test_desktop_config(){
		let config = DesktopConfig::new("jorge");

		config.save();
		println!("{:?}",config);
	}
}