use tui::{
	widgets::{Block, Borders, List, ListItem, ListState},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
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

	pub fn get_state_mut(&mut self) -> &mut ListState {
		&mut self.state
	}

	pub fn get_state(&self) -> &ListState {
		&self.state
	}

	pub fn get_items(&mut self) -> &ListState {
		&self.state
	}

	pub fn is_selected(&self) -> bool {
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

		let mut it = 0;

		let items: Vec<ListItem> = stateful_list
			.items.iter()
			.map(|screen_item| {

				let mut text = screen_item.get_name().to_string();
				text = match stateful_list.get_state().selected() {
					Some(idx) => if idx == it {format!(" ‣ {}", text)} else {format!("   {}", text)},
					None => format!("   {}", text)
				};

				it+=1;

				ListItem::new(String::from(text)).style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM))
			}).collect();
		
		let title_style = Style::default().fg(color).add_modifier(Modifier::BOLD);
		let title_style = if stateful_list.is_selected() {
			title_style.add_modifier(Modifier::REVERSED)
		}	else {
			title_style
		};
		
		let widget = List::new(items)
			.block(Block::default()
				.borders(Borders::ALL)
				.title(Span::styled(String::from(format!(" {} ", title)), title_style))
				.border_style(Style::default().fg(color)))
			.highlight_symbol("")
			.highlight_style(Style::default().fg(color).add_modifier(Modifier::BOLD).remove_modifier(Modifier::DIM),
			);
			
		ListWidget { widget }
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
 
