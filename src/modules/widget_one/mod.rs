use std::sync::mpsc::Sender;

use crossterm::event::{KeyEvent, KeyCode};
use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    text::{Line, Span},
};

use crate::modules::{WidgetModule, AppEvent};

pub struct WidgetOne {
    count: i32,
    tx: Sender<AppEvent>,
}

impl WidgetOne {
    pub fn new(tx: Sender<AppEvent>) -> Self {
        Self {
            count: 0,
            tx,
        }
    }
}

impl WidgetModule for WidgetOne {
    fn handle_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                self.count += 1;
                let _ = self.tx.send(AppEvent::ModuleMessage(
                    format!("[WidgetOne] Count incremented => {}", self.count)
                ));
            }
            KeyCode::Down => {
                self.count -= 1;
                let _ = self.tx.send(AppEvent::ModuleMessage(
                    format!("[WidgetOne] Count decremented => {}", self.count)
                ));
            }
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        // For multi-line text, we can build a Vec<Line>
        let lines = vec![
            Line::from("WidgetOne: Press Up/Down arrows."),
            Line::from(Span::styled(
                format!("Count: {}", self.count),
                Style::default().fg(Color::Yellow),
            )),
        ];

        let paragraph = Paragraph::new(lines)
            .block(Block::default().title("WidgetOne").borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }
}
