use std::fs::{self, DirEntry};
use std::collections::HashMap;

use crate::core;
use crate::core::pattern::*;
use crate::core::theme::Theme;
use crate::core::postscript::PostScript;
use crate::core::config::UserConfig;

use log::{info,error};

#[derive(Debug, Clone)]
pub struct Desktop{
	name: String,
	path: String,
	patterns: Vec<PatternFile>,
	post_scripts:HashMap<String,PostScript>,
	extras:Vec<PostScript>
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
	pub fn get_post_scripts(&self) -> &HashMap<String,PostScript> {
		&self.post_scripts
	}
	pub fn get_extras(&self) -> &Vec<PostScript> {
		&self.extras
	}
	

	pub fn get_by_name(desktop:&str)->Option<DesktopFile>{
		let all_desktops = Desktop::get_desktops();
		match all_desktops.into_iter().find(|item|item.get_name().to_lowercase() == desktop.to_lowercase()){
			Some(desktop) => Some(desktop),
			None => {
				error!("Desktop |{}| does not exist",desktop);
				None
			}
		}
	}

	pub fn get_desktops() -> Vec<DesktopFile> {
		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
		let desktops_dir = gtheme_home + &format!("/desktops");
		let entries = match fs::read_dir(&desktops_dir) {
			Ok(dir) => dir,
			Err(e) => {
				error!("Could not read directory |{}|: |{}|", &desktops_dir, e);
				return vec![]
			}
		};
			

		let mut vec = Vec::new();
		for entry in entries {
			let entry = match entry {
				Ok(entry) => entry,
				Err(e) => {
					error!("Error while reading entry from dir |{}|: |{}|", &desktops_dir, e);
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
	
			vec.push(DesktopFile{name: file_name, path});
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}

	pub fn apply(&self, theme: &Theme, actived: &HashMap<String,bool>, inverted: &HashMap<String,bool>,dry_run: bool) {
		//parameter HashMap(pattern_name,bool) in order to implement inverted themes
		info!("Applying theme in dry-run mode...");

		let post_scripts = self.get_post_scripts();
		info!("Applying |{}| theme to |{}| desktop...", theme.get_name(), self.get_name());
		let user_config = UserConfig::new();
		for pattern_file in self.get_patterns(){
			let pattern = pattern_file.to_pattern();
			
			//If not activated,skip pattern
			if !*actived.get(pattern.get_name()).unwrap_or(&false) { continue }

			pattern.fill(theme, *inverted.get(pattern.get_name()).unwrap_or(&false),&user_config,dry_run);
			if let Some(postscript) = post_scripts.get(pattern_file.get_name()) {
				info!("Executing |{}| post-script...", postscript.get_name());

				//Dont execute postscripts on dry-run mode
				if !dry_run{
					postscript.execute(&vec![String::from(pattern.get_output())])
				}
			}
		}

		let args_map = theme.get_extras();
		for extra_ps in self.get_extras() {
			if !*actived.get(extra_ps.get_name()).unwrap_or(&false){continue}

			let args = args_map.get(extra_ps.get_name()).unwrap_or(&vec![]).iter()
				.map(|arg|core::expand_path(arg)).collect();
			
			info!("Executing |{}| extra...",extra_ps.get_name());

			if !dry_run{
				extra_ps.execute(&args);
			}
		}
	}

	pub fn uninstall(&self) {
		let config_home = core::expand_path(core::CONFIG_HOME);
		
		let files_to_uninstall:Vec<String> = self.get_config_files().iter()
			.map(|file| String::from(file.file_name().to_str().unwrap())).collect();

		for entry_name in files_to_uninstall {
			let path = format!("{}/{}", config_home,entry_name);
			match fs_extra::dir::remove(&path) {
				Ok(_) => (),
				Err(e) => error!("Could not remove directory |{}|: |{}|",&path,e)
			}
		}
	}

	// TODO: Integrate desktopConfig inside Desktop to have direct access to active and inverted
	pub fn install(&self, previous: &Option<Desktop>, theme: &Theme, actived: &HashMap<String,bool>, inverted: &HashMap<String,bool>,dry_run:bool) {
		let config_home = core::expand_path(core::CONFIG_HOME);

		if dry_run{
			info!("Installing desktop in dry-run mode...")
		}
		if let Some(previous_desktop) = previous{
			info!("Uninstalling desktop |{}|...", previous_desktop.get_name());
			if !dry_run {
				previous_desktop.uninstall();
			}
		};
		if !dry_run{
			// Clean files to install
			self.uninstall();
		}


		let files_to_install = self.get_config_files();

		info!("Installing desktop |{}|...", self.get_name());
		for entry in files_to_install {
			//Break loop if dry_run
			if dry_run {break}

			let from = entry.path();

			let file_name = match entry.file_name().into_string() {
				Ok(file_name) => file_name,
				Err(_) => {
					error!("Error while converting OsString to String: |Invalid unicode data|");
					continue;
				}
			};
			let to = format!("{}/{}",config_home,file_name);


			let mut options = fs_extra::dir::CopyOptions::new();
			options.overwrite = true;
			options.copy_inside = true;
			match fs_extra::dir::copy(from, &to, &options) {
				Ok(_) => (),
				Err(e) => error!("Error while copying to |{}|: |{}|", &to, e),
			}
		}

		self.apply(theme, actived, inverted,dry_run);

		if let Some(previous_desktop) =  previous {
				//Exit postcript from previous desktop
			let previous_postscripts = previous_desktop.get_post_scripts();
			if let Some(ps) = previous_postscripts.get("desktop-exit") {
				info!("Executing |desktop-exit| post-script");
				
				//Dont execute exit postscript if dry-run mode
				if !dry_run{
					ps.execute(&vec![])
				};
			}
		};
	}

	pub fn get_config_files(&self) -> Vec<DirEntry> {
		let config_dir = format!("{}/.config", self.get_path());

		let entries = match fs::read_dir(&config_dir) {
			Ok(dir) => dir,
			Err(e) => {
				error!("Could not read directory |{}|: |{}|", &config_dir, e);
				return vec![]
			}
		};

		let mut vec = Vec::new();
		for entry in entries {
			let entry = match entry {
				Ok(entry) => entry,
				Err(e) => {
					error!("Error while reading entry from dir |{}|: |{}|", &config_dir, e);
					continue;
				}
			};

			vec.push(entry);
		}
		vec
	}
}

#[derive(Debug,Clone)]
pub struct DesktopFile {
	name: String,
	path: String,
}
impl DesktopFile{
	pub fn to_desktop(&self) -> Desktop {
		Desktop::from(self)
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
	fn test_install() {
		let desktops = Desktop::get_desktops();
		let desktop = desktops.into_iter().find(|desktop |desktop.get_name()=="jorge" ).unwrap().to_desktop();
		let desktops = Desktop::get_desktops();
		let previous = desktops.into_iter().find(|desktop |desktop.get_name()=="retro" ).unwrap().to_desktop();
		
		let themes = Theme::get_themes();
		let theme = themes.into_iter().find(|theme |theme.get_name()=="Dracula" ).unwrap().to_theme();

		let patterns = desktop.get_patterns();

		let mut actived = HashMap::new();
		for pattern in patterns{
			actived.insert(String::from(pattern.get_name()),true);
		}
		actived.insert(String::from("wallpaper"),true);

		let mut inverted = HashMap::new();
		inverted.insert(String::from("polybar"), true);

		desktop.install(&Some(previous),&theme,&actived,&inverted,false);
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
}
