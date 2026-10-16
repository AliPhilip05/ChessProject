use crate::lessons::{LESSONS, Lesson};
use std::fs;
use std::process::Command;

use super::progress::save_progress;
use super::source::source_modified;
use super::state::{Screen, UiState};

pub(super) fn run_current_test(state: &mut UiState) {
    let lesson = state.lesson();
    state.started.insert(lesson.id);
    let (passed, text) = run_lesson_test(lesson);
    state.last_modified = source_modified(state);
    state.test_passed = passed;
    state.test_output = text.lines().map(str::to_owned).collect();
    state.solution_offset = 0;
    if passed {
        state.completed.insert(lesson.id);
        let _ = save_progress(&state.completed);
        let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), lesson.solution_file);
        match fs::read_to_string(path) {
            Ok(source) => {
                state.solution = source.lines().map(str::to_owned).collect();
                state.screen = Screen::Solution;
                state.message =
                    "Exercise check passed. The matching reference solution loaded automatically."
                        .into();
            }
            Err(error) => {
                state.screen = Screen::TestResult;
                state.message = format!(
                    "Exercise check passed, but the reference solution could not load: {error}"
                );
            }
        }
    } else {
        state.completed.remove(lesson.id);
        let _ = save_progress(&state.completed);
        state.screen = Screen::Home;
        state.message = "Edit the exercise code, check the hint, and run the check again.".into();
    }
}

pub(super) fn run_all_tests(state: &mut UiState) {
    let mut summary = Vec::new();
    let mut passed_count = 0;
    for lesson in &LESSONS {
        state.started.insert(lesson.id);
        let (passed, output) = run_lesson_test(lesson);
        if passed {
            state.completed.insert(lesson.id);
            passed_count += 1;
            summary.push(format!("✓ {}", lesson.title));
        } else {
            state.completed.remove(lesson.id);
            summary.push(format!("✗ {}", lesson.title));
            summary.extend(
                output
                    .lines()
                    .rev()
                    .take(5)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|line| format!("  {line}")),
            );
        }
    }
    let _ = save_progress(&state.completed);
    state.test_passed = passed_count == LESSONS.len();
    state.test_output = summary;
    state.solution_offset = 0;
    state.screen = Screen::TestResult;
    state.message = format!(
        "Check all finished: {passed_count}/{} exercises passed.",
        LESSONS.len()
    );
}

fn run_lesson_test(lesson: &Lesson) -> (bool, String) {
    let manifest = format!("{}/exercises/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
    let target = lesson.id.replace('-', "_");
    let output = Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--test")
        .arg(&target)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();
    match output {
        Ok(output) => {
            let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            (output.status.success(), text)
        }
        Err(error) => (false, format!("Could not start Cargo: {error}")),
    }
}
