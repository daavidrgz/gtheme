pub mod widgets;
pub mod screenitem;
pub mod appstate;

use std::io;
use std::{time::Duration, error::Error};
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

use crate::app::widgets::{ListWidget, LogoWidget, OptionsWidget, HelpWidget};
use crate::app::screenitem::ScreenItem;
use crate::app::appstate::{AppState, Screen};

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
		Ok(())
	}

	fn manage_input(app_state: &mut AppState) -> bool {
		let (current_screen, map, global_config, show_popup) = app_state.get_mut_state();
		let lists = map.get_mut(&current_screen).unwrap();

		let current_list = if lists[0].is_selected() {0} else {1};

		if !crossterm::event::poll(Duration::from_millis(250)).unwrap() {
			return true
		}
			
		if let Event::Key(key) = event::read().unwrap() {
			match key.code {
				KeyCode::Char('q') | KeyCode::Char('Q') => return false,
				KeyCode::Char('h') | KeyCode::Char('H') => *show_popup = !*show_popup,
				KeyCode::Down => lists[current_list].next(),
				KeyCode::Up => lists[current_list].previous(),
				KeyCode::Left => {
					if current_list != 0 {
						lists[current_list].unselect();
						lists[current_list - 1].next();
					}
				},
				KeyCode::Right => {
					if current_list != 1 {
						lists[current_list].unselect();
						lists[current_list + 1].next();
					}
				},
				KeyCode::Tab => {
					let screen = if *app_state.get_mut_screen() == Screen::Desktop {Screen::Theme} else {Screen::Desktop};
					*app_state.get_mut_screen() = screen
				},
				KeyCode::Enter => lists[current_list].get_selected().unwrap().apply(global_config),
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
				}
				_ => {}
			}
		}
		true
	}

	fn draw_ui(f: &mut Frame<CrosstermBackend<io::Stdout>>, app_state: &mut AppState) {
		let (current_screen, map, global_config, show_popup) = app_state.get_mut_state();
		let lists = map.get_mut(&current_screen).unwrap();

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

		let mut logo_container = f.size();
			logo_container.height = 6;
			logo_container.width = logo_container.width / 2;

		let mut options_container = f.size();
			options_container.height = 6;
			options_container.width = options_container.width / 2 - h_padding;
			options_container.x = logo_container.width + h_padding;

		let mut main_container = f.size();
			main_container.height = main_container.height + logo_container.height + v_padding;
	
		let h_box = Layout::default()
			.direction(Direction::Horizontal)
			.vertical_margin(logo_container.height + v_padding)
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
		let widget_list_1 = ListWidget::new(&lists[0], global_config);
		let widget_list_2 = ListWidget::new(&lists[1], global_config);
		f.render_stateful_widget(widget_list_1.get_widget(), h_box[0], lists[0].get_state_mut());
		f.render_stateful_widget(widget_list_2.get_widget(), h_box[1], lists[1].get_state_mut());

		// Help
		if *show_popup {
			let help_widget = HelpWidget::new();
			let area = Self::centered_rect(60, 70, f.size());
			f.render_widget(Clear, area); //this clears out the background
			f.render_widget(help_widget.get_widget(), area);
		}
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
