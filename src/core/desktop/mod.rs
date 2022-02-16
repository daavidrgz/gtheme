use std::fs::{self, DirEntry};
use std::collections::HashMap;

use crate::core;
use crate::core::pattern::*;
use crate::core::theme::Theme;
use crate::core::postscript::PostScript;

#[derive(Debug)]
pub struct Desktop{
	name: String,
	path: String,
	patterns: Vec<PatternFile>
}
impl Desktop {
	
	// pub fn from(path:&str)->Self{
	// 	let re = Regex::new("%output-file%=(.*)").unwrap();
	// 	let mut file = File::open(path).expect(&format!("Error while opening pattern: {}",path));
	// 	let mut content = String::new();
	// 	file.read_to_string(&mut content).expect(&format!("Error while reading pattern: {}",path));

	// 	assert!(re.is_match(&content),"Pattern does not have output file specified (hint: %output-file%=/path/to/output/file)");
	// 	let captured = re.captures(&content).unwrap();
	// 	//captured[0] is the whole matched expression.
	// 	let output_path = core::expand_path(&captured[1]);
		
	// 	Pattern{
	// 		path: String::from(path),
	// 		output:output_path
	// 	}
	// }
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
		let entries = fs::read_dir(&desktops_dir).expect(&format!("Could not read directory:{}", &desktops_dir));

		let mut vec = Vec::new();
		for entry in entries {
			let entry = entry.expect(&format!("Error while reading entry from dir: {}", &desktops_dir));
			let file_name = entry.file_name().into_string().expect(&format!("Error while converting OsString to String (invalid unicode data?)"));
			let path = String::from(entry.path().to_str().expect(&format!("Error while converting OsString to String (invalid utf-8 data?)")));

			vec.push(DesktopFile{name: file_name, path});
		}
		vec.sort_by(|a,b| a.get_name().to_lowercase().cmp(&b.get_name().to_lowercase()));
		vec
	}

	pub fn apply(&self, theme: &Theme, actived: HashMap<String,bool>, inverted: HashMap<String,bool>) {
		//parameter HashMap(pattern_name,bool) in order to implement inverted themes
		let postscripts = PostScript::get_postscripts(self.get_name());

		for pattern_file in self.get_patterns(){
			let pattern = pattern_file.to_pattern();
			
			//If not activated,skip pattern
			if !*actived.get(pattern.get_name()).unwrap_or(&false) { continue }

			pattern.fill(theme, *inverted.get(pattern.get_name()).unwrap_or(&false));
			if let Some(postscript) = postscripts.get(pattern_file.get_name()) {
				postscript.execute(vec![pattern.get_output()])
			}
		}
		if !&theme.wallpaper.is_empty() && *actived.get("wallpaper").unwrap_or(&false){
			//TODO: wallpaper actived or
			if let Some(ps) = postscripts.get("wallpaper") {
				ps.execute(vec![&core::expand_path(&theme.wallpaper)]);
			}
		}
	}

	pub fn uninstall(&self) {
		let config_home = core::expand_path(core::CONFIG_HOME);
		
		let files_to_uninstall:Vec<String> = self.get_config_files().iter()
			.map(|file| String::from(file.file_name().to_str().unwrap())).collect();

		for entry_name in files_to_uninstall {
			if let Ok(()) = fs_extra::dir::remove(format!("{}/{}",config_home,entry_name)) {
			}
			// else{
			// 	println!("Couldnt remove");
			// }
		}
	}

	pub fn install(&self, previous: &Desktop, theme: &Theme, actived: HashMap<String,bool>, inverted: HashMap<String,bool>) {
		let config_home = core::expand_path(core::CONFIG_HOME);

		previous.uninstall();

		let files_to_install = self.get_config_files();

		for entry in files_to_install {
			let from = entry.path();
			let to = format!("{}/{}",config_home,entry.file_name().to_str().unwrap());
			let mut options = fs_extra::dir::CopyOptions::new();
			options.overwrite = true;
			options.copy_inside = true;
			if let Ok(_) = fs_extra::dir::copy(from, to, &options) {
			}
			// else{
			// 	println!("Couldnt copy");
			// }
		}

		self.apply(theme, actived, inverted);
	}

	pub fn get_config_files(&self) -> Vec<DirEntry> {
		let gtheme_home:String = core::expand_path(core::GTHEME_HOME);
		let config_dir = gtheme_home + &format!("/desktops/{}/.config", self.get_name());
		let entries = fs::read_dir(&config_dir).expect(&format!("Could not read directory:{}", &config_dir));

		let mut vec = Vec::new();
		for entry in entries {
			let entry = entry.expect(&format!("Error while reading entry from dir: {}", &config_dir));
			vec.push(entry);
		}
		vec
	}
	//TODO: delete patterns function for a given directory?
}

#[derive(Debug)]
pub struct DesktopFile {
	name: String,
	path: String,
}
impl DesktopFile{
	pub fn to_desktop(&self) -> Desktop{
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

		desktop.install(&previous,&theme,actived,inverted);
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
