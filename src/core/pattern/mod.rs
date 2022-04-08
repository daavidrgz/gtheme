use std::collections::HashSet;
use std::fs::{self,File,metadata};
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;
use log::{info,error,warn};

use crate::core;
use crate::core::theme::Theme;
use crate::core::desktop::DesktopFile;
use crate::core::config::UserConfig;

#[derive(Debug)]
pub struct Pattern {
	name: String,
	path: String,
	output: Option<String>,
	content: String,
	submodules: Option<Vec<PatternFile>>
}
impl Pattern {
	//TODO: From str or from PatternFile??
	pub fn from(pattern: &PatternFile) -> Self {

		let pattern_path = Path::new(pattern.get_path());

		let metadata = match metadata(pattern_path){
			Ok(metadata) => metadata,
			Err(e) =>{
				error!("Could not read metadata from |{}|: |{}|",pattern.get_path(),e);
				return Self::default(pattern);
			}
		};

		if metadata.is_dir(){
			//Change submodules name to "pattern_name.submodule_name" instead of "submodule_name"
			let submodules = Self::get_patterns_from_path(pattern_path).into_iter()
				.map(|pattern_file| PatternFile{
					path: pattern_file.get_path().to_string(),
					name: pattern_file.get_name().to_string()
				}).collect();
			
			//TODO
			//Return content as empty string. Maybe better option<String>? lazy to do this change tho
			return Pattern{
				name:pattern.get_name().to_string(),
				path:pattern.get_path().to_string(),
				output:None,
				content:"".to_string(),
				submodules:Some(submodules)
			}
		}else if !metadata.is_file(){
			error!("Pattern |{}| from |{}|is not a directory nor a file",pattern.get_name(),pattern.get_path());
			return Self::default(pattern);
		}

		let mut file = match File::open(pattern_path){
			Ok(file) => file,
			Err(e) =>{
				error!("Could not open pattern |{}| from |{}|: |{}|",pattern.get_name(),pattern.get_path(),e);
				return Self::default(pattern);
			}
		};

		let re = Regex::new(r"<\[output-file\]>=(.*)(\r\n|\r|\n)").unwrap();

		let mut content = String::new();
		file.read_to_string(&mut content).expect(&format!("Error while reading pattern: {}", pattern.get_path()));

		match file.read_to_string(&mut content){
			Ok(_) => (),
			Err(e)=>{
				error!("Error while reading pattern |{}| from |{}|: |{}|",pattern.get_name(), pattern.get_path(),e);
				return Self::default(pattern);
			}
		}
		let output_path = match re.captures(&content) {
			Some(capture) => Some(core::expand_path(&capture[1])),
			None => None
		};
		content = String::from(re.replace(&content,""));

		Pattern {
			name: String::from(pattern.get_name()),
			path: String::from(pattern.get_path()),
			output: output_path,
			content,
			submodules:None
		}
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_path(&self) -> &String {
		&self.path
	}
	pub fn get_output(&self) -> &Option<String> 	{
		&self.output
	}
	pub fn has_submodules(&self)->bool{
		self.submodules.is_some()
	}
	pub fn get_submodules(&self) -> &Option<Vec<PatternFile>>{
		&self.submodules
	}
	fn default(pattern: &PatternFile) ->Self{
		Pattern {
			name: String::from(pattern.get_name()),
			path: String::from(pattern.get_path()),
			output: None,
			content:"".to_string(),
			submodules:None
		}
	}

	pub fn get_by_name(desktop: &DesktopFile, pattern: &str) -> Option<PatternFile> {
		let all_patterns = Pattern::get_patterns(desktop);
		match all_patterns.into_iter().find(|item|item.get_name().to_lowercase() == pattern.to_lowercase()) {
			Some(pattern) => Some(pattern),
			None => {
				error!("Pattern |{}| does not exist",pattern);
				None
			}
		}
	}

	pub fn get_patterns(desktop: &DesktopFile) -> Vec<PatternFile> {
		let patterns_dir = format!("{}/gtheme/patterns", desktop.get_path());
		let path = Path::new(&patterns_dir);
		return Self::get_patterns_from_path(path);
	}

	fn get_patterns_from_path(path: &Path) -> Vec<PatternFile> {
		let entries = match fs::read_dir(path) {
			Ok(dir) => dir,
			Err(e) => {
				error!("Could not read directory |{}|: |{}|", path.display(), e);
				return vec![]
			}
		};

		let mut vec = Vec::new();
		for entry in entries {
			let entry = match entry {
				Ok(entry) => entry,
				Err(e) => {
					error!("Error while reading entry from dir |{}|: |{}|", path.display(), e);
					continue;
				}
			};

			let file_name = match entry.file_name().into_string() {
				Ok(f) => f,
				Err(_) => {
					error!("Error while converting OsString to String: |Invalid unicode data|");
					continue;
				}
			};

			let path = entry.path().display().to_string();
			let md = match metadata(&path) {
				Ok(md) => md,
				Err(err) => {
					error!("Could not read metadata from |{}|: |{}|", path, err);
					continue;
				}
			};

			if file_name.starts_with(".") || (!file_name.ends_with(".pattern") && !md.is_dir()){
				//If it is a hidden file or it is a file/symlink without pattern extension
				continue;
			}
			let name = if md.is_dir() {
				//If it is a directory(i.e module pattern), get the name from the whole dir name
				file_name
			}else{
				//If it is a file, get name from splitting extension
				match file_name.rsplit_once(".pattern") {
					None => file_name,
					Some((prefix,_)) => String::from(prefix)
				}
			};

			vec.push(PatternFile { name, path });
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}

	pub fn fill(&self, theme: &Theme, is_inverted: bool, user_config: &UserConfig, dry_run: bool) {
		info!("Filling |{}| pattern with |{}| theme...", self.get_name(), theme.get_name());

		//If there are submodules
		if let Some(submodules )= self.get_submodules() {
			for submodule in submodules{
				submodule.to_pattern().fill(theme,is_inverted,user_config,dry_run);
			}
			return;
		}
		
		// if pattern has no submodules (i.e, is a file)
		let filled_content = self.fill_values(theme, is_inverted, user_config);

		let output_path = match self.get_output() {
			Some(output_path) => output_path,
			None => {
				error!("Pattern |{}| does not have output file specified (hint: <[output-file]>=/path/to/output/file)", self.get_name());
				return
			}
		};

		// Return if dry_run mode. i.e, dont write content to output path
		if dry_run { return }

		let path = std::path::Path::new(output_path);
		let prefix = path.parent().unwrap();
		// TODO: error handling
		fs::create_dir_all(prefix).unwrap();
		// If cant create output file, returns
		let mut output_file = match File::create(output_path) {
			Ok(file) => file,
			Err(e) => {
				error!("Could not create |{}|: |{}|", output_path, e);
				return;
			}
		};
		match output_file.write_all(filled_content.as_bytes()) {
			Ok(_) => (),
			Err(e) => {
				error!("Could not write to |{}|: |{}|", output_path, e);
				return;
			}
		}
	}

	fn fill_values(&self, theme: &Theme, is_inverted: bool, user_config: &UserConfig) -> String {
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

			let re = Regex::new(&format!(r"<\[{}\]>", real_key)).unwrap();
			result = re.replace_all(&result, value).into_owned();
		}

		let re = Regex::new(r"<\[theme-name\]>").unwrap();
		result = re.replace_all(&result, theme.get_name()).into_owned();

		// Fill user defined properties
		for (key,value) in user_config.get_properties() {
			let re = Regex::new(&format!(r"<\[({})(?:\|(.*))?\]>",key)).unwrap();
			result = re.replace_all(&result, value).into_owned();
		}
	
		// Find not filled properties
		let mut missing_properties = HashSet::new();
		let mut default_properties = HashSet::new();

		let re = Regex::new(r"<\[((?:\w|-)+)(?:\|(.*))?\]>").unwrap();

		let mut default_filled_result = result.clone();
		for caps in re.captures_iter(&result){
			let property = match caps.get(1) {
				None => {
					//This warning should not happen, since property name captured group
					//is not optional, hence a string <[]> does not match in the regex and 
					//can't enter in this branch
					warn!("There is an empty property (<[]>) in pattern |{}|", self.get_name());
					continue
				},
				Some(value) => String::from(value.as_str())
			};
			let default = caps.get(2);
			match default {
				None => {missing_properties.insert(property);},
				Some(value) => {
					let default_value = String::from(value.as_str());
					// If pair property-default value were not replaced before
					if default_properties.insert((property.clone(),default_value.clone())){
						let re = Regex::new(&format!(r"<\[{}\|{}\]>", property, default_value)).unwrap();
						default_filled_result = re.replace_all(&default_filled_result, default_value).into_owned();
					}
				}
			};
		};
		for missing_property in missing_properties {
			warn!("Could not fill property |{}| in pattern |{}|", missing_property, self.get_name());
		}
		for (default_property,value) in default_properties {
			info!("Filled property |{}| with default value |{}| in pattern |{}|", default_property, value, self.get_name());
		}
		default_filled_result
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

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn test_regex_fill(){
		let content = "<[]>";
		let re = Regex::new(r"<\[((?:\w|-)+)?(?:\|(.*))?\]>").unwrap();

		println!("{}", re.is_match(content));
		for caps in re.captures_iter(content){
			// println!("{}",String::from(&caps[1]));
			// println!("{}",String::from(&caps[3]));
			// let cap =caps.get(1);
			dbg!(caps);
		};		
	}

	#[test]
	fn test_subpatterns(){
		let subpattern = "test.test2.subsubpattern";
		dbg!(subpattern.split('.').collect::<Vec<&str>>());
	}


}
