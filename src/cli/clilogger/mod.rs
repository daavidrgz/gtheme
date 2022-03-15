use log::{Record, Level, Metadata, Log};
use colored::*;
use std::{fs::{File, OpenOptions}, io::Write};

use crate::core;

pub struct CliLogger;

impl Log for CliLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return
		}

		let color = match record.level() {
			Level::Error => Color::Red,
			Level::Warn => Color::Yellow,
			Level::Info => Color::Blue,
			_ => Color::Green
		};

		print!("{} • ", record.level().to_string().color(color).bold());

		let body = record.args().to_string();
		let body_split = body.split('|');
		body_split.into_iter().enumerate().for_each(|(it, strip)| {
			match it % 2 {
				0 => print!("{}", strip),
				1 => print!("{}", strip.color(color).bold()),
				_ => ()
			}
		});
		println!("\n")

		let log_path: String = format!("{}/gtheme.log", core::expand_path(core::GTHEME_HOME));
		let log_file: File = match OpenOptions::new().create(true).write(true).open(&log_path) {
			Ok(f) => f,
			Err(e) => return
		};

		let plain_text = format!("{} - {}", record.level(), record.args());
		log_file.write_all(plain_text.as_bytes());
	}

	fn flush(&self) {}
}
