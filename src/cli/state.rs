use crate::lessons::{LESSONS, Lesson};
use std::collections::HashSet;
use std::time::SystemTime;

use super::progress::load_progress;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Screen {
    Home,
    List,
    Lesson,
    Check,
    Solution,
    TestResult,
    ResetConfirm,
}

pub(super) struct UiState {
    pub(super) screen: Screen,
    pub(super) selected: usize,
    pub(super) lesson_index: usize,
    pub(super) check_selected: usize,
    pub(super) show_hint: bool,
    pub(super) started: HashSet<&'static str>,
    pub(super) completed: HashSet<&'static str>,
    pub(super) message: String,
    pub(super) solution: Vec<String>,
    pub(super) solution_offset: usize,
    pub(super) test_output: Vec<String>,
    pub(super) test_passed: bool,
    pub(super) last_modified: Option<SystemTime>,
}

impl UiState {
    pub(super) fn new() -> Self {
        let completed = load_progress();
        let lesson_index = LESSONS
            .iter()
            .position(|lesson| !completed.contains(lesson.id))
            .unwrap_or(0);
        Self {
            screen: Screen::Home,
            selected: lesson_index,
            lesson_index,
            check_selected: 0,
            show_hint: false,
            started: completed.clone(),
            completed,
            message: String::new(),
            solution: Vec::new(),
            solution_offset: 0,
            test_output: Vec::new(),
            test_passed: false,
            last_modified: None,
        }
    }

    pub(super) fn lesson(&self) -> &'static Lesson {
        &LESSONS[self.lesson_index]
    }
}
