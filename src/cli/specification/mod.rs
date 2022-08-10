mod config;
mod desktop;
mod extra;
mod fav;
mod pattern;
mod theme;

use clap::{Arg, Command};

pub fn create_app<'a>(
    themes: &'a [&'a str],
    desktops: &'a [&'a str],
    patterns: &'a [&'a str],
    fav_themes: &'a [&'a str],
    extras: &'a [&'a str],
) -> Command<'a> {
    let mut app = Command::new("gtheme")
        .version("1.0")
        .about("A rust program that makes your theming life so much easier.")
        .author("David Rodriguez & Jorge Hermo")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .global(true)
                .multiple_occurrences(true)
                .help("Show more information"),
        );

    app = config::init(app);
    app = desktop::init(app, desktops, themes);
    app = theme::init(app, themes, patterns);
    app = pattern::init(app, patterns, desktops);
    app = extra::init(app, extras, desktops);
    app = fav::init(app, fav_themes, themes);

    app
}

const EMPTY_SLICE: &[&str] = &[];
pub fn create_app_no_suggestions<'a>() -> Command<'a> {
    create_app(
        EMPTY_SLICE,
        EMPTY_SLICE,
        EMPTY_SLICE,
        EMPTY_SLICE,
        EMPTY_SLICE,
    )
}
