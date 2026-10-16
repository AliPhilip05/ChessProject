use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::Write;

use super::super::state::UiState;

pub(super) fn draw_test_result<W: Write>(stdout: &mut W, state: &UiState) {
    let (label, color) = if state.test_passed {
        ("TESTS PASSED", Color::Green)
    } else {
        ("TESTS NEED WORK", Color::Yellow)
    };
    let _ = queue!(
        stdout,
        SetForegroundColor(color),
        Print(format!("{label} · {}\n", state.lesson().title)),
        ResetColor
    );
    for line in state
        .test_output
        .iter()
        .skip(state.solution_offset)
        .take(20)
    {
        let _ = queue!(stdout, Print(format!("{line}\n")));
    }
    let _ = queue!(
        stdout,
        Print("\nR rerun · S solution · N/P next/previous · L lesson list · Esc return\n")
    );
}
