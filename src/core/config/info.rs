use std::fs::{File};
use serde::{Serialize,Deserialize};
use toml;
use log::{info,warn,error};
use crate::core::desktop::DesktopFile;

use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DesktopInfo {
	author:String,
	description:String,
	dependencies:Vec<String>,
	credits:String
}


 
impl DesktopInfo {
	pub fn new(desktop: &DesktopFile) -> Self {
		let path = format!("{}/info.toml",desktop.get_path());

		let mut file = match File::open(&path) {
			Ok(file) => file,
			Err(e) => {
				warn!("Could not open desktop info, using default info: |{}|", e);
				return Self::default()
			}
		};
		let mut content = String::new();
		match  file.read_to_string(&mut content) {
			Ok(_) => (),
			Err(e) => {
				error!("Could not read desktop info, using default info: |{}|", e);
				return Self::default();
			}
		};
		match toml::from_str(&content){
			Ok(info) => {
				info!("Using desktop info |{}|",&path);
				info
			},
			Err(e) => {
				error!("Could not parse desktop info, using default info: |{}|", e);
				return Self::default()
			}
		}
	}
	pub fn get_author(&self) ->&String{
		&self.author
	}
	pub fn get_description(&self) ->&String{
		&self.description
	}
	pub fn get_dependencies(&self) ->&Vec<String>{
		&self.dependencies
	}
	pub fn get_credits(&self) ->&String{
		&self.credits
	}
}
impl Default for DesktopInfo{
	fn default() -> Self{
		DesktopInfo{
			author:"".to_string(),
			description:"".to_string(),
			dependencies:vec![],
			credits:"".to_string(),
		}
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn test_info(){
		let desktop = crate::core::desktop::Desktop::get_by_name("simple").unwrap();
		let info = DesktopInfo::new(&desktop);
		dbg!(info);

	}
}