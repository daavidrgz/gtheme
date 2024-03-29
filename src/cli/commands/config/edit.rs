use crate::{cli::commands::utils, core::config::UserConfig};

pub fn run() {
    let user_settings = UserConfig::new();
    utils::edit_file(&user_settings.get_path());
}
