use std::fs::{self};

use crate::core;

#[derive(Debug)]
pub struct PostScript{
	name:String,
	path:String,
}
impl PostScript{
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
	pub fn get_postscripts(desktop:&str)->Vec<PostScript>{
		let gtheme_home:String= core::expand_path("~/github/gtheme");
		let patterns_dir = gtheme_home + &format!("/desktops/{}/gtheme/patterns",desktop);
		let entries = fs::read_dir(&patterns_dir).expect(&format!("Could not read directory:{}",&patterns_dir));

		let mut vec = Vec::new();
		for entry in entries{
			let entry = entry.expect(&format!("Error while reading entry from dir: {}",&patterns_dir));
			let file_name =entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			let name = match file_name.rsplit_once("."){
				None => panic!("Error while splitting file name:{}",file_name),
				Some((prefix,_))=>String::from(prefix)
			};
			vec.push(PatternFile{name,path});
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}
}
