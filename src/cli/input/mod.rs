mod home;
mod lesson;
mod results;

use crossterm::event::{KeyCode, KeyEvent};

use super::state::{Screen, UiState};

pub(super) fn handle_key(key: KeyEvent, state: &mut UiState) -> bool {
    if matches!(key.code, KeyCode::Char('x') | KeyCode::Char('X'))
        && state.screen != Screen::ResetConfirm
    {
        if state.screen == Screen::List {
            state.lesson_index = state.selected;
        }
        state.screen = Screen::ResetConfirm;
        state.message.clear();
        return true;
    }
    if key.code == KeyCode::Char('?') {
        state.message = match state.screen {
            Screen::Home => "h:hint / l:list / c:check all / r:run / x:reset / n:next / p:previous / q:quit ?".into(),
            Screen::List => "n/p or ↑/↓ select / Enter: run exercise / l:back / q:quit".into(),
            Screen::Lesson => {
                "N/P next/previous · R run · H hint · C check · S solution · G Guidebook · L list · Q quit".into()
            }
            Screen::Check => "↑/↓ or 1–3 select · Enter check · R run · N/P next/previous · Esc/L lesson list · Q quit".into(),
            Screen::Solution => "↑/↓ or PgUp/PgDn scroll · N/P next/previous · Esc lesson · L list · Q quit".into(),
            Screen::TestResult => "R rerun · S solution · N/P next/previous · Esc lesson · L list · Q quit".into(),
            Screen::ResetConfirm => "y:reset current exercise / n:cancel".into(),
        };
        return true;
    }
    match state.screen {
        Screen::Home => home::handle_home(key, state),
        Screen::List => home::handle_list(key, state),
        Screen::Lesson => lesson::handle_lesson(key, state),
        Screen::Check => lesson::handle_check(key, state),
        Screen::Solution => results::handle_solution(key, state),
        Screen::TestResult => results::handle_test_result(key, state),
        Screen::ResetConfirm => results::handle_reset_confirm(key, state),
    }
}

#[cfg(test)]
mod tests {
    use super::{Screen, UiState, handle_key};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use std::collections::HashSet;

    fn state(screen: Screen) -> UiState {
        UiState {
            screen,
            selected: 0,
            lesson_index: 0,
            check_selected: 0,
            show_hint: false,
            started: HashSet::new(),
            completed: HashSet::new(),
            message: String::new(),
            solution: Vec::new(),
            solution_offset: 0,
            test_output: Vec::new(),
            test_passed: false,
            last_modified: None,
        }
    }

    fn press(character: char, state: &mut UiState) -> bool {
        handle_key(
            KeyEvent::new(KeyCode::Char(character), KeyModifiers::NONE),
            state,
        )
    }

    #[test]
    fn quit_works_from_every_screen() {
        for screen in [
            Screen::Home,
            Screen::List,
            Screen::Lesson,
            Screen::Check,
            Screen::Solution,
            Screen::TestResult,
            Screen::ResetConfirm,
        ] {
            assert!(!press('q', &mut state(screen)));
            assert!(!press('Q', &mut state(screen)));
        }
    }

    #[test]
    fn reset_uses_the_selected_list_exercise_and_cancel_does_not_advance() {
        let mut state = state(Screen::List);
        state.selected = 2;
        assert!(press('x', &mut state));
        assert!(state.screen == Screen::ResetConfirm);
        assert_eq!(state.lesson_index, 2);

        assert!(press('n', &mut state));
        assert!(state.screen == Screen::Home);
        assert_eq!(state.lesson_index, 2);
        assert!(state.completed.is_empty());
    }

    #[test]
    fn solution_scrolling_stays_within_its_content() {
        let mut state = state(Screen::Solution);
        state.solution = vec!["first".into(), "last".into()];
        let down = KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE);
        assert!(handle_key(down, &mut state));
        assert_eq!(state.solution_offset, 1);

        let up = KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE);
        assert!(handle_key(up, &mut state));
        assert_eq!(state.solution_offset, 0);
    }
}
