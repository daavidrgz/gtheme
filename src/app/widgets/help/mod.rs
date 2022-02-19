use tui::{
	widgets::{Block, Borders, List, ListItem},
	style::{Color, Modifier, Style},
	text::{Span, Spans},
};

use crate::app::statefullist::StatefulList;
use crate::app::screenitem::ScreenItem;

pub struct HelpWidget<'a> {
	widget: List<'a>
}
impl<'a> HelpWidget<'a> {
	pub fn new(stateful_list: &StatefulList<ScreenItem>) -> HelpWidget<'a> {
		let items = Self::create_help(stateful_list);

		let title_style = Style::default().fg(*stateful_list.get_color()).add_modifier(Modifier::BOLD).add_modifier(Modifier::REVERSED);
		let block = Block::default()
			.title(Span::styled(format!(" {} ", stateful_list.get_title()), title_style))
			.borders(Borders::ALL)
			.border_style(Style::default().fg(*stateful_list.get_color()));

		let list = List::new(items)
			.block(block);

		HelpWidget {
			widget: list
		}
	}

	fn create_help(stateful_list: &StatefulList<ScreenItem>) -> Vec<ListItem<'a>> {
		let title_style = Style::default().fg(Color::Blue)
			.add_modifier(Modifier::BOLD)
			.add_modifier(Modifier::ITALIC);
		let entry_key_style = Style::default().fg(Color::Yellow)
			.add_modifier(Modifier::BOLD);
		let entry_value_style = Style::default().add_modifier(Modifier::BOLD);

		let items: Vec<ListItem> = stateful_list.get_items().iter().enumerate().map(|(it, item)| {
			let line = item.get_name();
			
			let bar = match stateful_list.get_state().selected() {
				Some(idx) => if idx == it {" │ "} else {"   "},
				None => "   "
			};
			let bar_span = Span::styled(bar, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

			let words: Vec<&str> = line.split('#').collect();
			
			if words.len() > 2 {
				panic!("Format error in help file")
			}

			// Its a section title
			if words.len() == 1 {
				return ListItem::new(Spans::from(vec![bar_span, Span::styled(String::from(words[0]), title_style)]));
			}

			// Its a section entry
			let entry_key = Span::styled(String::from(words[0]), entry_key_style);
			let entry_value = Span::styled(String::from(words[1]), entry_value_style);
			ListItem::new(Spans::from(vec![bar_span, entry_key, entry_value]))
		}).collect();
		items
	}

	pub fn get_widget(self) -> List<'a> {
		self.widget
	}
}
