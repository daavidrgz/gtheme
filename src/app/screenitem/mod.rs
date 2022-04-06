use std::{process::{Command, Stdio}, env};
use log::{warn,error,info};
use tui::style::Color;
use std::fs::metadata;

use crate::app::statefullist::StatefulList;
use crate::core::{
	desktop::DesktopFile,
	theme::ThemeFile,
	pattern::PatternFile,
	postscript::PostScript,
	config::{GlobalConfig, DesktopConfig, DesktopInfo}
};

#[derive(Clone)]
pub enum ScreenItem {
	Desktop(DesktopFile),
	Theme(ThemeFile),
	Pattern(PatternFile),
	Extra(PostScript),
	Help(String),
	Info(String)
}
impl ScreenItem {
	pub fn get_name(&self) -> &str {
		match self {
			ScreenItem::Desktop(d) => d.get_name(),
			ScreenItem::Theme(t) => t.get_name(),
			ScreenItem::Pattern(p) => p.get_name(),
			ScreenItem::Extra(e) => e.get_name(),
			ScreenItem::Help(s) => &s,
			ScreenItem::Info(s) => &s
		}
	}

	pub fn get_path(&self) -> Option<String> {
		match self {
			ScreenItem::Desktop(d) => Some(d.get_path().clone()),
			ScreenItem::Theme(t) => Some(t.get_path().clone()),
			ScreenItem::Pattern(p) => Some(p.get_path().clone()),
			ScreenItem::Extra(e) => Some(e.get_path().clone()),
			ScreenItem::Help(_) => None,
			ScreenItem::Info(_) => None
		}
	}

	pub fn get_postscript_path(&self, desktop_config: &Option<DesktopConfig>) -> Option<String> {
		match self {
			ScreenItem::Pattern(p) => {
				let desktop = match desktop_config {
					Some(c) => c.get_desktop(),
					None => return None
				};
				match PostScript::get_postscript_by_name(desktop, p.get_name()) {
					Some(ps) => return Some(ps.get_path().clone()),
					None =>	{
						info!("The pattern |{}| has no postscript", p.get_name());
						return None
					}
				}
			},
			_ => return None
		}
	}

	pub fn get_theme(&self) -> Option<&ThemeFile> {
		match self {
			ScreenItem::Theme(t) => Some(t),
			_ => None
		}
	}
	pub fn get_desktop(&self) -> Option<&DesktopFile> {
		match self {
			ScreenItem::Desktop(d) => Some(d),
			_ => None,
		}
	}
	pub fn get_pattern(&self) -> Option<&PatternFile> {
		match self {
			ScreenItem::Pattern(p) => Some(p),
			_ => None
		}
	}
	pub fn get_extra(&self) -> Option<&PostScript> {
		match self {
			ScreenItem::Extra(e) => Some(e),
			_ => None
		}
	}
	pub fn get_help(&self) -> Option<&String> {
		match self {
			ScreenItem::Help(s) => Some(s),
			_ => None
		}
	}

	pub fn edit_file(path: &String) {
		match env::var("VISUAL") {
			Ok(value) => if value.is_empty() {
				warn!("Env var |$VISUAL| is empty, using |nano| instead")
			},
			Err(_) => warn!("Could not found env var |$VISUAL|, using |nano| instead")
		}

		match Command::new("sh")
		.arg("-c")
		.arg(format!("${{VISUAL:-nano}} {}", path))
		.stdin(Stdio::inherit())
		.stdout(Stdio::inherit())
		.output() {
			Ok(output) => {
				match output.status.success() {
					true => info!("File |{}| edited succesfully", path),
					false => error!("Could not edit |{}|, error: {}", path, String::from_utf8(output.stderr).unwrap())
				}
			},
			Err(e) => error!("Could not edit |{}|, error: {}", path, e)	
		}
	}

	pub fn explore_dir(path: &String) {
		match Command::new("sh")
		.arg("-c")
		.arg(format!("ranger {}", path))
		.stdin(Stdio::inherit())
		.stdout(Stdio::inherit())
		.output() {
			Ok(output) => {
				match output.status.success() {
					true => info!("Directory |{}| succesfully readed", path),
					false => error!("Could not read |{}|, error: {}", path, String::from_utf8(output.stderr).unwrap())
				}
			},
			Err(e) => error!("Could not read |{}|, error: {}", path, e)	
		}
	}

	pub fn edit(path: &String) {
		let md = match metadata(path) {
			Ok(md) => md,
			Err(err) => {
				error!("Could not read metadata from |{}|: |{}|", path, err);
				return;
			}
		};
		match md.is_dir() {
			true => Self::explore_dir(&path),
			false => Self::edit_file(&path)
		}
	}

	pub fn apply(&self, global_config: &mut GlobalConfig, desktop_config: &mut Option<DesktopConfig>) {
		match self {
			ScreenItem::Desktop(d) => Self::apply_desktop(d, global_config),
			ScreenItem::Theme(t) => Self::apply_theme(t, global_config, desktop_config),
			ScreenItem::Pattern(_) => Self::toggle_active(self, desktop_config),
			ScreenItem::Extra(_) => Self::toggle_active(self, desktop_config),
			ScreenItem::Help(_) | ScreenItem::Info(_) => ()
		}
	}
	
	pub fn is_default_theme(&self, desktop_config_opt: &Option<DesktopConfig>) -> bool {
		let desktop_config = match desktop_config_opt {
			Some(c) => c,
			None => return false
		};

		match self {
			ScreenItem::Theme(theme) => match desktop_config.get_default_theme() {
				Some(t) => theme.get_name() == t.get_name(),
				None => false
			},
			_ => false
		}
	}

