use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize,Deserialize};
use crate::core::desktop::DesktopFile;
use crate::core::pattern::PatternFile;
use crate::core::postscript::PostScript;
use crate::core::{theme::{Theme,ThemeFile}};
use log::{info,warn,error};

#[derive(Debug, Serialize, Deserialize)]
struct DesktopConfigDto {
	default_theme: Option<String>,
	actived: HashMap<String,bool>,
	inverted: HashMap<String,bool>,
}

#[derive(Debug,Clone)]
pub struct DesktopConfig {
	desktop: DesktopFile,
	default_theme: Option<ThemeFile>,
	actived: HashMap<String,bool>,
	inverted: HashMap<String,bool>,
}

impl DesktopConfigDto {
	fn new(desktop: &DesktopFile) -> DesktopConfigDto {
		let path = format!("{}/desktop_config.json",desktop.get_path());
		let mut file = match File::open(&path) {
			Ok(file) => file,
			Err(e) => {
				warn!("Could not open desktop config, using default config: |{}|", e);
				return Self::default(desktop)
			}
		};

		let mut content = String::new();
		match file.read_to_string(&mut content){
			Ok(_) => (),
			Err(e) => {
				error!("Could not read desktop config, using default config: |{}|", e);
				return Self::default(desktop)
			}
		};

		let dto = match serde_json::from_str(&content) {
			Ok(config) => {
				info!("Using desktop config |{}|", &path);
				config
			},
			Err(e) => {
				error!("Could not parse desktop config, using default config: |{}|", e);
				Self::default(desktop)
			}
		};

		//Ensure all keys are filled on hashmaps
		let desktop_owned = desktop.to_desktop();
		let patterns = desktop_owned.get_patterns();
		let mut actived = dto.actived;
		let mut inverted = dto.inverted;
		for pattern in patterns{
			let pattern_name = pattern.get_name();
			if let None = actived.get(pattern_name) {
				actived.insert(String::from(pattern_name),true);
			}
			if let None = inverted.get(pattern_name) {
				inverted.insert(String::from(pattern_name),false);
			}
		}
		let extras = PostScript::get_extras(desktop);
		for extra in extras{
			let extra_name = extra.get_name();
			if let None = actived.get(extra_name) {
				actived.insert(String::from(extra_name),false);
			}
		}

		DesktopConfigDto{
			default_theme:dto.default_theme,
			actived,
			inverted
		}
	}

	fn from(config: &DesktopConfig) -> Self {
		let default_theme = match config.get_default_theme() {
			Some(theme) => Some(String::from(theme.get_name())),
			None => None
		};

		DesktopConfigDto {
			default_theme,
			actived:config.get_actived().clone(),
			inverted:config.get_inverted().clone()
		}
	}

	fn save(&self, desktop: &DesktopFile) {

		let content = serde_json::to_string(self).unwrap();
		let path = format!("{}/desktop_config.json",desktop.get_path());

		let mut file = match OpenOptions::new().create(true).write(true).truncate(true).open(&path) {
			Ok(f) => f,
			Err(e) => {
				error!("Could not open |{}| with write permissions: |{}|",&path,e);
				return;
			}
		};

   		match file.write_all(&content.as_bytes()) {
			Err(e) => error!("Could not write desktop config in |{}|: |{}|",&path,e),
			_ => info!("Saving desktop config...")
		}
	}
	fn default(desktop: &DesktopFile) -> DesktopConfigDto{
		let desktop_owned = desktop.to_desktop();
		let patterns = desktop_owned.get_patterns();
		let mut actived = HashMap::new();
		let mut inverted = HashMap::new();
		for pattern in patterns{
			actived.insert(String::from(pattern.get_name()),true);
			inverted.insert(String::from(pattern.get_name()),false);
		}
		let extras = PostScript::get_extras(desktop);
		for extra in extras{
			actived.insert(String::from(extra.get_name()),false);
		}
		DesktopConfigDto {
			default_theme: None,
			actived,
			inverted
		}
	}
}

