use std::fs::{self,File};
use std::io::prelude::*;
use regex::Regex;
use log::{info,error};

use crate::core;
use crate::core::theme::Theme;
use crate::core::desktop::DesktopFile;

#[derive(Debug)]
pub struct Pattern {
	name: String,
	path: String,
	output: String,
	content: String
}
impl Pattern {
	//TODO: From str or from PatternFile??
	pub fn from(pattern: &PatternFile) -> Self {
		let re = Regex::new("%output-file%=(.*)").unwrap();
		let mut file = File::open(pattern.get_path()).expect(&format!("Error while opening pattern: {}", pattern.get_path()));

		let mut content = String::new();
		file.read_to_string(&mut content).expect(&format!("Error while reading pattern: {}", pattern.get_path()));

		if !re.is_match(&content){
			panic!("Pattern {} does not have output file specified (hint: %output-file%=/path/to/output/file)",pattern.get_path());
		}
		let captured = re.captures(&content).unwrap();
		//captured[0] is the whole matched expression.
		let output_path = core::expand_path(&captured[1]);

		//Delete where output-file is declared.
		let re = Regex::new("%output-file%=.*").unwrap();
		content = String::from(re.replace(&content,""));

		Pattern {
			name: String::from(pattern.get_name()),
			path: String::from(pattern.get_path()),
			output: output_path,
			content
		}
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_path(&self) -> &String {
		&self.path
	}
	pub fn get_output(&self) -> &String 	{
		&self.output
	}

	pub fn get_by_name(desktop:&DesktopFile,pattern:&str) -> Option<PatternFile>{
		let all_patterns = Pattern::get_patterns(desktop);
		match all_patterns.into_iter().find(|item|item.get_name().to_lowercase()==pattern.to_lowercase()){
			Some(pattern)=>Some(pattern),
			None=>{
				error!("Pattern |{}| does not exist",pattern);
				None
			}
		}
	}

	pub fn get_patterns(desktop: &DesktopFile) -> Vec<PatternFile> {
		
		let patterns_dir = format!("{}/gtheme/patterns", desktop.get_path());
		let entries = match fs::read_dir(&patterns_dir) {
			Ok(dir) => dir,
			Err(e) => {
				error!("Could not read directory |{}|: |{}|", &patterns_dir, e);
				return vec![]
			}
		};

		let mut vec = Vec::new();
		for entry in entries {
			let entry = match entry {
				Ok(entry) => entry,
				Err(e) => {
					error!("Error while reading entry from dir |{}|: |{}|", &patterns_dir, e);
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

			let path = match entry.path().to_str() {
				Some(path) => String::from(path),
				None =>{
					error!("Error while converting path to String: |Invalid UTF-8 data|");
					continue;
				}
			};

			let name = match file_name.rsplit_once(".") {
				None => file_name,
				Some((prefix,_)) => String::from(prefix)
			};

			vec.push(PatternFile { name, path });
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}

	pub fn fill(&self, theme: &Theme, is_inverted: bool) {
		info!("Filling |{}| pattern with |{}| theme...", self.get_name(), theme.get_name());

		let filled_content = self.fill_values(theme, is_inverted);
	
		//If cant create output file, returns
		let mut output_file = match File::create(self.get_output()) {
			Ok(file) => file,
			Err(e) => {
				error!("Could not create |{}|: |{}|", self.get_output(), e);
				return;
			}
		};
		match output_file.write_all(filled_content.as_bytes()) {
			Ok(_) => (),
			Err(e) => {
				error!("Could not write to |{}|: |{}|", self.get_output(), e);
				return;
			}
		}
	}

	fn fill_values(&self, theme: &Theme, is_inverted: bool) -> String {
		let mut result = String::from(&self.content);
		for (key,value) in theme.get_colors().iter() {
			let real_key = if is_inverted {
				match key.as_str() {
					"foreground" => "background",
					"background" => "foreground",
					"selection-foreground" => "selection-background",
					"selection-background" => "selection-foreground",
					_ => key
				}
			} else {
				key
			};

			let re = Regex::new(&format!("%{}%", real_key)).unwrap();
			result = re.replace_all(&result, value).into_owned();
		}
		result
	}
}

#[derive(Debug,Clone)]
pub struct PatternFile {
	name: String,
	path: String,
}
impl PatternFile {
	pub fn to_pattern(&self) -> Pattern {
		Pattern::from(self)
	}
	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_path(&self) -> &String {
		&self.path
	}
}

// #[cfg(test)]
// mod tests{
// 	use super::*;
// 	#[test]
// 	fn test_get_patterns() {
// 		let patterns = Pattern::get_patterns("simple");
// 		for pattern in &patterns {
// 			println!("Pattern: {} in {}", pattern.get_name(), pattern.get_path())
// 		}
// 	}
// }
