pub mod widgets;
pub mod screenitem;
pub mod appstate;
pub mod statefullist;

use std::io;
use std::{time::Duration, error::Error};
use log::*;
use tui::{
	backend::CrosstermBackend,
	widgets::Clear,
	layout::{Layout, Constraint, Direction, Rect},
	Terminal,
	Frame
};
use crossterm::{
	event::{self, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::core::config::GlobalConfig;

use crate::app::widgets::{ListWidget, LogoWidget, OptionsWidget, HelpWidget, LoggerWidget};
use crate::app::appstate::{AppState, Screen, Popup};

pub struct Ui {
	terminal: Terminal<CrosstermBackend<io::Stdout>>
}
impl Ui {
	pub fn default() -> Ui {
		let stdout = io::stdout();
		let backend = CrosstermBackend::new(stdout);
		Ui {
			terminal: Terminal::new(backend).unwrap(),
		}
	}

	pub fn start_ui(&mut self) -> Result<(), Box<dyn Error>> {
		enable_raw_mode()?;
		let mut stdout = io::stdout();
		execute!(stdout, EnterAlternateScreen)?;

		tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

		self.run_app()?;
		self.exit_ui()?;

		Ok(())
	}

	fn exit_ui(&mut self) -> Result<(), Box<dyn Error>> {
		disable_raw_mode()?;
		execute!(self.terminal.backend_mut(), LeaveAlternateScreen,)?;
		self.terminal.show_cursor()?;

		Ok(())
	}

	fn run_app(&mut self) -> Result<(), Box<dyn Error>> {
		let mut app_state = AppState::default(GlobalConfig::new());

		loop {
			self.terminal.draw(|f| Ui::draw_ui(f, &mut app_state))?;
			if !Ui::manage_input(&mut app_state) {break}
		}

		app_state.get_global_config().save();
		app_state.get_desktop_config().save();
		
		Ok(())
	}

	fn manage_input(app_state: &mut AppState) -> bool {
		let (current_screen, screens, current_popup, popups, show_logs, global_config, desktop_config) = app_state.get_mut_state();
		let lists = screens.get_mut(&current_screen).unwrap();

		let current_list = if lists[0].is_selected() {0} else {1};

		if !crossterm::event::poll(Duration::from_millis(250)).unwrap() {
			return true
		}
			
		if let Event::Key(key) = event::read().unwrap() {
			match key.code {
				KeyCode::Char('q') | KeyCode::Char('Q') => return false,
				KeyCode::Char('h') | KeyCode::Char('H') => {
					info!("Testing...");
					let help_list = popups.get_mut(&Popup::Help).unwrap();
					match current_popup {
						Some(Popup::Help) => {
							lists[0].next();
							help_list.unselect();
							*current_popup = None
						},
						Some(_) => {}
						None => {
							lists[0].unselect();
							lists[1].unselect();
							help_list.next();
							*current_popup = Some(Popup::Help)
						}
					}
				},
				KeyCode::Char('o') | KeyCode::Char('O') => {
					warn!("Alert");
					let extras_list = popups.get_mut(&Popup::Extras).unwrap();
					match current_popup {
						Some(Popup::Extras) => {
							lists[0].next();
							extras_list.unselect();
							*current_popup = None
						},
						Some(_) => {},
						None => {
							lists[0].unselect();
							lists[1].unselect();
							extras_list.next();
							*current_popup = Some(Popup::Extras)
						},
					}
				},
				KeyCode::Esc => {
					match current_popup {
						Some(p) => {
							let popup_list = popups.get_mut(p).unwrap();
							popup_list.unselect();
							lists[0].next();
							*current_popup = None
						},
						None => {}
					}
				}
				KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
					match current_popup {
						Some(p) => {
							let popup_list = popups.get_mut(p).unwrap();
							popup_list.next()
						},
						None => lists[current_list].next()
					}
				},
				KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
					match current_popup {
						Some(p) => {
							let popup_list = popups.get_mut(p).unwrap();
							popup_list.previous()
						},
						None => lists[current_list].previous()
					}
				},
				KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
					if current_list != 0 && *current_popup == None {
						lists[current_list].unselect();
						lists[current_list - 1].next();
					}
				},
				KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
					if current_list != 1 && *current_popup == None {
						lists[current_list].unselect();
						lists[current_list + 1].next();
					}
				},
				KeyCode::Tab => {
					if *current_popup == None {
						let screen = if *app_state.get_mut_screen() == Screen::Desktop {Screen::Theme} else {Screen::Desktop};
						*app_state.get_mut_screen() = screen
					}
				},
				KeyCode::Enter => {
					match current_popup {
						Some(p) => popups.get_mut(p).unwrap().get_selected().unwrap().apply(global_config, desktop_config),
						None => lists[current_list].get_selected().unwrap().apply(global_config, desktop_config),
					}
				},
				KeyCode::Char('f') | KeyCode::Char('F') => {
					let item = match lists[current_list].get_selected() {
						Some(item) => item.clone(),
						None => return true
					};
					if current_list == 0 { 
						lists.get_mut(0).unwrap().remove_fav(&item,global_config)
					} else {
						lists.get_mut(0).unwrap().add_fav(&item,global_config)
					}
				},
				KeyCode::Char('i') | KeyCode::Char('I') => {
					let item = match lists[current_list].get_selected() {
						Some(item) => item,
						None => return true
					};
					item.invert(desktop_config);
				},
				KeyCode::Char('e') | KeyCode::Char('E') => {
					match current_popup {
						Some(Popup::Extras) => popups.get_mut(&Popup::Extras).unwrap().get_selected().unwrap().edit(),
						Some(_) => {},
						None => lists[current_list].get_selected().unwrap().edit()
					}
				},
				KeyCode::Char('l') | KeyCode::Char('L') => *show_logs = !*show_logs,
				_ => {}
			}
		}
		true
	}

	fn draw_ui(f: &mut Frame<CrosstermBackend<io::Stdout>>, app_state: &mut AppState) {
		let (current_screen, screens, current_popup, popups, show_logs, global_config, desktop_config) = app_state.get_mut_state();
		let lists = screens.get_mut(&current_screen).unwrap();

		let theme = if *current_screen == Screen::Theme {
			let selected_theme = match lists.get(0).unwrap().get_selected() {
				None => None,
				Some(i) => Some(i.get_theme().unwrap().to_theme())
			};

			match lists.get(1).unwrap().get_selected() {
				None => selected_theme,
				Some(i) => Some(i.get_theme().unwrap().to_theme())
			}
		} else {
			None
		};

		// •• Layout definition ••

		let v_padding = 2;
		let h_padding = 4;

		let logs_height = if *show_logs {10} else {0};

		let mut logo_container = f.size();
			logo_container.height = 6;
			logo_container.width = logo_container.width / 2;

		let mut options_container = f.size();
			options_container.height = 6;
			options_container.width = options_container.width / 2 - h_padding;
			options_container.x = logo_container.width + h_padding;

		let mut main_container = f.size();
			main_container.y = logo_container.height + v_padding;
			main_container.height = main_container.height - (logo_container.height + logs_height + v_padding);

		let mut logs_container = f.size();
			logs_container.height = logs_height;
			logs_container.y = logo_container.height + main_container.height + v_padding;
	
		let h_box = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(50),Constraint::Percentage(50)].as_ref())
			.split(main_container);
		
		// •• Widget rendering ••

		// Logo
		let logo_widget = LogoWidget::new(theme);
		f.render_widget(logo_widget.get_widget(), logo_container);

		// Options
		let options_widget = OptionsWidget::new();
		f.render_widget(options_widget.get_widget(), options_container);

		// Lists
		let widget_list_1 = ListWidget::new(&lists[0], global_config, desktop_config);
		let widget_list_2 = ListWidget::new(&lists[1], global_config, desktop_config);
		f.render_stateful_widget(widget_list_1.get_widget(), h_box[0], lists[0].get_mut_state());
		f.render_stateful_widget(widget_list_2.get_widget(), h_box[1], lists[1].get_mut_state());

		// Help popup
		if *current_popup == Some(Popup::Help) {
			let help_list = popups.get_mut(&Popup::Help).unwrap();
			let help_widget = HelpWidget::new(help_list);
			let area = Self::centered_rect(60, 70, f.size());
			f.render_widget(Clear, area); //this clears out the background
			f.render_stateful_widget(help_widget.get_widget(), area, help_list.get_mut_state());
		}

		// Extras popup
		if *current_popup == Some(Popup::Extras) {
			let extras_list = popups.get_mut(&Popup::Extras).unwrap();
			let extras_widget = ListWidget::new(extras_list, global_config, desktop_config);
			let area = Self::centered_rect(60, 70, f.size());
			f.render_widget(Clear, area); //this clears out the background
			f.render_stateful_widget(extras_widget.get_widget(), area, extras_list.get_mut_state());
		}

		// Logger
		let logger_widget = LoggerWidget::new();
		f.render_widget(logger_widget.get_widget(),logs_container)
	}

	fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Percentage((100 - percent_y) / 2),
				Constraint::Percentage(percent_y),
				Constraint::Percentage((100 - percent_y) / 2),].as_ref())
			.split(r);

		Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage((100 - percent_x) / 2),
				Constraint::Percentage(percent_x),
				Constraint::Percentage((100 - percent_x) / 2)].as_ref(),
			)
			.split(popup_layout[1])[1]
	}
}
