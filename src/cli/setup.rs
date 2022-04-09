use colored::*;
use std::{process::{Command, Stdio}, io::{self, Write}};
use log::error;

use crate::core::config::UserConfig;

#[derive(Eq,PartialEq)]
enum Section {
	Monitor,
	Battery,
	Network,
	Others
}

impl Section {
	fn to_string(&self) -> &str {
		match self {
			Section::Monitor => "Monitor settings",
			Section::Battery => "Battery",
			Section::Network => "Network",
			Section::Others => "Others"
		}
	}

	fn run(&self, user_config: &mut UserConfig) {
		match self {
			Section::Monitor => Self::monitor_section(user_config),
			Section::Battery => Self::battery_section(user_config),
			Section::Network => Self::network_section(user_config),
			Section::Others => Self::others_section(user_config)
		}
	}

	fn process_select_input(elements: &Vec<(String, String)>) -> Option<String> {
		let length = elements.len();

		if length == 0 {
			print!("\n{} Press enter to continue... ", "Could not find any option.".yellow());
			io::stdout().flush().unwrap();
			match io::stdin().read_line(&mut String::new()) {
				Ok(_) => (),
				Err(e) => println!("\n{} {}\n", "Error while reading input: ".red().bold(), e)
			}
			return None
		}

		for (i,(e, desc)) in elements.iter().enumerate() {
			println!("{} {} {}", format!("{})", i+1).bold().green(), e.bold(), desc.bold())
		}
		println!("{} {}", format!("{})", length+1).bold().green(), "[None]".bold());
		println!("");

		print!("Select one option: ");
		loop {
			let mut option_str = String::new();
			io::stdout().flush().unwrap();
			match io::stdin().read_line(&mut option_str) {
				Ok(_) => (),
				Err(e) => println!("\n{} {}\n", "Error while reading input: ".red().bold(), e)
			}
			option_str = option_str.trim().to_string();

			match option_str.parse::<usize>() {
				Ok(i) => {
					if i > 0 && i <= length + 1 {
						return match elements.get(i-1) {
							Some((e,_)) => Some(e.clone()),
							None => None
						}
					}
				}
				Err(_) => ()
			}

			print!("{} try again: ", "Invalid option,".red().bold())
		}
	}

	fn process_type_input(default_value: Option<String>, validate: fn(&String) -> Result<(), String>) -> Option<String> {
		println!("{} {}\n", "Default option:".bold().green(), default_value.clone().unwrap_or("[None]".to_string()).bold());

		print!("Type (leave empty to use default): ");
		loop {
			let mut input_str = String::new();
			io::stdout().flush().unwrap();
			match io::stdin().read_line(&mut input_str) {
				Ok(_) => (),
				Err(e) => println!("\n{} {}\n", "Error while reading input: ".red().bold(), e)
			}
			input_str = input_str.trim().to_string();

			if input_str.is_empty() { return default_value }

			match validate(&input_str) {
				Ok(_) => return Some(input_str),
				Err(e) => print!("{}, try again: ", e.red().bold())
			}
		}
	}

	fn select_question(question: &str, elements: &Vec<(String,String)>, key: &str, user_config: &mut UserConfig) {
		println!("\n{}", format!("-> {}:", question).magenta().bold());
		let selection = Self::process_select_input(elements);
		if let Some(value) = selection {
			user_config.set_property(key, &value);
		}
	}

	fn type_question(question: &str, default_value: Option<String>, validate: fn(&String) -> Result<(), String>,
		key: &str, user_config: &mut UserConfig) {

		println!("\n{}", format!("-> {}:", question).magenta().bold());
		let input_opt = Self::process_type_input(default_value, validate);
		if let Some(value) = input_opt {
			user_config.set_property(key, &value);
		}
	}

	fn awk(content: &String, index: usize) -> Vec<String> {
		let mut connected = vec![];
		for line in content.trim().split('\n') {
			let words: Vec<&str> = line.split_whitespace().collect();
			if let Some(display) = words.get(index) {
				connected.push(display.to_string());
			}
		}
		connected
	}

