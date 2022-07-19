use std::fs::{self, DirEntry,metadata};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::core;
use crate::core::pattern::*;
use crate::core::theme::Theme;
use crate::core::postscript::PostScript;
use crate::core::config::{UserConfig,GlobalConfig};

use log::{info,error};

#[derive(Debug, Clone)]
pub struct Desktop{
	name: String,
	path: String,
	patterns: Vec<PatternFile>,
	post_scripts: BTreeMap<String,PostScript>,
	extras: Vec<PostScript>
}
impl Desktop {
	pub fn from(desktop: &DesktopFile) -> Self {
		let patterns = Pattern::get_patterns(desktop);
		let post_scripts = PostScript::get_postscripts(desktop);
		let extras = PostScript::get_extras(desktop);
		Desktop {
			name: String::from(desktop.get_name()),
			path: String::from(desktop.get_path()),
			patterns,
			post_scripts,
			extras
		}
	}
	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_path(&self) -> &String {
		&self.path
	}
	pub fn get_patterns(&self) -> &Vec<PatternFile> {
		&self.patterns
	}
	pub fn get_post_scripts(&self) -> &BTreeMap<String,PostScript> {
		&self.post_scripts
	}
	pub fn get_extras(&self) -> &Vec<PostScript> {
		&self.extras
	}
	
	pub fn get_by_name(desktop:&str) -> Option<DesktopFile> {
		let all_desktops = Desktop::get_desktops();
		match all_desktops.into_iter().find(|item|item.get_name().to_lowercase() == desktop.to_lowercase()) {
			Some(desktop) => Some(desktop),
			None => {
				error!("Desktop |{}| does not exist",desktop);
				None
			}
		}
	}

	pub fn exists(desktop: &str) -> bool {
		Desktop::get_desktops().iter().any(|desktop_file|desktop_file.get_name().to_lowercase() == desktop.to_lowercase())
	}

