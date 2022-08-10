use clap::Values;
use log::{error, info, warn};
use std::collections::BTreeMap;
use std::env;
use std::process::{Command, Stdio};

use crate::core::{
    config::{DesktopConfig, GlobalConfig},
    desktop::{Desktop, DesktopFile},
    pattern::Pattern,
};

pub enum Action {
    Enable,
    Disable,
    Toggle,
}

pub fn explore_directory(path: &str) {
    match env::var("FILE_EXPLORER") {
		Ok(value) => if value.is_empty() {
			warn!("Env var |$FILE_EXPLORER| is empty, using |ranger| instead |(try exporting env var FILE_EXPLORER in your shell config)|")
		},
		Err(_) => warn!("Could not found env var |$FILE_EXPLORER|, using |ranger| instead |(try exporting env var FILE_EXPLORER in your shell config)|")
	}

    info!("Reading |{}|...", path);

    match Command::new("sh")
        .arg("-c")
        .arg(format!("${{FILE_EXPLORER:-ranger}} {}", path))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
    {
        Ok(output) => match output.status.success() {
            true => info!("Directory |{}| readed succesfully", path),
            false => error!(
                "Could not read |{}|, error: |{}|",
                path,
                String::from_utf8(output.stderr).unwrap()
            ),
        },
        Err(e) => error!("Could not read |{}|, error: |{}|", path, e),
    }
}

pub fn edit_file(path: &str) {
    match env::var("EDITOR") {
        Ok(value) => {
            if value.is_empty() {
                warn!("Env var |$EDITOR| is empty, using |nano| instead")
            }
        }
        Err(_) => warn!("Could not found env var |$EDITOR|, using |nano| instead"),
    }

    info!("Editing |{}|...", path);

    match Command::new("sh")
        .arg("-c")
        .arg(format!("${{EDITOR:-nano}} {}", path))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
    {
        Ok(output) => match output.status.success() {
            true => info!("File |{}| edited succesfully", path),
            false => error!(
                "Could not edit |{}|, error: |{}|",
                path,
                String::from_utf8(output.stderr).unwrap()
            ),
        },
        Err(e) => error!("Could not edit |{}|, error: |{}|", path, e),
    }
}

pub fn get_desktop(desktop_opt: Option<&str>) -> Option<DesktopFile> {
    match desktop_opt {
        Some(desktop_str) => Desktop::get_by_name(desktop_str),
        None => {
            let global_config = GlobalConfig::new();
            match global_config.get_current_desktop() {
                Some(d) => Some(d.clone()),
                None => {
                    error!("|There is no desktop installed!| Try with -d option instead");
                    None
                }
            }
        }
    }
}

pub fn get_actived(
    values_opt: Option<Values>,
    current_desktop: &DesktopFile,
    desktop_config: &DesktopConfig,
) -> BTreeMap<String, bool> {
    let mut actived: BTreeMap<String, bool> = BTreeMap::new();
    match values_opt {
        Some(patterns) => {
            for p in patterns {
                if Pattern::get_by_name(current_desktop, p).is_some() {
                    actived.insert(p.to_string(), true);
                };
            }
        }
        None => actived = desktop_config.get_actived().clone(),
    }
    actived
}

pub fn get_inverted(
    values_opt: Option<Values>,
    current_desktop: &DesktopFile,
    desktop_config: &DesktopConfig,
) -> BTreeMap<String, bool> {
    let mut inverted: BTreeMap<String, bool> = desktop_config.get_inverted().clone();
    if let Some(patterns) = values_opt {
        for p_str in patterns {
            if let Some(p) = Pattern::get_by_name(current_desktop, p_str) {
                if let Some(default_inverted) = inverted.get_mut(p.get_name()) {
                    *default_inverted = !*default_inverted;
                } else {
                    inverted.insert(p.get_name().to_string(), true);
                }
            };
        }
    }
    inverted
}
