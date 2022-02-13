use std::collections::HashMap;
use std::fs::{self,File};
use std::io::prelude::*;
use regex::Regex;
use serde::{Serialize,Deserialize};

use crate::core;
use crate::core::pattern::Pattern;


#[derive(Debug,Serialize,Deserialize)]
pub struct Theme{
	pub name:String,
	pub vscode:String,
	pub wallpaper:String,
	pub colors:HashMap<String,String>
}

impl Theme{

	//TODO: from str or from ThemeFile??
	pub fn from(path:&str)-> Self{
		let mut file = File::open(path).expect("Could not open theme file");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("Could not read theme file");
		serde_json::from_str(&content).expect("Error while deserializing theme file")
	}
	pub fn fill_pattern(&self,pattern:&Pattern){
		let mut pattern_file = File::open(pattern.get_path()).expect(&format!("Could not open file: {}",pattern.get_path()));
	
		let mut content = String::new();
		pattern_file.read_to_string(&mut content).expect(&format!("Could not read content from: {}",pattern.get_path()));
		
		let filled_content =self.fill(content);
	
		let mut output_file = File::create(pattern.get_output()).expect(&format!("Could not create file: {}",pattern.get_output()));
		output_file.write_all(filled_content.as_bytes()).expect(&format!("Could not write content to: {}",pattern.get_output()));
	}
	fn fill(&self,content:String) -> String{
	
		let mut result = content;
		for (key,value) in self.colors.iter(){
			let re = Regex::new(&format!("%{}%",key)).unwrap();
			result = re.replace_all(&result,value).into_owned();
		}
		result
	}
	pub fn get_themes()->Vec<ThemeFile>{
		let gtheme_home:String= core::expand_path("~/github/gtheme");
		let themes_dir = gtheme_home+"/themes";
		let entries = fs::read_dir(&themes_dir).expect(&format!("Could not read directory:{}",&themes_dir));

		// let entries_str:Vec<String> = entries.map(|entry|entry.unwrap().file_name().into_string().unwrap()).collect();

		// entries_str
		let mut vec = Vec::new();
		for entry in entries{
			let entry = entry.expect(&format!("Error while reading entry from dir: {}",&themes_dir));
			let file_name =entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			let name = match file_name.rsplit_once("."){
				None => panic!("Error while splitting file name"),
				Some((prefix,_))=>String::from(prefix)
			};
			vec.push(ThemeFile{name,path});
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}
	
}

#[derive(Debug)]
pub struct ThemeFile{
	name:String,
	path:String,
}
impl ThemeFile{
	pub fn get_name(&self) ->&String{
		&self.name
	}

	pub fn get_path(&self) ->&String{
		&self.path
	}

}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn test_get_themes(){
		let themes = Theme::get_themes();
		for theme in &themes{
			println!("Theme: {} in {}",theme.get_name(),theme.get_path())
		}

	}
}