use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use chrono::{DateTime, Utc};
use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};
use serde::{Deserialize, Serialize};
use tui::{backend::CrosstermBackend, Terminal};

const DB_PATH: &str = "sandbox.json";

#[derive(Serialize, Deserialize, Clone)]
struct Pet {
    id: usize,
    name: String,
    category: String,
    age: usize,
    created_at: DateTime<Utc>,
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Pets,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Pets => 1,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("Failed to enable raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("Failed to poll works") {
                if let CEvent::Key(key) = event::read().expect("Failed to read events") {
                    tx.send(Event::Input(key)).expect("Failed to send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(())
}
