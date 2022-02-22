use log::{Record, Level, Metadata, Log};
use colored::*;

pub struct CliLogger;

impl Log for CliLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
      metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
			if self.enabled(record.metadata()) {
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
			}
    }

    fn flush(&self) {}
}
