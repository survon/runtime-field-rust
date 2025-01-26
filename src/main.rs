use std::io;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

mod modules;
use modules::{WidgetModule, AppEvent};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Setup the terminal in raw mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2) Create an event bus for modules -> main
    let (tx, rx): (Sender<AppEvent>, Receiver<AppEvent>) = mpsc::channel();

    // 3) Initialize each submodule
    //    Two widgets capture input, two are display-only
    let mut widget_one = modules::widget_one::WidgetOne::new(tx.clone());
    let mut widget_two = modules::widget_two::WidgetTwo::new(tx.clone());
    let mut widget_three = modules::widget_three::WidgetThree::new();
    let mut widget_four = modules::widget_four::WidgetFour::new();

    // 4) Main loop
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        // (A) Poll for input or time out
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let CEvent::Key(key_event) = event::read()? {
                // Check global "quit" key (q / Ctrl-C)
                if is_quit_event(&key_event) {
                    break;
                }
                // Dispatch input to modules that care about it
                widget_one.handle_input(key_event);
                widget_two.handle_input(key_event);
            }
        }

        // (B) Process any events from modules
        while let Ok(app_event) = rx.try_recv() {
            match app_event {
                AppEvent::ModuleMessage(msg) => {
                    eprintln!("Main received: {}", msg);
                }
                AppEvent::KeyPress(k) => {
                    eprintln!("Main got keypress: {:?}", k);
                }
            }
        }

        // (C) Render all submodules
        terminal.draw(|frame| {
            // Split the screen in 2 rows, each with 2 columns => 4 total
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(frame.area());

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);

            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            widget_one.render(frame, top_chunks[0]);
            widget_two.render(frame, top_chunks[1]);
            widget_three.render(frame, bottom_chunks[0]);
            widget_four.render(frame, bottom_chunks[1]);
        })?;

        // (D) Throttle the loop so we don't spin at 100% CPU
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // 5) Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn is_quit_event(key: &KeyEvent) -> bool {
    use crossterm::event::{KeyCode, KeyModifiers};
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), _) => true,
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => true,
        _ => false,
    }
}
