use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::progress::save_progress;
use super::runner::run_current_test;
use super::source::source_path;
use super::state::{Screen, UiState};

pub(super) fn reset_current_exercise(state: &mut UiState) {
    let relative = format!("exercises/src/{}.rs", state.lesson().id.replace('-', "_"));
    let source_path = source_path(state);
    let backup_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".gte-backup")
        .join(state.lesson().id.replace('-', "_") + ".rs");
    let original = Command::new("git")
        .args(["show", &format!("HEAD:{relative}")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();
    let original = match original {
        Ok(output) if output.status.success() => output.stdout,
        Ok(output) => {
            state.screen = Screen::Home;
            state.message = format!(
                "Could not restore the exercise from Git: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
            return;
        }
        Err(error) => {
            state.screen = Screen::Home;
            state.message = format!("Could not start Git to reset the exercise: {error}");
            return;
        }
    };
    if let Err(error) = fs::create_dir_all(backup_path.parent().expect("backup parent"))
        .and_then(|()| fs::copy(&source_path, &backup_path).map(|_| ()))
        .and_then(|()| fs::write(&source_path, original))
    {
        state.screen = Screen::Home;
        state.message =
            format!("Reset failed; the current file is backed up when possible: {error}");
        return;
    }
    state.completed.remove(state.lesson().id);
    state.started.remove(state.lesson().id);
    let _ = save_progress(&state.completed);
    state.screen = Screen::Home;
    state.show_hint = false;
    state.message = format!(
        "Reset {}. Previous contents saved in .gte-backup/.",
        state.lesson().title
    );
    run_current_test(state);
}
