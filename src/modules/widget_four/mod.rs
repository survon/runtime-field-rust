use crossterm::event::KeyEvent;
use rand::Rng;
use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Sparkline},
    style::{Style, Color},
};

use crate::modules::WidgetModule;

pub struct WidgetFour {
    data: Vec<u64>,
}

impl WidgetFour {
    pub fn new() -> Self {
        Self {
            data: vec![0; 30],
        }
    }
}

impl WidgetModule for WidgetFour {
    fn handle_input(&mut self, _key: KeyEvent) {
        // No input handling
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Shift data left, push a random value at the end
        let mut rng = rand::thread_rng();
        let next_val = rng.gen_range(0..100);

        // remove first element, push at end
        self.data.remove(0);
        self.data.push(next_val);

        let sparkline = Sparkline::default()
            .block(Block::default().title("WidgetFour").borders(Borders::ALL))
            .data(&self.data)
            .style(Style::default().fg(Color::Magenta));

        frame.render_widget(sparkline, area);
    }
}