	fn pipeline(commands: &Vec<(&str, Vec<&str>)>) -> (Option<i32>, String) {
		if commands.len() == 0 { return (None, String::new()) }

		let mut stdin = Stdio::inherit();
		for (command, args) in commands.iter().take(commands.len()-1) {
			let mut output = match Command::new(command).args(args)
				.stdin(stdin)
				.stdout(Stdio::piped()).spawn() {
					Ok(o) => o,
					Err(e) => {
						error!("Could not execute {}: {}", command, e);
						return (None, String::new())
					}
				};

			stdin = Stdio::from(output.stdout.take().unwrap());
		}

		let (last_command, last_args) = commands.last().unwrap();
		let last_output = match Command::new(last_command).args(last_args)
			.stdin(stdin).output() {
				Ok(o) => o,
				Err(e) => {
					error!("Could not execute {}: {}", last_command, e);
					return (None, String::new())
				}
			};

		let output_str = match String::from_utf8(last_output.stdout) {
			Ok(s) => s,
			Err(e) => {
				error!("Error while converting stdout content to string: {}", e);
				return (None, String::new())
			}
		};
		let error_code = last_output.status.code();
		(error_code, output_str)
	}

	fn monitor_section(user_config: &mut UserConfig) {
		// MONITORS
		let connected_cmd = vec![
			("xrandr", vec![]),
			("grep", vec![" connected"])
		];
		let (_, connected_content) = Self::pipeline(&connected_cmd);
		let connected = Self::awk(&connected_content, 0);
		
		let disconnected_cmd = vec![
			("xrandr", vec![]),
			("grep", vec![" disconnected"])
		];
		let (_, disconnected_content) = Self::pipeline(&disconnected_cmd);
		let disconnected = Self::awk(&disconnected_content, 0);

		let monitors_print: Vec<(String,String)> = connected.into_iter()
			.map(|i| (i, "(connected)".to_string()))
			.chain(disconnected.into_iter().map(|i| (i, "".to_string())))
			.collect();

		Self::select_question(
			"Select main monitor output (for more info see 'xrandr')",
			&monitors_print,
			"monitor",
			user_config
		);

		Self::select_question(
			"Select monitor fallback output",
			&monitors_print,
			"monitor-fallback",
			user_config
		);

		// BACKLIGHT
		let backlight_cmd = vec![
			("ls", vec!["/sys/class/backlight", "-1"])
		];
		let (_, backlight_content) = Self::pipeline(&backlight_cmd);
		let backlight_cards = Self::awk(&backlight_content,  0);
		let backlight_print: Vec<(String,String)> = backlight_cards.into_iter().map(|i| (i, "".to_string())).collect();
		
		Self::select_question(
			"Select backlight card for brightness control (for more info see 'brightnessctl')",
			&backlight_print,
			"backlight-card",
			user_config
		);
	}

	fn battery_section(user_config: &mut UserConfig) {
		let power_path = "/sys/class/power_supply";

		// BATTERY ID
		let power_cmd = vec![
			("ls", vec![power_path, "-1"]),
		];
		let (_, power_content) = Self::pipeline(&power_cmd);
		let power = Self::awk(&power_content,  0);

		let mut batteries = vec![];
		let mut adapters = vec![];

		for power_item in power{
			let type_path = format!("{}/{}/type",power_path,power_item);
			let type_cmd = vec![
				("cat", vec![type_path.as_str()])
			];
			let (_, mut power_item_type) = Self::pipeline(&type_cmd);
			power_item_type = power_item_type.trim().to_string();
			if power_item_type == "Battery" {
				batteries.push(power_item);
			} else if power_item_type == "Mains" {
				adapters.push(power_item);
			}
		}

		let battery_print: Vec<(String,String)> = batteries.into_iter().map(|i| (i, "".to_string())).collect();

		Self::select_question(
			"Select battery (for more info see 'upower -d')",
			&battery_print,
			"battery",
			user_config
		);

		// BATTERY ADAPTER
		let battery_adp_print: Vec<(String,String)> = adapters.into_iter().map(|i| (i, "".to_string())).collect();

		Self::select_question(
			"Select battery adapter",
			&battery_adp_print,
			"battery-adapter",
			user_config
		);
	}

