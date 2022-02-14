use std::fs::{self,File};
use std::io::prelude::*;
use regex::Regex;

use crate::core;
use crate::core::theme::Theme;

#[derive(Debug)]
pub struct Pattern{
	name:String,
	path:String,
	output:String,
	content:String
}
impl Pattern{
	//TODO: From str or from PatternFile??
	pub fn from(pattern: &PatternFile)->Self{
		let re = Regex::new("%output-file%=(.*)").unwrap();
		let mut file = File::open(pattern.get_path()).expect(&format!("Error while opening pattern: {}",pattern.get_path()));
		let mut content = String::new();
		file.read_to_string(&mut content).expect(&format!("Error while reading pattern: {}",pattern.get_path()));

		assert!(re.is_match(&content),"Pattern does not have output file specified (hint: %output-file%=/path/to/output/file)");
		let captured = re.captures(&content).unwrap();
		//captured[0] is the whole matched expression.
		let output_path = core::expand_path(&captured[1]);

		//Delete where output-file is declared.
		let re = Regex::new("%output-file%=.*").unwrap();
		content = String::from(re.replace(&content,""));

		Pattern{
			name:String::from(pattern.get_name()),
			path: String::from(pattern.get_path()),
			output:output_path,
			content
		}
	}
	pub fn get_name(&self)->&String{
		&self.name
	}
	pub fn get_path(&self)->&String{
		&self.path
	}
	pub fn get_output(&self)->&String{
		&self.output
	}
	//TODO: use DesktopFile or str?
	pub fn get_patterns(desktop:&str)->Vec<PatternFile>{
		let gtheme_home:String= core::expand_path(core::GTHEME_HOME);
		let patterns_dir = gtheme_home + &format!("/desktops/{}/gtheme/patterns",desktop);
		let entries = fs::read_dir(&patterns_dir).expect(&format!("Could not read directory:{}",&patterns_dir));

		let mut vec = Vec::new();
		for entry in entries{
			let entry = entry.expect(&format!("Error while reading entry from dir: {}",&patterns_dir));
			let file_name =entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			let name = match file_name.rsplit_once("."){
				None => file_name,
				Some((prefix,_))=>String::from(prefix)
			};
			vec.push(PatternFile{name,path});
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}
	pub fn fill(&self,theme:&Theme,is_inverted:bool){
		let filled_content =self.fill_values(theme,is_inverted);
	
		let mut output_file = File::create(self.get_output()).expect(&format!("Could not create file: {}",self.get_output()));
		output_file.write_all(filled_content.as_bytes()).expect(&format!("Could not write content to: {}",self.get_output()));
	}
	fn fill_values(&self,theme:&Theme,is_inverted:bool) -> String{
	
		let mut result = String::from(&self.content);
		for (key,value) in theme.colors.iter(){

			let real_key = if is_inverted {
				match key.as_str(){
					"foreground"=> "background",
					"background"=>"foreground",
					"selection-foreground"=>"selection-background",
					"selection-background"=>"selection-foreground",
					_=>key
				}
			}else{
				key
			};

			let re = Regex::new(&format!("%{}%",real_key)).unwrap();
			result = re.replace_all(&result,value).into_owned();
		}
		result
	}
	//TODO: delete patterns function for a given directory?
}

#[derive(Debug)]
pub struct PatternFile{
	name:String,
	path:String,
}
impl PatternFile{
	pub fn to_pattern(&self)->Pattern{
		Pattern::from(self)
	}
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
	fn test_get_patterns(){
		let patterns = Pattern::get_patterns("simple");
		for pattern in &patterns{
			println!("Pattern: {} in {}",pattern.get_name(),pattern.get_path())
		}
	}
}
