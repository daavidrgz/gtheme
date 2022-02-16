use tui::{
	widgets::{Block, Borders, List, ListItem, ListState},
	style::{Color, Modifier, Style},
};

use crate::app::ScreenItem;

pub struct StatefulList<T> {
	state: ListState,
	index: usize,
	items: Vec<T>,
}
impl<T> StatefulList<T> {
	pub fn with_items(items: Vec<T>, selected: bool) -> StatefulList<T> {
		let mut state = ListState::default();
		state.select(if selected {Some(0)} else {None});
		StatefulList {
			index: 0,
			state,
			items,
		}
	}

	pub fn get_state(&mut self) -> &mut ListState {
		&mut self.state
	}

	pub fn get_items(&mut self) -> &ListState {
		&self.state
	}

	pub fn is_selected(&mut self) -> bool {
		match self.state.selected() {
			Some(_) => true,
			None => false,
		}
	}

	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => if i >= self.items.len() - 1 {0} else {i + 1}
			None => self.index,
		};
		self.index = i;
		self.state.select(Some(i));
	}

	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => if i == 0 {self.items.len() - 1} else {i - 1}
			None => self.index,
		};
		self.index = i;
		self.state.select(Some(i));
	}

	pub fn unselect(&mut self) {
		self.state.select(None);
	}
}

pub struct ListWidget<'a> {
	widget: List<'a>
}
impl<'a> ListWidget<'a> {
	pub fn new(title: &str, color: Color, stateful_list: &StatefulList<ScreenItem>) -> ListWidget<'a> {
		let items: Vec<ListItem> = stateful_list
			.items
			.iter()
			.map(|i| {
				let name = match i {
					ScreenItem::Desktop(d) => d.get_name(),
					ScreenItem::Pattern(p) => p.get_name(),
					ScreenItem::Theme(t) => t.get_name(),
				};
				ListItem::new(String::from(name)).style(Style::default().fg(Color::DarkGray).bg(Color::Reset))
			}).collect();

		let widget = List::new(items)
			.block(Block::default()
				.borders(Borders::ALL)
				.title(format!(" {} ", title))
				.border_style(Style::default().fg(color)))
			.highlight_symbol(" ‣ ")
			.highlight_style(
				Style::default()
					.fg(color)
					.add_modifier(Modifier::BOLD),
			);
			
		ListWidget {
			widget
		}
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
 
