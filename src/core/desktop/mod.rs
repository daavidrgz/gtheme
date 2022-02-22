use std::fs::{self, DirEntry};
use std::collections::HashMap;

use crate::core;
use crate::core::pattern::*;
use crate::core::theme::Theme;
use crate::core::postscript::PostScript;

use log::{info,error};

#[derive(Debug, Clone)]
pub struct Desktop{
	name: String,
	path: String,
	patterns: Vec<PatternFile>
}
impl Desktop {
	
	pub fn from(desktop: &DesktopFile) -> Self {
		let patterns = Pattern::get_patterns(desktop.get_name());
		Desktop {
			name: String::from(desktop.get_name()),
			path: String::from(desktop.get_path()),
			patterns
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

	pub fn apply(&self, theme: &Theme, actived: &HashMap<String,bool>, inverted: &HashMap<String,bool>) {
		//parameter HashMap(pattern_name,bool) in order to implement inverted themes
		let postscripts = PostScript::get_postscripts(self.get_name());

		info!("Applying |{}| theme to |{}| desktop...", theme.get_name(), self.get_name());
		for pattern_file in self.get_patterns(){
			let pattern = pattern_file.to_pattern();
			
			//If not activated,skip pattern
			if !*actived.get(pattern.get_name()).unwrap_or(&true) { continue }

			pattern.fill(theme, *inverted.get(pattern.get_name()).unwrap_or(&false));
			if let Some(postscript) = postscripts.get(pattern_file.get_name()) {
				info!("Executing |{}| post-script...", postscript.get_name());
				postscript.execute(&vec![String::from(pattern.get_output())])
			}
		}

		let args_map = theme.get_extras();
		for extra_ps in PostScript::get_extras(self.get_name()) {
			if !*actived.get(extra_ps.get_name()).unwrap_or(&false){continue}

			let args = args_map.get(extra_ps.get_name()).unwrap_or(&vec![]).iter()
				.map(|arg|core::expand_path(arg)).collect();
			
			info!("Executing |{}| extra...",extra_ps.get_name());
			extra_ps.execute(&args);
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

	pub fn install(&self, previous: &Desktop, theme: &Theme, actived: &HashMap<String,bool>, inverted: &HashMap<String,bool>) {
		let config_home = core::expand_path(core::CONFIG_HOME);

		info!("Uninstalling desktop |{}|...", previous.get_name());
		previous.uninstall();
		self.uninstall();

		let files_to_install = self.get_config_files();

		info!("Installing desktop |{}|...", self.get_name());
		for entry in files_to_install {
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

		self.apply(theme, actived, inverted);

		let postscripts = PostScript::get_postscripts(previous.get_name());
		if let Some(ps) = postscripts.get("desktop-exit") {
			info!("Executing |desktop-exit| post-script");
			ps.execute(&vec![]);
		}
	}

	pub fn get_config_files(&self) -> Vec<DirEntry> {
		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
		let config_dir = gtheme_home + &format!("/desktops/{}/.config", self.get_name());

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

		desktop.install(&previous,&theme,&actived,&inverted);
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
