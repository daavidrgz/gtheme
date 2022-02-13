
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Theme{
	pub vscode:String,
	pub wallpaper:String,
	pub colors:HashMap<String,String>
}
impl Theme{
	pub fn from(path:&str)-> Self{
		let mut file = File::open(path).expect("Could not open theme file");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("Could not read theme file");
		serde_json::from_str(&content).expect("Error while deserializing theme file")
	}
	pub fn fill_pattern(&self,pattern:&Pattern){
		let mut pattern_file = File::open(&pattern.path).expect(&format!("Could not open file: {}",&pattern.path));
	
		let mut content = String::new();
		pattern_file.read_to_string(&mut content).expect(&format!("Could not read content from: {}",&pattern.path));
		
		let filled_content =self.fill(content);
	
		let mut output_file = File::create(&pattern.output).expect(&format!("Could not create file: {}",&pattern.output));
		output_file.write_all(filled_content.as_bytes()).expect(&format!("Could not write content to: {}",&pattern.output));
	}
	fn fill(&self,content:String) -> String{
	
		let mut result = content;
		for (key,value) in self.colors.iter(){
			let re = Regex::new(&format!("%{}%",key)).unwrap();
			result = re.replace_all(&result,value).into_owned();
		}
		result
	}
}

#[derive(Debug)]
pub struct Pattern{
	pub path:String,
	pub output:String
}
impl Pattern{
	pub fn from(path:&str)->Self{
		let re = Regex::new("%output-file%=(.*)").unwrap();
		let mut file = File::open(path).unwrap();
		let mut content = String::new();
		file.read_to_string(&mut content).unwrap();
		assert!(re.is_match(&content),"Pattern does not have output file specified (hint: %output-file%=/path/to/output/file)");
		let captured = re.captures(&content).unwrap();
		//captured[0] is the whole matched expression.
		Pattern{
			path: String::from(path),
			output:String::from(&captured[1])
		}
	}
}



#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn test(){
		let pattern = Pattern::from("./kitty.pattern");
		let theme = Theme::from("./themes/Vue.json");
		theme.fill_pattern(&pattern);
	}
}
