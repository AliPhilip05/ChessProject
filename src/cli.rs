//! Interactive exercise terminal, composed from input, state, runner, and rendering modules.
mod input;
mod navigation;
mod progress;
mod render;
mod reset;
mod runner;
mod source;
mod state;
mod writer;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use std::time::Duration;

use input::handle_key;
use render::{draw, plain_fallback};
use runner::run_current_test;
use source::source_modified;
use state::{Screen, UiState};
use writer::CrLfWriter;

pub fn run() {
    if let Err(error) = terminal::enable_raw_mode() {
        plain_fallback(&format!("Raw mode is unavailable ({error})."));
        return;
    }
    let mut stdout = CrLfWriter::new(io::stdout());
    if execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All)).is_err() {
        let _ = terminal::disable_raw_mode();
        plain_fallback("The terminal could not enter the interactive screen.");
        return;
    }

    let mut state = UiState::new();
    run_current_test(&mut state);
    let mut running = true;
    while running {
        draw(&mut stdout, &state);
        match event::poll(Duration::from_millis(200)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => running = handle_key(key, &mut state),
                Ok(_) => {}
                Err(_) => running = false,
            },
            Ok(false) => {
                if matches!(state.screen, Screen::Home | Screen::Solution)
                    && source_modified(&state).is_some_and(|modified| {
                        state.last_modified.is_some_and(|last| modified > last)
                    })
                {
                    run_current_test(&mut state);
                }
            }
            Err(_) => running = false,
        }
    }

    let _ = execute!(stdout, Show, LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();
    println!(
        "We hope you're enjoying learning Rust! If you want to continue working on the exercises at a later point, you can simply run `gte` again in this directory."
    );
}
