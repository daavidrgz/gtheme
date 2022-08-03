mod help_content;
pub use help_content::HELP_CONTENT;

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::tui::screenitem::ScreenItem;
use crate::tui::statefullist::StatefulList;

pub struct HelpWidget<'a> {
    widget: List<'a>,
}
impl<'a> HelpWidget<'a> {
    pub fn new(stateful_list: &StatefulList<ScreenItem>) -> HelpWidget<'a> {
        let items = Self::create_help(stateful_list);

        let title_style = Style::default()
            .fg(*stateful_list.get_color())
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::REVERSED);
        let block = Block::default()
            .title(Span::styled(
                format!(" {} ", stateful_list.get_title()),
                title_style,
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(*stateful_list.get_color()));

        let list = List::new(items).block(block);

        HelpWidget { widget: list }
    }

    fn create_help(stateful_list: &StatefulList<ScreenItem>) -> Vec<ListItem<'a>> {
        let title_style = Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC);
        let entry_key_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let entry_value_style = Style::default().add_modifier(Modifier::BOLD);

        let mut items: Vec<ListItem> = vec![];
        for (it, item) in stateful_list.get_items().iter().enumerate() {
            let line = item.get_name();

            let bar = match stateful_list.get_state().selected() {
                Some(idx) => {
                    if idx == it {
                        " ┃ "
                    } else {
                        "   "
                    }
                }
                None => "   ",
            };
            let bar_span = Span::styled(
                bar,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

            let words: Vec<&str> = line.split('#').collect();

            // Its a section title
            if words.len() == 1 {
                items.push(ListItem::new(Spans::from(vec![
                    bar_span,
                    Span::styled(String::from(words[0]), title_style),
                ])));
                continue;
            }

            // Its a section entry
            let entry_key = Span::styled(String::from(words[0]), entry_key_style);
            let entry_value = Span::styled(String::from(words[1]), entry_value_style);
            let list_item = ListItem::new(Spans::from(vec![bar_span, entry_key, entry_value]));
            items.push(list_item)
        }
        items
    }

    pub fn get_widget(self) -> List<'a> {
        self.widget
    }
}
