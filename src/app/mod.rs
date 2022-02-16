pub mod widgets;

use std::collections::HashMap;
use std::io;
use std::{time::Duration, error::Error};
use tui::{
	backend::CrosstermBackend,
	layout::{Layout, Constraint, Direction},
	style::Color,
	Terminal,
	Frame
};
use crossterm::{
	event::{self, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::core::{
	desktop::{DesktopFile, Desktop},
	theme::{ThemeFile, Theme},
	pattern::{PatternFile, Pattern}
};
use crate::app::widgets::{ListWidget, LogoWidget,StatefulList};

#[derive(Eq, PartialEq, Hash)]
enum Screen {
	Desktop,
	Theme
}

pub enum ScreenItem {
	Desktop(DesktopFile),
	Theme(ThemeFile),
	Pattern(PatternFile)
}
impl ScreenItem {
	pub fn get_name(&self) -> &str {
		match self {
			ScreenItem::Desktop(d) => d.get_name(),
			ScreenItem::Theme(t) => t.get_name(),
			ScreenItem::Pattern(p) => p.get_name()
		}
	}
}

struct AppState<T> {
	current_screen: Screen,
	lists: HashMap<Screen, [StatefulList<T>; 2]>
}
impl<T> AppState<T> {
	pub fn new(lists: HashMap<Screen, [StatefulList<T>; 2]>) -> AppState<T> {
		AppState {
			current_screen: Screen::Desktop,
			lists
		}
	}
	
	pub fn get_state(&mut self) -> (&mut Screen, &mut HashMap<Screen, [StatefulList<T>; 2]>) {
		(&mut self.current_screen, &mut self.lists)
	}
}

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

		let res = self.run_app();
		self.exit_ui();

		if let Err(err) = res {
			println!("{:?}", err)
		}

		Ok(())
	}

	fn exit_ui(&mut self) -> Result<(), Box<dyn Error>>  {
		disable_raw_mode()?;
		execute!(self.terminal.backend_mut(), LeaveAlternateScreen,)?;
		self.terminal.show_cursor()?;

		Ok(())
	}

	fn run_app(&mut self) -> Result<(), Box<dyn Error>> {
		let desktops = Desktop::get_desktops().into_iter().map(|e|ScreenItem::Desktop(e)).collect();
		let desktops_list = StatefulList::with_items(desktops, true);

		let patterns = Pattern::get_patterns("simple").into_iter().map(|e|ScreenItem::Pattern(e)).collect();
		let patterns_list = StatefulList::with_items(patterns, false);

		let themes = Theme::get_themes().into_iter().map(|e|ScreenItem::Theme(e)).collect();
		let themes_list = StatefulList::with_items(themes, true);

		let fav_themes = Theme::get_themes().into_iter().map(|e|ScreenItem::Theme(e)).collect();
		let fav_themes_list = StatefulList::with_items(fav_themes, false);

		let mut map = HashMap::new();
		map.insert(Screen::Desktop, [desktops_list, patterns_list]);
		map.insert(Screen::Theme, [themes_list, fav_themes_list]);

		let mut app_state = AppState::new(map);

		loop {
			self.terminal.draw(|f| Ui::draw_ui(f, &mut app_state))?;
			
			let (current_screen, map) = app_state.get_state();
			let lists = map.get_mut(current_screen).unwrap();

			let current_list = if lists[0].is_selected() {0} else {1};

			if crossterm::event::poll(Duration::from_millis(250))? {
				if let Event::Key(key) = event::read()? {
					match key.code {
						KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
						KeyCode::Down => lists[current_list].next(),
						KeyCode::Up => lists[current_list].previous(),
						KeyCode::Char('n') => {
							lists[current_list].unselect();
							lists[(current_list + 1) % 2].next();
						},
						_ => {}
					}
				}
			}
		}
	}

	fn draw_ui(f: &mut Frame<CrosstermBackend<io::Stdout>>, app_state: &mut AppState<ScreenItem>) {
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

		let widget_list_1 = ListWidget::new("DESKTOPS", Color::LightCyan, &lists[0]);
		let widget_list_2 = ListWidget::new("PATTERNS", Color::Magenta, &lists[1]);

		f.render_stateful_widget(widget_list_1.get_widget(), h_box[0], lists[0].get_state());
		f.render_stateful_widget(widget_list_2.get_widget(), h_box[1], lists[1].get_state());
	}
}
