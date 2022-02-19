use std::fs::{self};
use std::process::{Command,Stdio};
use std::collections::HashMap;

use crate::core;

#[derive(Debug,Clone)]
pub struct PostScript {
	name: String,
	path: String,
}
impl PostScript{
	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_path(&self) -> &String {
		&self.path
	}

	//TODO: use DesktopFile or str?
	pub fn get_postscripts(desktop: &str) -> HashMap<String,PostScript> {
		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
		let postscripts_dir = gtheme_home + &format!("/desktops/{}/gtheme/post-scripts", desktop);
		let entries = fs::read_dir(&postscripts_dir).expect(&format!("Could not read directory:{}", &postscripts_dir));

		let mut map = HashMap::new();
		for entry in entries {
			let entry = entry.expect(&format!("Error while reading entry from dir: {}", &postscripts_dir));
			let file_name = entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			let name = match file_name.rsplit_once(".") {
				None => file_name,
				Some((prefix,_)) => String::from(prefix)
			};
			map.insert(name.clone(), PostScript { name, path });
		}
		//TODO: decide to sort or not
		map
	}

	pub fn get_extras(desktop: &str) -> Vec<PostScript> {
		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
		let extras_dir = gtheme_home + &format!("/desktops/{}/gtheme/extras", desktop);
		let entries = fs::read_dir(&extras_dir).expect(&format!("Could not read directory:{}", &extras_dir));

		let mut extras_vec: Vec<PostScript> = Vec::new();
		for entry in entries {
			let entry = entry.expect(&format!("Error while reading entry from dir: {}", &extras_dir));
			let file_name = entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			let name = match file_name.rsplit_once(".") {
				None => file_name,
				Some((prefix,_)) => String::from(prefix)
			};
			extras_vec.push(PostScript { name, path });
		}
		extras_vec
	}

	pub fn execute(&self, args: Vec<&str>) {
		Command::new(self.get_path())
			.stdout(Stdio::null())
			.stdin(Stdio::null())
			.stderr(Stdio::null())
			.args(args)
			.spawn().expect(&format!("Could not execute file:{}", self.get_path()));
	}
}
#[cfg(test)]
mod tests{
	use super::*;
	use crate::core::desktop::Desktop;
	#[test]
	fn test_get_postscripts() {
		let desktops = Desktop::get_desktops();
		let desktop = desktops.into_iter().find(|desktop |desktop.get_name()=="jorge").unwrap().to_desktop();

		let postscripts = PostScript::get_postscripts(desktop.get_name());

		for ps in postscripts.values() {
			println!("post-script {} in {}",ps.get_name(),ps.get_path());
		}
	}

}
