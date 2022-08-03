use crate::core::config::GlobalConfig;
use crate::tui::screenitem::ScreenItem;
use tui::{style::Color, widgets::ListState};

pub struct StatefulList<T> {
    state: ListState,
    index: usize,
    infinite: bool,
    items: Vec<T>,
    color: Color,
    title: String,
    active_text: String,
    inactive_text: String,
    active_text_color: Option<Color>,
    inactive_text_color: Option<Color>,
    alignment: bool,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            items,
            state: ListState::default(),
            index: 0,

            infinite: false,
            color: Color::White,
            title: "LIST".to_string(),
            active_text: "• Active ".to_string(),
            active_text_color: None,
            inactive_text: "".to_string(),
            inactive_text_color: None,
            alignment: false,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.state.select(if selected { Some(0) } else { None });
        self
    }
    pub fn infinite(mut self, infinite: bool) -> Self {
        self.infinite = infinite;
        self
    }
    pub fn active_text(mut self, active_text: &str) -> Self {
        self.active_text = active_text.to_string();
        self
    }
    pub fn active_text_color(mut self, active_text_color: Color) -> Self {
        self.active_text_color = Some(active_text_color);
        self
    }
    pub fn inactive_text(mut self, inactive_text: &str) -> Self {
        self.inactive_text = inactive_text.to_string();
        self
    }
    pub fn inactive_text_color(mut self, inactive_text_color: Color) -> Self {
        self.inactive_text_color = Some(inactive_text_color);
        self
    }
    pub fn alignment(mut self, alignment: bool) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn get_state(&self) -> &ListState {
        &self.state
    }
    pub fn get_mut_state(&mut self) -> &mut ListState {
        &mut self.state
    }
    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }
    pub fn get_mut_items(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
    pub fn get_color(&self) -> &Color {
        &self.color
    }
    pub fn get_title(&self) -> &String {
        &self.title
    }
    pub fn get_active_text(&self) -> &String {
        &self.active_text
    }
    pub fn get_active_text_color(&self) -> &Option<Color> {
        &self.active_text_color
    }
    pub fn get_inactive_text(&self) -> &String {
        &self.inactive_text
    }
    pub fn get_inactive_text_color(&self) -> &Option<Color> {
        &self.inactive_text_color
    }
    pub fn get_infinite(&self) -> &bool {
        &self.infinite
    }
    pub fn get_alignment(&self) -> &bool {
        &self.alignment
    }
    pub fn get_length(&self) -> usize {
        self.items.len()
    }

    pub fn get_selected(&self) -> Option<&T> {
        match self.state.selected() {
            Some(idx) => self.items.get(idx),
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
            Some(i) => {
                if i + 1 >= self.items.len() {
                    if !self.get_infinite() {
                        i
                    } else {
                        0
                    }
                } else {
                    i + 1
                }
            }
            None => self.index,
        };
        self.index = i;
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    if !self.get_infinite() {
                        i
                    } else {
                        self.items.len() - 1
                    }
                } else {
                    i - 1
                }
            }
            None => self.index,
        };
        self.index = i;
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl StatefulList<ScreenItem> {
    pub fn add_fav(&mut self, item: &ScreenItem, global_config: &mut GlobalConfig) {
        match item {
            ScreenItem::Theme(t) => {
                global_config.add_fav_theme(t);
                global_config.save();

                let theme_name = t.get_name().to_lowercase();
                match self
                    .items
                    .binary_search_by(|item| item.get_name().to_lowercase().cmp(&theme_name))
                {
                    Ok(_) => (),
                    Err(pos) => self.items.insert(pos, item.clone()),
                }
            }
            _ => (),
        }
    }

    pub fn remove_fav(&mut self, item: &ScreenItem, global_config: &mut GlobalConfig) {
        match item {
            ScreenItem::Theme(t) => {
                global_config.remove_fav_theme(t);
                global_config.save();

                let theme_name = t.get_name().to_lowercase();
                match self
                    .items
                    .binary_search_by(|item| item.get_name().to_lowercase().cmp(&theme_name))
                {
                    Ok(pos) => {
                        self.items.remove(pos);
                    }
                    Err(_) => (),
                }
            }
            _ => (),
        }
    }
}