impl DesktopConfig{
	pub fn new(desktop: &DesktopFile) -> Self {
		let dto = DesktopConfigDto::new(desktop);
		let themes = Theme::get_themes();
		let default_theme = match dto.default_theme {
			Some(theme_name) => themes.into_iter().find(|theme| *theme.get_name().to_lowercase() == theme_name.to_lowercase()),
			None => None
		};
		DesktopConfig {
			desktop: desktop.clone(),
			default_theme,
			actived: dto.actived,
			inverted: dto.inverted
		}
	}
	pub fn get_default_theme(&self) -> &Option<ThemeFile> {
		&self.default_theme
	}
	pub fn get_mut_default_theme(&mut self) -> &mut Option<ThemeFile> {
		&mut self.default_theme
	}
	pub fn get_actived(&self) -> &HashMap<String, bool> {
		&self.actived
	}
	pub fn get_mut_actived(&mut self) -> &mut HashMap<String, bool> {
		&mut self.actived
	}
	pub fn get_inverted(&self) -> &HashMap<String, bool> {
		&self.inverted
	}
	pub fn get_mut_inverted(&mut self) -> &mut HashMap<String, bool> {
		&mut self.inverted
	}
	pub fn save(&self) {
		DesktopConfigDto::from(self).save(&self.desktop)
	}
	pub fn enable_pattern(&mut self,pattern:&PatternFile){
		let state = self.actived.get(pattern.get_name()).unwrap_or(&false);

		match state {
			true => warn!("Pattern |{}| was already |enabled|",pattern.get_name()),
			false=> {
				self.actived.insert(String::from(pattern.get_name()),true);
				info!("Pattern |{}| successfully |enabled|!",pattern.get_name());
			}
		}
	}
	pub fn disable_pattern(&mut self,pattern: &PatternFile) {
		let state = self.actived.get(pattern.get_name()).unwrap_or(&true);
		
		match state{
			false => warn!("Pattern |{}| was already |disabled|!",pattern.get_name()),
			true => {
				self.actived.insert(String::from(pattern.get_name()),false);
				info!("Pattern |{}| successfully |disabled|!",pattern.get_name());
			}
		}
	}
	pub fn toggle_pattern(&mut self, pattern: &PatternFile){
		let state = self.actived.get(pattern.get_name()).unwrap_or(&true);
		match state{
			true=>self.disable_pattern(pattern),
			false=>self.enable_pattern(pattern)
		}
	}

	pub fn enable_invert_pattern(&mut self,pattern:&PatternFile){
		let state = self.inverted.get(pattern.get_name()).unwrap_or(&false);

		match state{
			true => warn!("Pattern |{}| was already |inverted|",pattern.get_name()),
			false=> {
				self.inverted.insert(String::from(pattern.get_name()),true);
				info!("Pattern |{}| successfully |inverted|!",pattern.get_name());
			}
		}
	}
	pub fn disable_invert_pattern(&mut self,pattern: &PatternFile) {
		let state = self.inverted.get(pattern.get_name()).unwrap_or(&true);

		match state{
			false => {
				self.inverted.insert(String::from(pattern.get_name()),false);
				info!("Pattern |{}| successfully |inverted|!",pattern.get_name());
			}
			true=> warn!("Pattern |{}| was already |inverted|!",pattern.get_name())
		}
	}
	pub fn toggle_invert_pattern(&mut self, pattern: &PatternFile){
		let state = self.inverted.get(pattern.get_name()).unwrap_or(&true);
		match state{
			true=>self.disable_pattern(pattern),
			false=>self.enable_pattern(pattern)
		}
	}

	pub fn enable_extra(&mut self,extra:&PostScript){
		let state = self.actived.get(extra.get_name()).unwrap_or(&false);

		match state{
			true => warn!("Extra |{}| was already |enabled|",extra.get_name()),
			false=> {
				self.actived.insert(String::from(extra.get_name()),true);
				info!("Extra |{}| successfully |enabled|!",extra.get_name());
			}
		}
	}
	pub fn disable_extra(&mut self,extra: &PostScript) {
		let state = self.actived.get(extra.get_name()).unwrap_or(&true);

		match state{
			false => {
				self.actived.insert(String::from(extra.get_name()),false);
				info!("Extra |{}| successfully |disabled|!",extra.get_name());
			}
			true=> warn!("Extra |{}| was already |disabled|!",extra.get_name())
		}
	}
	pub fn toggle_extra(&mut self, extra: &PostScript){
		let state = self.actived.get(extra.get_name()).unwrap_or(&true);
		match state{
			true=>self.disable_extra(extra),
			false=>self.enable_extra(extra)
		}
	}

}
