use std::fs;
use std::process::{Command,Stdio};
use std::collections::BTreeMap;
use log::error;

use crate::core::desktop::DesktopFile;

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
	
	pub fn get_postscripts(desktop: &DesktopFile) -> BTreeMap<String,PostScript> {
		let postscripts_dir = format!("{}/gtheme/post-scripts", desktop.get_path());
		let entries = match fs::read_dir(&postscripts_dir) {
			Ok(dir) => dir,
			Err(e) => {
				error!("Could not read directory |{}|: |{}|", &postscripts_dir, e);
				return BTreeMap::new()
			}
		};

		let mut map = BTreeMap::new();
		for entry in entries {
			let entry = match entry {
				Ok(d) => d,
				Err(e) => {
					error!("Error while reading entry from dir |{}|: |{}|", &postscripts_dir, e);
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
				None => {
					error!("Error while converting path to String: |Invalid UTF-8 data|");
					continue;
				}
			};

			let name = match file_name.rsplit_once(".") {
				None => file_name,
				Some((prefix,_)) => String::from(prefix)
			};
			map.insert(name.clone(), PostScript { name, path });
		}
		map
	}

	pub fn get_postscript_by_name(desktop:&DesktopFile, postscript_name: &str) -> Option<PostScript> {
		let all_postscripts = PostScript::get_postscripts(desktop);
		all_postscripts.get(postscript_name).cloned()
	}

	pub fn get_extra_by_name(desktop:&DesktopFile, extra: &str)->Option<PostScript> {
		let all_extras = PostScript::get_extras(desktop);
		all_extras.into_iter().find(|item| item.get_name().to_lowercase() == extra.to_lowercase())
	}

	pub fn get_extras(desktop: &DesktopFile) -> Vec<PostScript> {
		let extras_dir = format!("{}/gtheme/extras", desktop.get_path());

		let entries = match fs::read_dir(&extras_dir) {
			Ok(dir) => dir,
			Err(e) => {
				error!("Could not read directory |{}|: |{}|", &extras_dir, e);
				return vec![]
			}
		};

		let mut extras_vec: Vec<PostScript> = Vec::new();
		for entry in entries {
			let entry = match entry {
				Ok(entry) => entry,
				Err(e) => {
					error!("Error while reading entry from dir |{}|: |{}|", &extras_dir, e);
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

			let path = match entry.path().to_str() {
				Some(path) => String::from(path),
				None => {
					error!("Error while converting path to String: |Invalid UTF-8 data|");
					continue;
				}
			};

			let name = match file_name.rsplit_once(".") {
				None => file_name,
				Some((prefix,_)) => String::from(prefix)
			};
			extras_vec.push(PostScript { name, path });
		}
		extras_vec
	}

	pub fn execute(&self, args: &Vec<String>) {
		match Command::new(self.get_path())
			.stdout(Stdio::null())
			.stdin(Stdio::null())
			.stderr(Stdio::null())
			.args(args)
			.spawn() {
				Ok(_) => (),
				Err(e) => error!("Could not execute file |{}|: |{}|", self.get_path(), e)	
			}
	}
}
