use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use crate::core::{self};
use log::{info,warn,error};

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
		let path = format!("{}/user_config.json",core::expand_path(core::GTHEME_HOME));
		let mut file = match File::open(&path) {
			Ok(file) => file,
			Err(e) => {
				warn!("Could not open user config, using default config: |{}|", e);
				let config =  Self::default();
				config.save();
				return config
			}
		};
		let mut content = String::new();
		match  file.read_to_string(&mut content) {
			Ok(_) => (),
			Err(e) => {
				error!("Could not read user config, using default config: |{}|", e);
				let config =  Self::default();
				config.save();
				return config;
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
			properties: config.properties.clone(),
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
