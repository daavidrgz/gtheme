use std::fs::{self,File};
use std::io::prelude::*;
use regex::Regex;

use crate::core;


#[derive(Debug)]
pub struct Pattern{
	path:String,
	output:String
}
impl Pattern{
	//TODO: From str or from PatternFile??
	pub fn from(path:&str)->Self{
		let re = Regex::new("%output-file%=(.*)").unwrap();
		let mut file = File::open(path).expect(&format!("Error while opening pattern: {}",path));
		let mut content = String::new();
		file.read_to_string(&mut content).expect(&format!("Error while reading pattern: {}",path));

		assert!(re.is_match(&content),"Pattern does not have output file specified (hint: %output-file%=/path/to/output/file)");
		let captured = re.captures(&content).unwrap();
		//captured[0] is the whole matched expression.
		let output_path = core::expand_path(&captured[1]);
		
		Pattern{
			path: String::from(path),
			output:output_path
		}
	}
	pub fn get_path(&self)->&String{
		&self.path
	}
	pub fn get_output(&self)->&String{
		&self.output
	}
	pub fn get_patterns(desktop:&str)->Vec<PatternFile>{
		let gtheme_home:String= core::expand_path("~/github/gtheme");
		let patterns_dir = gtheme_home + &format!("/desktops/{}/gtheme/patterns",desktop);
		let entries = fs::read_dir(&patterns_dir).expect(&format!("Could not read directory:{}",&patterns_dir));

		// let entries_str:Vec<String> = entries.map(|entry|entry.unwrap().file_name().into_string().unwrap()).collect();

		// entries_str
		let mut vec = Vec::new();
		for entry in entries{
			let entry = entry.expect(&format!("Error while reading entry from dir: {}",&patterns_dir));
			let file_name =entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			let name = match file_name.rsplit_once("."){
				None => panic!("Error while splitting file name"),
				Some((prefix,_))=>String::from(prefix)
			};
			vec.push(PatternFile{name,path});
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}
	//TODO: delete patterns function for a given directory?
}

#[derive(Debug)]
pub struct PatternFile{
	name:String,
	path:String,
}
impl PatternFile{
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
