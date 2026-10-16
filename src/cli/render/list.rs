use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::Write;

use super::super::state::UiState;
use crate::lessons::LESSONS;

pub(super) fn draw_list<W: Write>(stdout: &mut W, state: &UiState) {
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print("EXERCISE LIST\n"),
        ResetColor
    );
    for (index, lesson) in LESSONS.iter().enumerate() {
        let pointer = if index == state.selected { "›" } else { " " };
        let status = if state.completed.contains(lesson.id) {
            "✓"
        } else if state.started.contains(lesson.id) {
            "◉"
        } else {
            "○"
        };
        if index == state.selected {
            let _ = queue!(stdout, SetForegroundColor(Color::Green));
        }
        let _ = queue!(
            stdout,
            Print(format!(
                " {pointer} {status} {:<28} {}\n",
                lesson.title, lesson.track
            ))
        );
        let _ = queue!(stdout, ResetColor);
    }
    let _ = queue!(
        stdout,
        Print(
            "\n○ not started · ◉ in progress · ✓ complete\nN/P or ↑/↓ select · Enter open · L/Esc return\n"
        )
    );
}
