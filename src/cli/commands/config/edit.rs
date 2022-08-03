use crate::cli::utils;
use crate::core::config::UserConfig;
use log::error;

pub fn run() {
    if !UserConfig::exists() {
        error!("|There is no global settings file|, run |gtheme config setup| first");
        return;
    }
    let user_settings = UserConfig::new();
    utils::edit_file(&user_settings.get_path());
}
