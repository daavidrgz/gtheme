pub mod widgets;
pub mod screenitem;
pub mod appstate;
pub mod statefullist;

use std::process::{Command, Stdio};
use std::io::{self, Stdout};
use std::time::Duration;
use log::LevelFilter;
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
use crate::tui::{
	screenitem::ScreenItem,
	widgets::{ListWidget, LogoWidget, HelpWidget, LoggerWidget, InfoWidget},
	appstate::{AppState, Screen, Popup}
};

const LEFT_LIST: usize = 0;
const RIGHT_LIST: usize = 1;

pub fn start_tui() {
	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend).unwrap();

	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen).unwrap();

	// Logger init
	tui_logger::init_logger(LevelFilter::Info).unwrap();
	tui_logger::set_default_level(log::LevelFilter::Info);

	run_app(&mut terminal);
	exit_ui(&mut terminal);
}

fn exit_ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
	disable_raw_mode().unwrap();
	execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
	terminal.show_cursor().unwrap();

	match Command::new("clear")
	.stdout(Stdio::inherit())
	.output() {
		Ok(_) => (),
		Err(e) => println!("ERROR Error while clearing terminal: {}", e)
	};
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
	let mut app_state = AppState::default(GlobalConfig::new());

	loop {
		terminal.draw(|f| draw_ui(f, &mut app_state)).unwrap();
		if !manage_input(&mut app_state, terminal) {break}
	}

	app_state.get_global_config().save();
	if let Some(desktop_config) = app_state.get_desktop_config() {
		desktop_config.save();
	};
}

