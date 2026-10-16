use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use super::state::UiState;

pub(super) fn source_path(state: &UiState) -> PathBuf {
    let file_name = state.lesson().id.replace('-', "_");
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("exercises/src")
        .join(format!("{file_name}.rs"))
}

pub(super) fn source_modified(state: &UiState) -> Option<SystemTime> {
    fs::metadata(source_path(state)).ok()?.modified().ok()
}
