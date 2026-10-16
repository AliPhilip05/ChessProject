use crossterm::event::{KeyCode, KeyEvent};
use std::fs;

use super::super::navigation::{advance_next, advance_previous, open_lesson_list};
use super::super::runner::run_current_test;
use super::super::state::{Screen, UiState};

pub(super) fn handle_lesson(key: KeyEvent, state: &mut UiState) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Home,
        KeyCode::Char('l') | KeyCode::Char('L') => open_lesson_list(state),
        KeyCode::Char('q') | KeyCode::Char('Q') => return false,
        KeyCode::Char('h') | KeyCode::Char('H') => state.show_hint = !state.show_hint,
        KeyCode::Char('n') | KeyCode::Char('N') => {
            advance_next(state);
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            advance_previous(state);
        }
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
            run_current_test(state);
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            state.screen = Screen::Check;
            state.check_selected = 0;
            state.message = "Select an answer and press Enter to check your understanding.".into();
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            let path = format!(
                "{}/{}",
                env!("CARGO_MANIFEST_DIR"),
                state.lesson().solution_file
            );
            let source = fs::read_to_string(path)
                .unwrap_or_else(|error| format!("Could not open reference solution: {error}"));
            state.solution = source.lines().map(str::to_owned).collect();
            state.solution_offset = 0;
            state.screen = Screen::Solution;
        }
        KeyCode::Char('g') | KeyCode::Char('G') => {
            state.message = format!(
                "Exercise reference: start `cargo run -- --web`, then open http://127.0.0.1:7878/guide/reference#{}",
                state.lesson().reference_anchor
            );
        }
        _ => {}
    }
    true
}

pub(super) fn handle_check(key: KeyEvent, state: &mut UiState) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Lesson,
        KeyCode::Char('l') | KeyCode::Char('L') => {
            open_lesson_list(state);
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            advance_next(state);
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            advance_previous(state);
        }
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
            run_current_test(state)
        }
        KeyCode::Up => state.check_selected = state.check_selected.saturating_sub(1),
        KeyCode::Down => state.check_selected = (state.check_selected + 1).min(2),
        KeyCode::Char('1') => state.check_selected = 0,
        KeyCode::Char('2') => state.check_selected = 1,
        KeyCode::Char('3') => state.check_selected = 2,
        KeyCode::Enter => {
            if state.check_selected == state.lesson().answer {
                state.message =
                    "Concept check passed. Press R to run the Rust exercise check.".into();
                state.screen = Screen::Lesson;
            } else {
                state.message = "Not quite. Revisit the prompt or hint, then try again.".into();
            }
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => return false,
        _ => {}
    }
    true
}