	pub fn set_default_theme(&self, desktop_config_opt: &mut Option<DesktopConfig>) {
		let desktop_config = match desktop_config_opt {
			Some(c) => c,
			None => {
				error!("|There is no desktop installed|, cannot set default theme!");
				return
			}
		};
		match self {
			ScreenItem::Theme(t) => {
				desktop_config.set_default_theme(t);
				desktop_config.save()
			},
			_ => {}
		}
	}

	pub fn create_desktop_info(&self, stateful_list: &mut StatefulList<ScreenItem>) {
		match self {
			ScreenItem::Desktop(d) => {
				let desktop_info = DesktopInfo::new(&d);

				let mut lines: Vec<ScreenItem> = vec![];
				let name_str = format!("Name: {}", d.get_name());
				let author_str = format!("Author: {}", desktop_info.get_author());
				let credits_str = format!("Credits: {}", desktop_info.get_credits());
				let description_str = format!("Description: {}", desktop_info.get_description());
				let dependencies_str = format!("Dependencies:");
		
				lines.push(ScreenItem::Info(name_str));
				lines.push(ScreenItem::Info(author_str));
				lines.push(ScreenItem::Info(credits_str));
				lines.push(ScreenItem::Info(description_str));
				lines.push(ScreenItem::Info(dependencies_str));

		    for dependency in desktop_info.get_dependencies(){
					lines.push(ScreenItem::Info(format!("- {}",dependency)));
				}
				*stateful_list = StatefulList::with_items(lines)
					.color(Color::Green)
					.title("INFO ")
			},
			_ => ()
		}

	}

	pub fn invert(&self, desktop_config_opt: &mut Option<DesktopConfig>) {
		let desktop_config = match desktop_config_opt {
			Some(c) => c,
			None => {
				error!("|There is no desktop installed|, cannot invert pattern!");
				return
			}
		};
		match self {
			ScreenItem::Pattern(p) => {
				desktop_config.toggle_invert_pattern(p);
				desktop_config.save()
			},
			_ => {}
		}
	}
	
	pub fn is_inverted(&self, desktop_config: &Option<DesktopConfig>) -> bool {
		match self {
			ScreenItem::Pattern(p) => {
				match desktop_config {
					Some(d_config) => *d_config.get_inverted().get(p.get_name()).unwrap_or(&false),
					None => false
				}
			},
			_ => false
		}
	}

	pub fn is_active(&self, global_config: &GlobalConfig, desktop_config: &Option<DesktopConfig>) -> bool {
		match self {
			ScreenItem::Desktop(d) => {
				match global_config.get_current_desktop() {
					Some(current_desktop) => d.get_name() == current_desktop.get_name(),
					None => false
				}
			},
			ScreenItem::Theme(t) => {
				match global_config.get_current_theme() {
					Some(current_theme) => t.get_name() == current_theme.get_name(),
					None => false
				}
			},
			ScreenItem::Pattern(p) => { 
				match desktop_config {
					Some(d_config) => *d_config.get_actived().get(p.get_name()).unwrap_or(&false),
					None => true
				}
			},
			ScreenItem::Extra(e) => { 
				match desktop_config {
					Some(d_config) => *d_config.get_actived().get(e.get_name()).unwrap_or(&false),
					None => false
				}
			},
			ScreenItem::Help(_) | ScreenItem::Info(_) => false
		}
	}

	fn toggle_active(item: &ScreenItem, desktop_config_opt: &mut Option<DesktopConfig>) {
		let desktop_config = match desktop_config_opt {
			None => {
				warn!("Cannot activate item, |there is no desktop installed!|");
				return
			}
			Some(config) => config,
		};

		match item {
			ScreenItem::Pattern(p) => desktop_config.toggle_pattern(p),
			ScreenItem::Extra(e) => desktop_config.toggle_extra(e),
			_ => ()
		}
		desktop_config.save()
	}

	fn apply_theme(theme: &ThemeFile, global_config: &mut GlobalConfig, desktop_config_opt: &mut Option<DesktopConfig>) {
		let current_desktop = match global_config.get_current_desktop() {
			Some(d) => d.to_desktop(),
			None => {
				error!("Cannot apply a theme, |there is no desktop installed!|");
				return
			}
		};
		let desktop_config = match desktop_config_opt {
			Some(config) => config,
			None => {
				error!("Cannot apply a theme, |there is no desktop installed!|");
				return
			}
		};

		current_desktop.apply_theme(
			&theme.to_theme(),
			desktop_config.get_actived(),
			desktop_config.get_inverted(),
			false
		);

		*global_config.get_mut_current_theme() = Some(theme.clone());
		global_config.save()
	}

	fn apply_desktop(next_desktop: &DesktopFile, global_config: &mut GlobalConfig){
		let current_desktop = match global_config.get_current_desktop() {
			Some(d) => Some(d.to_desktop()),
			None => None
		};

		let next_desktop_config = DesktopConfig::new(&next_desktop);
		let theme = match next_desktop_config.get_default_theme() {
			Some(t) => t.clone(),
			None => {
				error!("There is no |default theme| specified in desktop |{}|", next_desktop.get_name());
				return
			}
		};

		*global_config.get_mut_current_desktop() = Some(next_desktop.clone());
		*global_config.get_mut_current_theme() = Some(theme.clone());
		global_config.save();

		next_desktop.to_desktop().apply(
			&current_desktop,
			&theme.to_theme(),
			next_desktop_config.get_actived(),
			next_desktop_config.get_inverted(),
			false
		)
	}
}
