use crate::core::config::UserConfig;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    let key = matches.value_of("key").unwrap();
    let value = matches.value_of("value").unwrap();

    let mut user_settings = UserConfig::new();
    user_settings.set_property(key, value);
    user_settings.save();
}
