use crate::core::config::UserConfig;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    let key = matches.value_of("key").unwrap();

    let mut user_settings = UserConfig::new();
    user_settings.unset_property(key);
    user_settings.save();
}
