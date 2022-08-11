use log::{error, info, warn};
use regex::{Captures, Regex};
use std::collections::BTreeMap;
use std::fs::{self, metadata, File};
use std::io::prelude::*;
use std::path::Path;

use crate::core;
use crate::core::config::UserConfig;
use crate::core::desktop::DesktopFile;
use crate::core::theme::Theme;

#[derive(Debug)]
pub struct Pattern {
    name: String,
    path: String,
    output: Option<String>,
    content: Option<String>,
    submodules: Option<Vec<PatternFile>>,
}
impl Pattern {
    //TODO: From str or from PatternFile??
    pub fn from(pattern: &PatternFile) -> Self {
        let pattern_path = Path::new(pattern.get_path());

        let metadata = match metadata(pattern_path) {
            Ok(metadata) => metadata,
            Err(e) => {
                error!(
                    "Could not read metadata from |{}|: |{}|",
                    pattern.get_path(),
                    e
                );
                return Self::default(pattern);
            }
        };

        if metadata.is_dir() {
            let submodules = Self::get_patterns_from_path(pattern_path);
            return Pattern {
                name: pattern.get_name().to_string(),
                path: pattern.get_path().to_string(),
                output: None,
                content: None,
                submodules: Some(submodules),
            };
        } else if !metadata.is_file() {
            error!(
                "Pattern |{}| from |{}|is not a directory nor a file",
                pattern.get_name(),
                pattern.get_path()
            );
            return Self::default(pattern);
        }

        let mut file = match File::open(pattern_path) {
            Ok(file) => file,
            Err(e) => {
                error!(
                    "Could not open pattern |{}| from |{}|: |{}|",
                    pattern.get_name(),
                    pattern.get_path(),
                    e
                );
                return Self::default(pattern);
            }
        };

        let re = Regex::new(r"<\[output-file\]>=(.*)(\r\n|\r|\n)").unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content).expect(&format!(
            "Error while reading pattern: {}",
            pattern.get_path()
        ));

        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "Error while reading pattern |{}| from |{}|: |{}|",
                    pattern.get_name(),
                    pattern.get_path(),
                    e
                );
                return Self::default(pattern);
            }
        }
        let output_path = match re.captures(&content) {
            Some(capture) => Some(core::utils::expand_path(&capture[1])),
            None => None,
        };
        content = String::from(re.replace(&content, ""));

        Pattern {
            name: String::from(pattern.get_name()),
            path: String::from(pattern.get_path()),
            output: output_path,
            content: Some(content),
            submodules: None,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_path(&self) -> &String {
        &self.path
    }
    pub fn get_output(&self) -> &Option<String> {
        &self.output
    }
    pub fn get_content(&self) -> &Option<String> {
        &self.content
    }
    pub fn has_submodules(&self) -> bool {
        self.submodules.is_some()
    }
    pub fn get_submodules(&self) -> &Option<Vec<PatternFile>> {
        &self.submodules
    }
    fn default(pattern: &PatternFile) -> Self {
        Pattern {
            name: String::from(pattern.get_name()),
            path: String::from(pattern.get_path()),
            output: None,
            content: None,
            submodules: None,
        }
    }

    pub fn get_by_name(desktop: &DesktopFile, pattern: &str) -> Option<PatternFile> {
        let all_patterns = Pattern::get_patterns(desktop);
        match all_patterns
            .into_iter()
            .find(|item| item.get_name().to_lowercase() == pattern.to_lowercase())
        {
            Some(pattern) => Some(pattern),
            None => {
                error!("Pattern |{}| does not exist", pattern);
                None
            }
        }
    }

    pub fn get_patterns(desktop: &DesktopFile) -> Vec<PatternFile> {
        let patterns_dir = format!("{}/gtheme/patterns", desktop.get_path());
        let path = Path::new(&patterns_dir);
        return Self::get_patterns_from_path(path);
    }

    fn get_patterns_from_path(path: &Path) -> Vec<PatternFile> {
        let entries = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(e) => {
                error!("Could not read directory |{}|: |{}|", path.display(), e);
                return vec![];
            }
        };

        let mut vec = Vec::new();
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    error!(
                        "Error while reading entry from dir |{}|: |{}|",
                        path.display(),
                        e
                    );
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

            let path = entry.path().display().to_string();
            let md = match metadata(&path) {
                Ok(md) => md,
                Err(err) => {
                    error!("Could not read metadata from |{}|: |{}|", path, err);
                    continue;
                }
            };

            if file_name.starts_with(".") || (!file_name.ends_with(".pattern") && !md.is_dir()) {
                //If it is a hidden file or it is a file/symlink without pattern extension
                continue;
            }
            let name = if md.is_dir() {
                //If it is a directory(i.e module pattern), get the name from the whole dir name
                file_name
            } else {
                //If it is a file, get name from splitting extension
                match file_name.rsplit_once(".pattern") {
                    None => file_name,
                    Some((prefix, _)) => String::from(prefix),
                }
            };

            vec.push(PatternFile { name, path });
        }
        vec.sort_by(|a, b| {
            a.get_name()
                .to_lowercase()
                .cmp(&b.get_name().to_lowercase())
        });
        vec
    }

    pub fn fill(&self, theme: &Theme, is_inverted: bool, user_config: &UserConfig, dry_run: bool) {
        info!(
            "Filling |{}| pattern with |{}| theme...",
            self.get_name(),
            theme.get_name()
        );

        //If there are submodules
        if let Some(submodules) = self.get_submodules() {
            for submodule in submodules {
                submodule
                    .to_pattern()
                    .fill(theme, is_inverted, user_config, dry_run);
            }
            return;
        }

        // if pattern has no submodules (i.e, is a file)
        let filled_content = self.fill_values(theme, is_inverted, user_config);

        let output_path = match self.get_output() {
            Some(output_path) => output_path,
            None => {
                error!("Pattern |{}| does not have output file specified (hint: <[output-file]>=/path/to/output/file)", self.get_name());
                return;
            }
        };

        // Return if dry_run mode. i.e, dont write content to output path
        if !dry_run {
            if let Err(e) = core::utils::write_content_to(&filled_content, Path::new(output_path)) {
                error!("Could not create |{}|: |{}|", output_path, e);
            }
        }
    }

    pub fn fill_values(
        &self,
        theme: &Theme,
        is_inverted: bool,
        user_config: &UserConfig,
    ) -> String {
        let pattern_content = self.get_content().as_ref().unwrap();
        let pattern_name = self.get_name();

        let mut extended_keys = BTreeMap::new();

        extended_keys.extend(user_config.get_properties().clone().into_iter());
        // Ensure that user_config does not overwrites theme keys
        extended_keys.extend(theme.get_colors().clone().into_iter());
        extended_keys.insert("theme-name".to_string(), theme.get_name().to_string());

        let re = Regex::new(r"<\[((?:\w|-)+)?(?:\|(.*))?\]>").unwrap();

        let result =re.replace_all(pattern_content,|captured:&Captures|{

			// Check if there is no property key on the match
			let property = match captured.get(1) {
				None => {
					let whole_capture = captured.get(0).unwrap();
					warn!("There is an empty property (<[]>) in pattern |{pattern_name}|: content |byte offset {}|",
						whole_capture.start());
					return whole_capture.as_str().to_string();
				},
				Some(value) => value.as_str()
			};

			// Invert background colors if needed
			let property = Pattern::get_real_property(property,is_inverted);

			if let Some(value) = extended_keys.get(property){
				return value.to_string()
			}else{
				// Check for a default value for missing property
				match captured.get(2){
					Some(value) => {
						let default_value = value.as_str();
						info!("Filled property |{property}| with default value |{default_value}| in pattern |{pattern_name}|");
						return default_value.to_string()
					},
					None=> {
						let whole_capture = captured.get(0).unwrap();
						warn!("Could not fill property |{property}| in pattern |{pattern_name}|: content |byte offset {}|",
							whole_capture.start());
						return whole_capture.as_str().to_string()
					}
				}
			}
		}).to_string();

        return result;
    }

    // This inverts background and foreground key colors.
    fn get_real_property(property: &str, is_inverted: bool) -> &str {
        if is_inverted {
            match property {
                "foreground" => "background",
                "background" => "foreground",
                "selection-foreground" => "selection-background",
                "selection-background" => "selection-foreground",
                _ => property,
            }
        } else {
            property
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatternFile {
    name: String,
    // TODO: Change path type to PathBuf
    path: String,
}
impl PatternFile {
    pub fn to_pattern(&self) -> Pattern {
        Pattern::from(self)
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_path(&self) -> &String {
        &self.path
    }
}

// #[cfg(test)]
// mod tests{

// }
