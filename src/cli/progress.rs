use crate::lessons::LESSONS;
use std::collections::HashSet;
use std::{fs, io};

fn progress_path() -> String {
    format!("{}/.gte-state.txt", env!("CARGO_MANIFEST_DIR"))
}

pub(super) fn load_progress() -> HashSet<&'static str> {
    let Ok(saved) = fs::read_to_string(progress_path()) else {
        return HashSet::new();
    };
    LESSONS
        .iter()
        .filter(|lesson| saved.lines().any(|line| line == lesson.id))
        .map(|lesson| lesson.id)
        .collect()
}

pub(super) fn save_progress(completed: &HashSet<&'static str>) -> io::Result<()> {
    let saved = LESSONS
        .iter()
        .filter(|lesson| completed.contains(lesson.id))
        .map(|lesson| lesson.id)
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(progress_path(), saved)
}
