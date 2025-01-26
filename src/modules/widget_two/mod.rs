use std::sync::mpsc::Sender;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem},
    style::{Style, Color},
};

use crate::modules::{WidgetModule, AppEvent};

pub struct WidgetTwo {
    items: Vec<String>,
    index: usize,
    tx: Sender<AppEvent>,
}

impl WidgetTwo {
    pub fn new(tx: Sender<AppEvent>) -> Self {
        Self {
            items: vec![
                "apple".to_string(),
                "banana".to_string(),
                "grapes".to_string(),
                "orange".to_string(),
            ],
            index: 0,
            tx,
        }
    }
}

impl WidgetModule for WidgetTwo {
    fn handle_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => {
                if self.index > 0 {
                    self.index -= 1;
                    let _ = self.tx.send(AppEvent::ModuleMessage(
                        format!("[WidgetTwo] selected: {}", self.items[self.index])
                    ));
                }
            }
            KeyCode::Right => {
                if self.index < self.items.len() - 1 {
                    self.index += 1;
                    let _ = self.tx.send(AppEvent::ModuleMessage(
                        format!("[WidgetTwo] selected: {}", self.items[self.index])
                    ));
                }
            }
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let style = if i == self.index {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };
                ListItem::new(val.clone()).style(style)
            })
            .collect();

        let list = List::new(list_items)
            .block(Block::default().title("WidgetTwo").borders(Borders::ALL));

        frame.render_widget(list, area);
    }
}
