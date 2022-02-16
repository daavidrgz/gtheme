use tui::{
	widgets::{Block, Borders, List, ListItem, ListState},
	style::{Color, Modifier, Style},
	text::Span,
};

use crate::app::ScreenItem;

pub struct StatefulList<T> {
	state: ListState,
	index: usize,
	items: Vec<T>,
	color: Color,
	title: String
}
impl<T> StatefulList<T> {
	pub fn with_items(items: Vec<T>, color: Color, title: String, selected: bool) -> StatefulList<T> {
		let mut state = ListState::default();
		state.select(if selected {Some(0)} else {None});
		StatefulList {
			index: 0,
			color,
			title,
			state,
			items,
		}
	}

	pub fn get_state(&self) -> &ListState {
		&self.state
	}
	pub fn get_state_mut(&mut self) -> &mut ListState {
		&mut self.state
	}
	pub fn get_items(&self) -> &Vec<T> {
		&self.items
	}
	pub fn get_color(&self) -> &Color {
		&self.color
	}
	pub fn get_title(&self) -> &String {
		&self.title
	}
	pub fn get_length(&self) -> usize {
		self.items.len()
	}
	
	pub fn get_selected(&self) -> Option<&T> {
		match self.state.selected() {
			Some(idx) => Some(&self.items[idx]),
			None => None,
		}
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
	pub fn new(stateful_list: &StatefulList<ScreenItem>) -> ListWidget<'a> {

		let color = *stateful_list.get_color();
		let title = stateful_list.get_title();
		let mut it = 0;

		let items: Vec<ListItem> = stateful_list
			.items.iter()
			.map(|screen_item| {

				let mut text = screen_item.get_name().to_string();
				text = match stateful_list.get_state().selected() {
					Some(idx) => {
						if idx == it {
							let aux_text = format!(" ‣ {}", text);
							if idx == 0 { format!("{} ↓", aux_text) }
							else if idx == stateful_list.get_length() - 1 { format!("{} ↑", aux_text) }
							else { format!("{} ↓ ↑", aux_text) }
							
						} else {
							format!("   {}", text)
						}
					},
					None => format!("   {}", text)
				};

				it+=1;

				ListItem::new(String::from(text)).style(Style::default().fg(Color::Gray).add_modifier(Modifier::DIM))
			}).collect();
		
		let mut title_style = Style::default().fg(color).add_modifier(Modifier::BOLD).add_modifier(Modifier::REVERSED);

		let mut border_style = Style::default().fg(color);
		border_style = if !stateful_list.is_selected() {
			border_style.add_modifier(Modifier::DIM)
		}	else {
			border_style
		};

		let widget = List::new(items)
			.block(Block::default()
				.borders(Borders::ALL)
				.title(Span::styled(String::from(format!(" {} ", title)), title_style))
				.border_style(border_style))
			.highlight_symbol("")
			.highlight_style(Style::default().fg(color).add_modifier(Modifier::BOLD).remove_modifier(Modifier::DIM),
			);
			
		ListWidget { widget }
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
 
