use crossterm::event::{KeyCode, KeyEvent};

use super::super::navigation::{advance_next, advance_previous, open_lesson_list};
use super::super::runner::{run_all_tests, run_current_test};
use super::super::state::{Screen, UiState};
use crate::lessons::LESSONS;

pub(super) fn handle_home(key: KeyEvent, state: &mut UiState) -> bool {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => return false,
        KeyCode::Char('p') | KeyCode::Char('P') => advance_previous(state),
        KeyCode::Char('n') | KeyCode::Char('N') => advance_next(state),
        KeyCode::Char('h') | KeyCode::Char('H') => state.show_hint = !state.show_hint,
        KeyCode::Char('l') | KeyCode::Char('L') => open_lesson_list(state),
        KeyCode::Char('c') | KeyCode::Char('C') => run_all_tests(state),
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
            run_current_test(state);
        }
        KeyCode::Char('g') | KeyCode::Char('G') => {
            state.message =
                "Book: start `cargo run -- --web`, then open http://127.0.0.1:7878/guide".into();
        }
        _ => {}
    }
    true
}

pub(super) fn handle_list(key: KeyEvent, state: &mut UiState) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('l') | KeyCode::Char('L') => {
            state.screen = Screen::Home;
            state.message.clear();
        }
        KeyCode::Char('q') | KeyCode::Char('Q') => return false,
        KeyCode::Up | KeyCode::Char('p') | KeyCode::Char('P') => {
            state.selected = state.selected.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('n') | KeyCode::Char('N') => {
            state.selected = (state.selected + 1).min(LESSONS.len() - 1);
        }
        KeyCode::Enter => {
            state.lesson_index = state.selected;
            run_current_test(state);
        }
        KeyCode::Char('h') | KeyCode::Char('H') => state.show_hint = !state.show_hint,
        _ => {}
    }
    true
}
