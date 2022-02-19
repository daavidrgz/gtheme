use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use fs_extra::file;
use serde::{Serialize,Deserialize};
use crate::core::{self,desktop::{Desktop,DesktopFile},theme::{Theme,ThemeFile}};

#[derive(Debug, Serialize, Deserialize)]
struct DesktopConfigDto {
	default_theme:Option<String>,
	actived: HashMap<String,bool>,
	inverted: HashMap<String,bool>,
}

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
	fn save(&self,desktop:&str){
		let content = serde_json::to_string(self)
.unwrap();
		let path = format!("{}/desktops/{}/desktop_config.json",core::expand_path(core::GTHEME_HOME),desktop);
        let mut file = OpenOptions::new().write(true).truncate(true).open(path).expect("Could not open global config file with write permissions");
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
	pub fn save(&self){

	}
}