fn manage_input(app_state: &mut AppState, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> bool {
	let (current_screen, screens, current_popup, popups,
		show_log, global_config, desktop_config) = app_state.get_mut_state();
	let lists = screens.get_mut(&current_screen).unwrap();

	let current_list = if lists[LEFT_LIST].is_selected() {LEFT_LIST} else {RIGHT_LIST};

	if !crossterm::event::poll(Duration::from_millis(2000)).unwrap() {
		return true
	}

	if let Event::Key(key) = event::read().unwrap() {
		match key.code {
			KeyCode::Char('q') | KeyCode::Char('Q') => return false,
			KeyCode::Char('h') | KeyCode::Char('H') => {
				let help_list = popups.get_mut(&Popup::Help).unwrap();
				match current_popup {
					Some(Popup::Help) => {
						lists[LEFT_LIST].next();
						help_list.unselect();
						*current_popup = None
					},
					Some(_) => {}
					None => {
						lists[LEFT_LIST].unselect();
						lists[RIGHT_LIST].unselect();
						help_list.next();
						*current_popup = Some(Popup::Help)
					}
				}
			},
			KeyCode::Char('o') | KeyCode::Char('O') => {
				let extras_list = popups.get_mut(&Popup::Extras).unwrap();
				match current_popup {
					Some(Popup::Extras) => {
						lists[LEFT_LIST].next();
						extras_list.unselect();
						*current_popup = None
					},
					Some(_) => {},
					None => {
						lists[LEFT_LIST].unselect();
						lists[RIGHT_LIST].unselect();
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
						lists[LEFT_LIST].next();
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
				if current_list != LEFT_LIST && *current_popup == None {
					lists[RIGHT_LIST].unselect();
					lists[LEFT_LIST].next();
				}
			},
			KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
				if current_list != RIGHT_LIST && *current_popup == None {
					lists[LEFT_LIST].unselect();
					lists[RIGHT_LIST].next();
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
					Some(p) => {
						if let Some(i) = popups.get_mut(p).unwrap().get_selected() {
							i.apply(global_config, desktop_config)
						}
					}
					None => {
						if let Some(i)= lists[current_list].get_selected() {
							i.apply(global_config, desktop_config)
						}
					}
				}
			},
			KeyCode::Char('f') | KeyCode::Char('F') => {
				let item = match lists[current_list].get_selected() {
					Some(item) => item.clone(),
					None => return true
				};
				if current_list == LEFT_LIST { 
					lists.get_mut(LEFT_LIST).unwrap().remove_fav(&item, global_config)
				} else {
					lists.get_mut(LEFT_LIST).unwrap().add_fav(&item, global_config);
				}
			},
			KeyCode::Char('i') | KeyCode::Char('I') => {
				match current_popup {
					Some(Popup::Info) => {
						lists[LEFT_LIST].next();
						*current_popup = None
					},
					Some(_) => (),
					None => {
						let item = match lists[current_list].get_selected() {
							Some(i) => i,
							None => return true
						};
						match item {
							ScreenItem::Pattern(_) => item.invert(desktop_config),
							ScreenItem::Desktop(_) => {
								let info_list = popups.get_mut(&Popup::Info).unwrap();
								item.create_desktop_info(info_list);
	
								lists[LEFT_LIST].unselect();
								lists[RIGHT_LIST].unselect();
								info_list.next();
								*current_popup = Some(Popup::Info)
							}
							_ => ()
						}
					}
				}
			},
			KeyCode::Char('e') | KeyCode::Char('E') => {
				let item = match current_popup {
					Some(popup) => match popups.get_mut(popup).unwrap().get_selected() {
						Some(i) => i,
						None => return true
					},
					None => match lists[current_list].get_selected() {
						Some(i) => i,
						None => return true
					}
				};
				match item.get_path() {
					Some(path) => ScreenItem::edit(&path),
					None => return true
				}
				terminal.clear().unwrap();
			},
			KeyCode::Char('p') | KeyCode::Char('P') => {
				let item = match lists[current_list].get_selected() {
					Some(i) => i,
					None =>  return true
				};
				match item.get_postscript_path(desktop_config) {
					Some(path) => ScreenItem::edit(&path),
					None => return true
				}
				terminal.clear().unwrap();
			},
			KeyCode::Char('z') | KeyCode::Char('Z') => {
				let item = match lists[current_list].get_selected() {
					Some(i) => i,
					None => return true
				};
				item.set_default_theme(desktop_config);
			},
			KeyCode::Char('l') | KeyCode::Char('L') => *show_log = !*show_log,
			_ => {}
		}
	}
	true
}

fn draw_ui(f: &mut Frame<CrosstermBackend<io::Stdout>>, app_state: &mut AppState) {
	let (current_screen, screens,
		current_popup, popups,
		show_log, global_config, desktop_config) = app_state.get_mut_state();

	let lists = screens.get_mut(&current_screen).unwrap();
	
	// Colors preview
	let current_list = if lists[LEFT_LIST].is_selected() {LEFT_LIST} else {RIGHT_LIST};
	let theme = match lists[current_list].get_selected() {
		Some(item) => {
			match item.get_theme() {
				Some(t) => Some(t.to_theme()),
				None => None
			} 
		},
		None => None
	};

	// •• Layout definition ••

	let v_padding = 2;
	let h_padding = 4;

	let logs_height = if *show_log {10} else {0};

	let mut logo_container = f.size();
		logo_container.height = 6;
		// logo_container.width = logo_container.width / 2;

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
	// let options_widget = OptionsWidget::new();
	// f.render_widget(options_widget.get_widget(), options_container);

	// Lists
	let widget_list_1 = ListWidget::new(&lists[0], global_config, desktop_config);
	let widget_list_2 = ListWidget::new(&lists[1], global_config, desktop_config);
	f.render_stateful_widget(widget_list_1.get_widget(), h_box[0], lists[0].get_mut_state());
	f.render_stateful_widget(widget_list_2.get_widget(), h_box[1], lists[1].get_mut_state());

	// Logger
	let logger_widget = LoggerWidget::new();
	f.render_widget(logger_widget.get_widget(),logs_container);

	// Help popup
	if *current_popup == Some(Popup::Help) {
		let help_list = popups.get_mut(&Popup::Help).unwrap();
		let help_widget = HelpWidget::new(help_list);
		let area = centered_rect(60, 70, f.size());
		f.render_widget(Clear, area);
		f.render_stateful_widget(help_widget.get_widget(), area, help_list.get_mut_state());
	}

	// Extras popup
	if *current_popup == Some(Popup::Extras) {
		let extras_list = popups.get_mut(&Popup::Extras).unwrap();
		let extras_widget = ListWidget::new(extras_list, global_config, desktop_config);
		let area = centered_rect(60, 70, f.size());
		f.render_widget(Clear, area);
		f.render_stateful_widget(extras_widget.get_widget(), area, extras_list.get_mut_state());
	}

	// Info popup
	if *current_popup == Some(Popup::Info) {
		let info_list = popups.get_mut(&Popup::Info).unwrap();
		let info_widget = InfoWidget::new(info_list);
		let area = centered_rect(60, 70, f.size());
		f.render_widget(Clear, area);
		f.render_stateful_widget(info_widget.get_widget(), area, info_list.get_mut_state());
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