	fn network_section(user_config: &mut UserConfig) {
		// NETWORK INTERFACE
		let ifs_cmd = vec![
			("ls", vec!["/sys/class/net", "-1"])
		];
		let (_, ifs_content) = Self::pipeline(&ifs_cmd);
		let ifs = Self::awk(&ifs_content,  0);
		let ifs_print: Vec<(String,String)> = ifs.into_iter().map(|i| (i, "".to_string())).collect();
		
		Self::select_question(
			"Select main network interface",
			&ifs_print,
			"network-if",
			user_config
		);
	}

	fn others_section(user_config: &mut UserConfig) {
		fn validate_program(program: &String) -> Result<(),String> {
			let program_cmd = vec![
				("which", vec![program.as_str()])
			];
			let (exit_code, _) = Section::pipeline(&program_cmd);
			return match exit_code {
				Some(c) => if c == 0 { Ok(()) } else { Err(format!("There is no program called '{}'", program)) },
				None => Err("Could not validate program".to_string())
			}
		}

		fn validate_font(font: &String)-> Result<(),String> {
			let font_cmd = vec![
				("fc-list", vec!["-q", &font])
			];
			let (exit_code, _) = Section::pipeline(&font_cmd);
			return match exit_code {
				Some(c) => if c == 0 { Ok(()) } else { Err(format!("There is no font called '{}'", font)) },
				None => Err("Could not validate font".to_string())
			}
		}

		fn validate_font_size(font_size: &String) -> Result<(),String> {
			match font_size.parse::<u32>(){
				Ok(_) => Ok(()),
				Err(e) => Err(format!("Invalid font size: {}", e	))
			}
		}

		Self::type_question(
			"Select default terminal emulator",
			None,
			validate_program,
			"terminal",
			user_config
		);

		Self::type_question(
			"Select default browser",
			None,
			validate_program,
			"browser",
			user_config
		);

		Self::type_question(
			"Select default font family (this will overwrite specific desktop fonts)",
			None,
			validate_font,
			"default-font",
			user_config
		);

		Self::type_question(
			"Select default font size",
			None,
			validate_font_size,
			"default-font-size",
			user_config
		);
	}
}

struct Setup {
	sections: Vec<Section>
}

impl Setup {
	pub fn new() -> Self {
		let sections = vec![
			Section::Monitor,
			Section::Battery,
			Section::Network,
			Section::Others
		];

		Setup { sections }
	}

	pub fn print_sections(&self, current_section: &Section) {
		for s in &self.sections {
			if current_section == s {
				println!("{}", format!("• {}", s.to_string()).bold().yellow())
			} else {
				println!("{}", format!("• {}", s.to_string()))
			}
		}
	}

	pub fn run_setup(&self) {
		Self::clear_screen();

		let mut user_config = UserConfig::default();

		let length = self.sections.len();
		for (idx, section) in self.sections.iter().enumerate() {
			println!("{} {}\n", "GTHEME SETUP".underline().bold(), format!("[{}/{}]", idx+1, length).bold().yellow());
			self.print_sections(section);
			section.run(&mut user_config);
			Self::clear_screen();
		}
		user_config.save();
	}

	fn clear_screen() {
		match Command::new("clear")
		.stdout(Stdio::inherit())
		.output() {
			Ok(_) => (),
			Err(e) => println!("ERROR Error while clearing terminal: {}", e)
		};
	}
}

pub fn start() {
	if let Err(e) =super::commands::generate_completions(){
		println!("{e}")
	}

	if UserConfig::exists() {
		let mut option_str = String::new();
		print!("{} already exists. Do you want to {}? (y/[N]): ", "User config".bold().yellow(), "overwrite it".bold().yellow());
		io::stdout().flush().unwrap();
		match io::stdin().read_line(&mut option_str) {
			Ok(_) => (),
			Err(e) => println!("\n{} {}\n", "Error while reading input: ".red().bold(), e)
		}
		match option_str.trim() {
			"y" | "yes" => (),
			_ => return
		}
	}

	Setup::new().run_setup();
}
