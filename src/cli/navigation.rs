use crate::lessons::LESSONS;

use super::runner::run_current_test;
use super::state::{Screen, UiState};

pub(super) fn open_lesson_list(state: &mut UiState) {
    state.selected = state.lesson_index;
    state.screen = Screen::List;
    state.message.clear();
}

pub(super) fn advance_next(state: &mut UiState) {
    if !state.completed.contains(state.lesson().id) {
        state.message =
            "This exercise is not complete yet. Edit it, then press R to run it.".into();
        return;
    }
    let next = (state.lesson_index + 1..LESSONS.len())
        .chain(0..state.lesson_index)
        .find(|index| !state.completed.contains(LESSONS[*index].id));
    match next {
        Some(index) => {
            state.lesson_index = index;
            state.selected = index;
            state.show_hint = false;
            state.message.clear();
            run_current_test(state);
        }
        None => {
            state.message = "All exercises are complete. Use L to review the exercise list.".into();
        }
    }
}

pub(super) fn advance_previous(state: &mut UiState) {
    if state.lesson_index == 0 {
        state.message = "This is the first exercise.".into();
        return;
    }
    state.lesson_index -= 1;
    state.selected = state.lesson_index;
    state.show_hint = false;
    state.message.clear();
    run_current_test(state);
}
