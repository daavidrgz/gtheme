use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use crate::core::{self};
use log::{info,warn,error};

#[derive(Debug, Serialize, Deserialize)]
struct UserConfigDto {
	mandatory:HashMap<String,String>,
	extra:HashMap<String,String>
}

#[derive(Debug)]
pub struct UserConfig {
	mandatory:HashMap<String,String>,
	extra:HashMap<String,String>
}
 
impl UserConfigDto {
	fn new() -> Self {
		let path = format!("{}/user_config.json",core::expand_path(core::GTHEME_HOME));
		let mut file = match File::open(&path) {
			Ok(file) => file,
			Err(e) => {
				warn!("Could not open user config, using default config: |{}|", e);
				return Self::default()
			}
		};
		let mut content = String::new();
		match  file.read_to_string(&mut content) {
			Ok(_) => (),
			Err(e) => {
				error!("Could not read user config, using default config: |{}|", e);
				return Self::default()
			}
		};
		match serde_json::from_str(&content){
			Ok(config) => {
				info!("Using user config |{}|",&path);
				config
			},
			Err(e) => {
				error!("Could not parse user config, using default config: |{}|", e);
				return Self::default()
			}
		}
	}

	fn from(config:&UserConfig) -> Self {
		UserConfigDto {
			mandatory: config.mandatory.clone(),
			extra:config.extra.clone()
		}
	}

	fn save(&self) {
		let content = serde_json::to_string_pretty(self).unwrap();
		let path = format!("{}/user_config.json",core::expand_path(core::GTHEME_HOME));
		let mut file = match OpenOptions::new().create(true).write(true).truncate(true).open(&path) {
			Ok(f) => f,
			Err(e) => {
				error!("Could not open |{}|: |{}|", &path, e);
				return;
			}
		};
		match file.write_all(&content.as_bytes()){
			Err(e) => error!("Could not write user config in |{}|: |{}|", &path, e),
			_=> info!("Saving user config...")
		}	
	}

}

impl Default for UserConfigDto {
	fn default() -> UserConfigDto {
		UserConfigDto {
			mandatory:HashMap::new(),
			extra:HashMap::new()
		}
	}
}

impl UserConfig {
	pub fn new() -> Self {
		let dto = UserConfigDto::new();

		UserConfig {
			mandatory:dto.mandatory,
			extra:dto.extra
		}
	}

	pub fn save(&self) {
		UserConfigDto::from(self).save()
	}
	pub fn set_mandatory(&mut self, property:&str,value:&str){
		self.mandatory.insert(String::from(property),String::from(value));
	}
	pub fn set_extra(&mut self, property:&str,value:&str){
		self.extra.insert(String::from(property),String::from(value));
	}
	pub fn get_mandatory(&self) -> &HashMap<String,String>{
		&self.mandatory
	}
	pub fn get_extra(&self) -> &HashMap<String, String> {
		&self.extra
	}
	fn check_mandatory(&self){
		let path = format!("{}/user_config.json",core::expand_path(core::GTHEME_HOME));

		if let None = self.mandatory.get("monitor"){
			warn!("Monitor property is not set on user config |({})|",path)
		}
	}
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn test_config(){
		let mut config = UserConfig::new();
		config.set_mandatory("monitor", "HDMI-0");
		config.save();
	}
}