	pub fn get_desktops() -> Vec<DesktopFile> {
		let gtheme_home: String = core::expand_path(core::GTHEME_HOME);
		let desktops_dir = gtheme_home + &format!("/desktops");
			
		let mut vec = Vec::new();
		let desktop_entries = core::get_files(Path::new(&desktops_dir));

		for entry in desktop_entries {			
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

			let metadata = match entry.metadata(){
				Ok(metadata) => metadata,
				Err(err) => {
					error!("Could not read metadata from desktop |{}|: |{}|",path,err);
					continue;
				}
			};
			if !metadata.is_dir() || file_name.starts_with("."){
				//If isnt a dir or is a hidden dir
				continue;
			}
	
			vec.push(DesktopFile{name: file_name, path});
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}

	pub fn apply_theme(&self, theme: &Theme, actived: &BTreeMap<String,bool>, inverted: &BTreeMap<String,bool>, dry_run: bool) {
		if dry_run {
			info!("Applying theme in dry-run mode...");
		}

		let post_scripts = self.get_post_scripts();
		info!("Applying |{}| theme to |{}| desktop...", theme.get_name(), self.get_name());
		let user_config = UserConfig::new();
		for pattern_file in self.get_patterns(){
			let pattern = pattern_file.to_pattern();

			if !*actived.get(pattern.get_name()).unwrap_or(&false) { continue }

			pattern.fill(theme, *inverted.get(pattern.get_name()).unwrap_or(&false), &user_config, dry_run);
			if let Some(postscript) = post_scripts.get(pattern_file.get_name()) {
				info!("Executing |{}| post-script...", postscript.get_name());

				//Dont execute postscripts on dry-run mode
				if !dry_run {
					let output = pattern.get_output().as_ref().unwrap_or(&"".to_string()).clone();
					postscript.execute(&vec![output])
				}
			}
		}

		let args_map = theme.get_extras();
		for extra_ps in self.get_extras() {
			if !*actived.get(extra_ps.get_name()).unwrap_or(&false) { continue }

			let args = args_map.get(extra_ps.get_name()).unwrap_or(&vec![]).iter()
				.map(|arg|core::expand_path(arg)).collect();
			
			info!("Executing |{}| extra...",extra_ps.get_name());

			if !dry_run {
				extra_ps.execute(&args);
			}
		}
	}

	pub fn clean_files(&self) {
		let config_home = core::expand_path(core::CONFIG_HOME);
		
		//Remove only config files, not fonts
		for entry in self.get_config_files() {
			let path = Path::new(&config_home).join(entry.file_name());
			if let Err(e) =  fs_extra::dir::remove(&path) {
				error!("Could not remove directory |{}|: |{}|", path.display(), e);
			}
		}
	}

	pub fn apply(&self, previous: &Option<Desktop>, theme: &Theme, actived: &BTreeMap<String,bool>, inverted: &BTreeMap<String,bool>, dry_run: bool) {
		if dry_run {
			info!("Installing desktop |{}| in dry-run mode...",self.get_name());
		} else {
			info!("Installing desktop |{}|...", self.get_name());
		}

		let config_home = core::expand_path(core::CONFIG_HOME);
		if let Some(previous_desktop) = previous {
			info!("Uninstalling desktop |{}|...", previous_desktop.get_name());
			if !dry_run {
				previous_desktop.clean_files();
			}
		}

		if !dry_run {
			self.clean_files(); // Clean files to install
		}

		let config_files = self.get_config_files();
		info!("Copying config files to |{}|...",&config_home);
		for entry in config_files {
			if dry_run { break }
			let from = entry.path();
			let to = Path::new(&config_home);
			core::copy(&vec![from.as_path()],&to)
		}

		let fonts_home = core::expand_path( "~/.local/share/fonts/gtheme-fonts");
		info!("Copying fonts to |{}|...",&fonts_home);
		if !dry_run {
			let fonts_files = self.get_fonts_files();
			let from_buf:Vec<PathBuf> = fonts_files.into_iter()
				.map(|entry| entry.path()).collect();
			let from:Vec<&Path> = from_buf.iter().map(|buf| buf.as_path()).collect();

			let to = Path::new(&fonts_home);
			//Only copy if there is a fonts dir on desktop
			core::copy(&from,&to);
		}

		self.apply_theme(theme, actived, inverted,dry_run);

		if let Some(previous_desktop) = previous {
			// Exit postcript from previous desktop
			let previous_postscripts = previous_desktop.get_post_scripts();
			if let Some(ps) = previous_postscripts.get("desktop-exit") {
				info!("Executing |desktop-exit| post-script");
				// Dont execute exit postscript if dry-run mode
				if !dry_run {
					ps.execute(&vec![])
				};
			}
		};
	}

	pub fn get_config_files(&self) -> Vec<DirEntry> {
		let config_dir = format!("{}/.config", self.get_path());
		return core::get_files(Path::new(&config_dir));
	}

	pub fn get_fonts_files(&self) -> Vec<DirEntry> {
		let fonts_dir = format!("{}/fonts", self.get_path());
		return core::get_files(Path::new(&fonts_dir));
	}

	pub fn add(from: &Path)-> Result<(),Option<DesktopFile>>{
		info!("Adding desktop from |{}|...",from.to_str().unwrap());

		let md = match metadata(from) {
			Ok(md) => md,
			Err(err) => {
				error!("Could not read metadata from |{}|: |{}|", from.to_str().unwrap(), err);
				return Err(None);
			}
		};
		
		if !md.is_dir() {
			error!("|{}| is not a directory", from.to_str().unwrap());
			return Err(None);
		}

		let desktop_name = match from.file_name() {
			Some(name) => name.to_str().unwrap(),
			None => {
				error!("Could not get directory name from path |{}|", from.to_str().unwrap());
				return Err(None);
			}
		};

		if let Some(desktop_file) =  Desktop::get_by_name(desktop_name){
			return Err(Some(desktop_file));
		}

		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
		let desktops_dir = &format!("{}/desktops",gtheme_home);

		let to = Path::new(&desktops_dir);
		core::copy(&vec![from],&to);
		//TODO: check if copied successfully?
		info!("Successfully added desktop |{}|", desktop_name);
		return Ok(());
	}

	pub fn new_skeleton(desktop_name: &str) {
		if Desktop::exists(desktop_name) {
			error!("Desktop |{}| already exists", desktop_name);
			return;
		}

		let desktop_path = format!("{}/desktops/{}", core::expand_path(core::GTHEME_HOME), desktop_name);

		match fs::create_dir_all(&desktop_path) {
			Ok(_) => info!("Created directory |{}|", &desktop_path),
			Err(e) => {
				error!("Error while creating directory |{}|: |{}|", &desktop_path, e);
				return;
			}
		}

		let directories = vec![
			".config","gtheme","gtheme/extras","gtheme/patterns","gtheme/post-scripts",
		];

		for directory in directories {
			let target_path = format!("{}/{}", desktop_path, directory);
			match fs::create_dir_all(&target_path) {
				Ok(_) => info!("Created directory |{}|", &target_path),
				Err(e) => {
					error!("Error while creating directory |{}|: |{}|", &target_path, e);
					return;
				}
			}
		}

		match Desktop::get_by_name(desktop_name) {
			Some(desktop_file) => {
				core::config::DesktopConfig::create_default(&desktop_file);
				core::config::DesktopInfo::create_default(&desktop_file);
			} 
			None => {
				error!("Could not get desktop |{}|", desktop_name);
				return;
			}
		};		
		info!("Successfully created desktop |{}|", desktop_name);
	}
}

#[derive(Debug,Clone)]
pub struct DesktopFile {
	name: String,
	path: String,
}
impl DesktopFile {
	pub fn to_desktop(&self) -> Desktop {
		Desktop::from(self)
	}
	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn get_path(&self) -> &String {
		&self.path
	}
	// WARNING: After uninstalling a desktop, you SHOULD NOT use a DesktopFile or a Desktop 
	// that references this desktop. Behaviour is undefined.
	pub fn remove(&self) {
		let path = self.get_path();
		let desktop_name = self.get_name();
		info!("Removing desktop |{desktop_name}| from |{path}|");

		let global_config = GlobalConfig::new();
		if let Some(current_desktop) = global_config.get_current_desktop() {
			let current_desktop_name = current_desktop.get_name();
			if current_desktop_name == desktop_name {
				error!("Cannot uninstall current desktop |({current_desktop_name})|");
				return;
			}
		}
		if let Err(reason) = fs_extra::dir::remove(&path) {
			error!("Could not remove desktop |{desktop_name}| from |{path}|: |{reason}|");
			return;
		}
		info!("Successfully removed desktop |{desktop_name}|");
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn test_add() {
		let desktops = Desktop::get_desktops();
		let desktop = desktops.into_iter().find(|desktop |desktop.get_name()=="jorge" ).unwrap().to_desktop();
		let desktops = Desktop::get_desktops();
		let previous = desktops.into_iter().find(|desktop |desktop.get_name()=="retro" ).unwrap().to_desktop();
		
		let themes = Theme::get_themes();
		let theme = themes.into_iter().find(|theme |theme.get_name()=="Dracula" ).unwrap().to_theme();

		let patterns = desktop.get_patterns();

		let mut actived = BTreeMap::new();
		for pattern in patterns{
			actived.insert(String::from(pattern.get_name()),true);
		}
		actived.insert(String::from("wallpaper"),true);

		let mut inverted = BTreeMap::new();
		inverted.insert(String::from("polybar"), true);

		desktop.apply(&Some(previous),&theme,&actived,&inverted,false);
	}

	#[test]
	fn test_get_desktop_config() {
		let desktops = Desktop::get_desktops();
		let desktop = desktops.into_iter().find(|desktop |desktop.get_name()=="jorge" ).unwrap().to_desktop();

 		println!("{:?}",desktop.get_config_files().iter().map(|e|String::from(e.file_name().to_str().unwrap())).collect::<Vec<String>>());
	}

	#[test]
	fn test_get_desktop_patterns() {
		let desktops = Desktop::get_desktops();
		for desktop in &desktops{
			println!("Desktop: {} in {}",desktop.get_name(),desktop.get_path())
		}
		let desktop = desktops[4].to_desktop();
		println!("Patterns in {}: {:?}",desktop.get_name(),desktop.get_patterns())
	}


	#[test]
	fn test_desktop_remove(){
		let desktop = Desktop::get_by_name("test").unwrap();
		desktop.remove();
	}
}
