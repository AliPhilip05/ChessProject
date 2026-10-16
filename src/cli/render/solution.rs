use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::Write;

use super::super::state::UiState;
use crate::lessons::LESSONS;

pub(super) fn draw_solution<W: Write>(stdout: &mut W, state: &UiState) {
    let complete_count = state.completed.len().min(LESSONS.len());
    let exercise_file = state.lesson().id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("REFERENCE · {}\n", state.lesson().title)),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "Progress: {complete_count}/{} · exercises/src/{exercise_file}.rs\n\n",
            LESSONS.len()
        )),
        ResetColor
    );
    if state.completed.contains(state.lesson().id) {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("✓ Exercise tests passed · matching solution loaded\n"),
            ResetColor
        );
        let test_lines = state
            .test_output
            .iter()
            .filter(|line| {
                line.starts_with("running ")
                    || line.starts_with("test ")
                    || line.starts_with("test result:")
            })
            .take(7);
        for line in test_lines {
            let _ = queue!(stdout, Print(format!("  {line}\n")));
        }
        let _ = queue!(stdout, Print("\n"));
    }
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("MATCHING REFERENCE SOLUTION\n"),
        ResetColor
    );
    for line in state.solution.iter().skip(state.solution_offset).take(20) {
        let _ = queue!(stdout, Print(format!("{line}\n")));
    }
    let _ = queue!(
        stdout,
        Print("\n↑/↓ scroll · PgUp/PgDn page · N/P next/previous · L lesson list · Esc return\n")
    );
}
