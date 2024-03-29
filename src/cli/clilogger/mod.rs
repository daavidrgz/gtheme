use chrono::Local;
use colored::*;
use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::{fs, io::Write, path::Path};

use crate::core;

pub struct CliLogger {
    pub level: Level,
}

impl Log for CliLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Blue,
                Level::Debug => Color::White,
                _ => unreachable!("Unhandled log level"),
            };

            print!("{} • ", record.level().to_string().color(color).bold());

            let body = record.args().to_string();
            let body_split = body.split('|');
            for (it, strip) in body_split.into_iter().enumerate() {
                match it % 2 {
                    0 => print!("{}", strip),
                    1 => print!("{}", strip.color(color).bold()),
                    _ => (),
                }
            }
            println!("");
        }

        let log_path: String = format!("{}/logs/gtheme.log", core::expand_path(core::GTHEME_MISC));
        let mut log_file = FileRotate::new(
            log_path,
            AppendCount::new(2),
            ContentLimit::Lines(1000),
            Compression::None,
        );

        let time = Local::now().format("%H:%M:%S %Y-%m-%d");
        let record_text = record.args().to_string().replace("|", "");
        let plain_text = format!("[{}] {}: {}\n", time, record.level(), record_text);

        let _ = log_file.write_all(plain_text.as_bytes());
    }

    fn flush(&self) {}
}

impl CliLogger {
    pub fn init_logger(verbose_level: u64) {
        let log_dir = Path::new(&core::expand_path(core::GTHEME_MISC)).join("logs");
        let _ = fs::create_dir_all(&log_dir);

        log::set_max_level(LevelFilter::Debug);
        if verbose_level == 0 {
            static CLI_LOGGER: CliLogger = CliLogger { level: Level::Warn };
            log::set_logger(&CLI_LOGGER).unwrap();
        } else if verbose_level == 1 {
            static CLI_LOGGER: CliLogger = CliLogger { level: Level::Info };
            log::set_logger(&CLI_LOGGER).unwrap();
        } else {
            static CLI_LOGGER: CliLogger = CliLogger {
                level: Level::Debug,
            };
            log::set_logger(&CLI_LOGGER).unwrap();
        }
    }
}
