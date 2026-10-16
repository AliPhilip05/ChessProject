use crossterm::event::{KeyCode, KeyEvent};
use std::fs;

use super::super::navigation::{advance_next, advance_previous, open_lesson_list};
use super::super::reset::reset_current_exercise;
use super::super::runner::run_current_test;
use super::super::state::{Screen, UiState};

pub(super) fn handle_solution(key: KeyEvent, state: &mut UiState) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Lesson,
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
            run_current_test(state)
        }
        KeyCode::Char('l') | KeyCode::Char('L') => {
            open_lesson_list(state);
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            advance_next(state);
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            advance_previous(state);
        }
        KeyCode::Up => state.solution_offset = state.solution_offset.saturating_sub(1),
        KeyCode::Down => {
            state.solution_offset =
                (state.solution_offset + 1).min(state.solution.len().saturating_sub(1))
        }
        KeyCode::PageUp => state.solution_offset = state.solution_offset.saturating_sub(12),
        KeyCode::PageDown => {
            state.solution_offset =
                (state.solution_offset + 12).min(state.solution.len().saturating_sub(1))
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => return false,
        _ => {}
    }
    true
}

pub(super) fn handle_test_result(key: KeyEvent, state: &mut UiState) -> bool {
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
        KeyCode::Up => state.solution_offset = state.solution_offset.saturating_sub(1),
        KeyCode::Down => {
            state.solution_offset =
                (state.solution_offset + 1).min(state.test_output.len().saturating_sub(1))
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => return false,
        _ => {}
    }
    true
}

pub(super) fn handle_reset_confirm(key: KeyEvent, state: &mut UiState) -> bool {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => reset_current_exercise(state),
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Backspace => {
            state.screen = Screen::Home;
            state.message = "Reset cancelled.".into();
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => return false,
        _ => {}
    }
    true
}
