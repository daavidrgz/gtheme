
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Theme{
	vscode:String,
	wallpaper:String,
	colors:HashMap<String,String>
}
impl Theme{
	pub fn from(path:&str)-> Self{
		let mut file = File::open(path).expect("Could not open theme file");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("Could not read theme file");
		serde_json::from_str(&content).expect("Error while deserializing theme file")
	}
	pub fn fill_pattern(&self,pattern:&str,output:&str){
		let mut pattern_file = File::open(pattern).expect(&format!("Could not open file: {}",pattern));
	
		let mut content = String::new();
		pattern_file.read_to_string(&mut content).expect(&format!("Could not read content from: {}",pattern));
		
		let filled_content =self.fill(content);
	
		let mut output_file = File::create(output).expect(&format!("Could not create file: {}",output));
		output_file.write_all(filled_content.as_bytes()).expect(&format!("Could not write content to: {}",output));
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



#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn test(){
		let path = "./themes/Nord.json";
		let mut file = File::open(path).unwrap();
		let mut content = String::new();
		file.read_to_string(&mut content).unwrap();
		let theme:Theme = serde_json::from_str(&content).unwrap();
		println!("{:?}",theme);
	}
}