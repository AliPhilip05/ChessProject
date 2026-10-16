use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::Write;

use super::super::state::UiState;

pub(super) fn draw_check<W: Write>(stdout: &mut W, state: &UiState) {
    let lesson = state.lesson();
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("KNOWLEDGE CHECK\n"),
        ResetColor
    );
    let _ = queue!(
        stdout,
        Print(format!("{}\n\n{}\n", lesson.title, lesson.question))
    );
    for (index, choice) in lesson.choices.iter().enumerate() {
        let pointer = if index == state.check_selected {
            "›"
        } else {
            " "
        };
        if index == state.check_selected {
            let _ = queue!(stdout, SetForegroundColor(Color::Green));
        }
        let _ = queue!(
            stdout,
            Print(format!(" {pointer} {}. {choice}\n", index + 1))
        );
        let _ = queue!(stdout, ResetColor);
    }
    let _ = queue!(
        stdout,
        Print(
            "\n↑/↓ or 1–3 select · Enter check · R run · N/P next/previous · L list · Esc lesson\n"
        )
    );
}
