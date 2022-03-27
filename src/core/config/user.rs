use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use toml;
use log::{info,warn,error};
use std::path::Path;
use crate::core;

#[derive(Debug, Serialize, Deserialize)]
struct UserConfigDto {
	properties:HashMap<String,String>,
}

#[derive(Debug)]
pub struct UserConfig {
	properties:HashMap<String,String>,
}
 
impl UserConfigDto {
	fn new() -> Self {
		let path = format!("{}/user_settings.toml",core::expand_path(core::GTHEME_HOME));
		let mut file = match File::open(&path) {
			Ok(file) => file,
			Err(e) => {
				warn!("Could not open user settings, using default settings: |{}|", e);
				let config =  Self::default();
				config.save();
				return config
			}
		};
		let mut content = String::new();
		match  file.read_to_string(&mut content) {
			Ok(_) => (),
			Err(e) => {
				error!("Could not read user settings, using default settings: |{}|", e);
				let config =  Self::default();
				config.save();
				return config;
			}
		};
		match toml::from_str(&content){
			Ok(config) => {
				info!("Using user settings |{}|",&path);
				config
			},
			Err(e) => {
				error!("Could not parse user settings, using default settings: |{}|", e);
				return Self::default()
			}
		}
	}

	fn from(config:&UserConfig) -> Self {
		UserConfigDto {
			properties: config.properties.clone(),
		}
	}

	fn save(&self) {
		let content = toml::to_string_pretty(self).unwrap();
		let mut splitted:Vec<&str> = content.trim().split("\n").collect();

		let mut to_order:Vec<&str> = splitted.drain(1..).collect();
		to_order.sort_by(|a,b| a.cmp(b));

		let content = splitted.into_iter().chain(to_order.into_iter())
			.map(|e|e.to_string()).collect::<Vec<String>>().join("\n");

		let path = format!("{}/user_settings.toml",core::expand_path(core::GTHEME_HOME));
		let mut file = match OpenOptions::new().create(true).write(true).truncate(true).open(&path) {
			Ok(f) => f,
			Err(e) => {
				error!("Could not open |{}|: |{}|", &path, e);
				return;
			}
		};
		match file.write_all(&content.as_bytes()){
			Err(e) => error!("Could not write user settings in |{}|: |{}|", &path, e),
			_=> info!("Saving user settings...")
		}	
	}
}

impl Default for UserConfigDto {
	fn default() -> UserConfigDto {
		UserConfigDto {
			properties:HashMap::new(),
		}
	}
}

impl UserConfig {
	pub fn new() -> Self {
		let dto = UserConfigDto::new();

		UserConfig {
			properties:dto.properties,
		}
	}

	pub fn save(&self) {
		UserConfigDto::from(self).save()
	}
	pub fn set_properties(&mut self, property:&str,value:&str){
		self.properties.insert(String::from(property),String::from(value));
	}
	pub fn set_property(&mut self, property:&str,value:&str){
		self.properties.insert(String::from(property),String::from(value));
	}
	pub fn get_properties(&self) -> &HashMap<String,String>{
		&self.properties
	}
	pub fn exists() -> bool {
		let path = format!("{}/user_settings.toml",core::expand_path(core::GTHEME_HOME));
		Path::new(&path).exists()
	}
}
impl Default for UserConfig{
	fn default() -> Self{
		UserConfig{
			properties:HashMap::new(),
		}
	}
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_config(){
		let mut config = UserConfig::new();
		config.set_properties("monitor", "HDMI-0");
		config.save();
	}
}
