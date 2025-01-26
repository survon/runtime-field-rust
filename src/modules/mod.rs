use crossterm::event::KeyEvent;
use ratatui::{backend::Backend, Frame, layout::Rect};

/// Events that submodules can emit for the main loop
#[derive(Debug)]
pub enum AppEvent {
    ModuleMessage(String),
    KeyPress(KeyEvent),
}

/// Each submodule implements this trait:
/// - `handle_input` for keyboard events
/// - `render` for drawing in a specific terminal rectangle
pub trait WidgetModule {
    fn handle_input(&mut self, _key: KeyEvent) {}

    fn render(&mut self, frame: &mut Frame, area: Rect);
}

// Re-export the individual widgets
pub mod widget_one;
pub mod widget_two;
pub mod widget_three;
pub mod widget_four;
