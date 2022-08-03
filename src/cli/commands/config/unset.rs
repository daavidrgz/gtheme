use crate::core::config::UserConfig;
use clap::ArgMatches;
use log::error;

pub fn run(matches: &ArgMatches) {
    if !UserConfig::exists() {
        error!("|There is no global settings file|, run |gtheme config setup| first");
        return;
    }

    let key = matches.value_of("key").unwrap();

    let mut user_settings = UserConfig::new();
    user_settings.unset_property(key);
    user_settings.save();
}
