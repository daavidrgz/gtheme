pub mod widgets;
pub mod screenitem;
pub mod appstate;

use std::io;
use std::{time::Duration, error::Error};
use tui::{
	backend::CrosstermBackend,
	layout::{Layout, Constraint, Direction},
	Terminal,
	Frame
};
use crossterm::{
	event::{self, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::app::widgets::{ListWidget, LogoWidget};
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
		let mut app_state = AppState::default();

		loop {
			self.terminal.draw(|f| Ui::draw_ui(f, &mut app_state))?;
			if !Ui::manage_input(&mut app_state) {return Ok(())}
		}
	}

	fn manage_input(app_state: &mut AppState) -> bool {
		let (current_screen, map) = app_state.get_state();
		let lists = map.get_mut(&current_screen).unwrap();

		let current_list = if lists[0].is_selected() {0} else {1};

		if !crossterm::event::poll(Duration::from_millis(250)).unwrap() {
			return true
		}
			
		if let Event::Key(key) = event::read().unwrap() {
			match key.code {
				KeyCode::Char('q') | KeyCode::Char('Q') => return false,
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
					let screen = if *app_state.get_screen() == Screen::Desktop {Screen::Theme} else {Screen::Desktop};
					app_state.set_screen(screen)
				},
				KeyCode::Enter => {
					lists[current_list].get_selected().unwrap().apply()
				},
				_ => {}
			}
		}
		true
	}

	fn draw_ui(f: &mut Frame<CrosstermBackend<io::Stdout>>, app_state: &mut AppState) {
		let padding = 2;

		let mut logo_container = f.size();
			logo_container.height = 6;
		let mut main_container = f.size();
		main_container.height = main_container.height + logo_container.height + padding;
	
		let h_box = Layout::default()
			.direction(Direction::Horizontal)
			.vertical_margin(logo_container.height + padding)
			.constraints([Constraint::Percentage(50),Constraint::Percentage(50)].as_ref())
			.split(main_container);
		
		let logo_widget = LogoWidget::new();
		f.render_widget(logo_widget.get_widget(), logo_container);

		let (current_screen, map) = app_state.get_state();
		let lists = map.get_mut(&current_screen).unwrap();

		let widget_list_1 = ListWidget::new(&lists[0]);
		let widget_list_2 = ListWidget::new(&lists[1]);

		f.render_stateful_widget(widget_list_1.get_widget(), h_box[0], lists[0].get_state_mut());
		f.render_stateful_widget(widget_list_2.get_widget(), h_box[1], lists[1].get_state_mut());
	}
}
