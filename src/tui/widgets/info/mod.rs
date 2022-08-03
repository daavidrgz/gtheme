use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::tui::screenitem::ScreenItem;
use crate::tui::statefullist::StatefulList;

pub struct InfoWidget<'a> {
    widget: List<'a>,
}

impl<'a> InfoWidget<'a> {
    pub fn new(stateful_list: &StatefulList<ScreenItem>) -> InfoWidget<'a> {
        let items = Self::create_info(stateful_list);

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

        InfoWidget { widget: list }
    }

    fn create_info(stateful_list: &StatefulList<ScreenItem>) -> Vec<ListItem<'a>> {
        let entry_key_style = Style::default()
            .fg(Color::Green)
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
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

            let list_item = if line.starts_with("-") {
                let entry_key = Span::styled(format!(" {}", line.to_string()), entry_value_style);
                ListItem::new(Spans::from(vec![bar_span, entry_key]))
            } else {
                let words: Vec<&str> = line.splitn(2, ':').collect();
                let key = words.get(0).unwrap_or(&"").clone();
                let value = words.get(1).unwrap_or(&"").clone();
                let entry_key = Span::styled(format!("{}:", key.to_string()), entry_key_style);
                let entry_value = Span::styled(value.to_string(), entry_value_style);
                ListItem::new(Spans::from(vec![bar_span, entry_key, entry_value]))
            };
            items.push(list_item);
        }
        items
    }

    pub fn get_widget(self) -> List<'a> {
        self.widget
    }
}
