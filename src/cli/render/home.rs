use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::Write;

use super::super::state::UiState;
use crate::lessons::LESSONS;
use crossterm::terminal;

pub(super) fn draw_home<W: Write>(stdout: &mut W, state: &UiState) {
    let lesson = state.lesson();
    let bar_width = terminal::size()
        .map(|(columns, _)| (columns as usize).saturating_sub(28).clamp(10, 94))
        .unwrap_or(52);
    let complete_count = state.completed.len().min(LESSONS.len());
    let filled = complete_count * bar_width / LESSONS.len();
    let marker_width = if complete_count < LESSONS.len() { 1 } else { 0 };
    let exercise_file = lesson.id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print("Progress: ["),
        SetForegroundColor(Color::Green),
        Print("#".repeat(filled)),
        SetForegroundColor(Color::Yellow),
        Print(if complete_count < LESSONS.len() {
            ">"
        } else {
            ""
        }),
        SetForegroundColor(Color::DarkGrey),
        Print("-".repeat(bar_width.saturating_sub(filled + marker_width))),
        SetForegroundColor(Color::White),
        Print(format!("] {complete_count}/{}\n", LESSONS.len())),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "Current exercise: exercises/src/{exercise_file}.rs\n\n"
        )),
        ResetColor,
        SetForegroundColor(Color::White),
        Print(format!("{}\n{}\n", lesson.title, lesson.prompt)),
        ResetColor
    );
    if state.show_hint {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("\nHINT\n{}\n", lesson.hint)),
            ResetColor
        );
    }
    let rows = terminal::size()
        .map(|(_, rows)| rows as usize)
        .unwrap_or(24);
    let output_lines = state.test_output.len().min(rows.saturating_sub(12));
    if !state.test_output.is_empty() {
        let _ = queue!(
            stdout,
            SetForegroundColor(if state.test_passed {
                Color::Green
            } else {
                Color::Yellow
            }),
            Print(if state.test_passed {
                "\n✓ Exercise check passed\n"
            } else {
                "\nExercise check output\n"
            }),
            ResetColor
        );
        for line in state
            .test_output
            .iter()
            .skip(state.test_output.len().saturating_sub(output_lines))
        {
            let _ = queue!(stdout, Print(format!("{line}\n")));
        }
        if output_lines < state.test_output.len() {
            let _ = queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("… output shortened to fit terminal\n"),
                ResetColor
            );
        }
    }
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(if state.completed.contains(lesson.id) {
            "\nh:hint / l:list / c:check all / x:reset / n:next / q:quit ?\n"
        } else {
            "\nh:hint / l:list / c:check all / x:reset / q:quit ?\n"
        }),
        ResetColor
    );
}
