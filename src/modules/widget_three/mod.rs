use crossterm::event::KeyEvent;
use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Gauge},
    style::{Style, Color},
};

use crate::modules::WidgetModule;

pub struct WidgetThree {
    progress: f64,
}

impl WidgetThree {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
        }
    }
}

impl WidgetModule for WidgetThree {
    fn handle_input(&mut self, _key: KeyEvent) {
        // No input handling
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Slowly increment
        self.progress += 0.02;
        if self.progress >= 1.0 {
            self.progress = 0.0;
        }

        let gauge = Gauge::default()
            .block(Block::default().title("WidgetThree").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(self.progress);

        frame.render_widget(gauge, area);
    }
}
