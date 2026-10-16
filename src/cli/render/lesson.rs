use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::Write;

use super::super::state::UiState;

pub(super) fn draw_lesson<W: Write>(stdout: &mut W, state: &UiState) {
    let lesson = state.lesson();
    let exercise_file = lesson.id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("{}\n", lesson.track)),
        ResetColor
    );
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print(format!("{}\n\n", lesson.title)),
        ResetColor
    );
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "EDIT exercises/src/{exercise_file}.rs\nREFERENCE exercises/reference.md#{}\n\n",
            lesson.reference_anchor
        )),
        ResetColor
    );
    let _ = queue!(
        stdout,
        Print(format!("{}\n\nTASK\n{}\n", lesson.concept, lesson.prompt))
    );
    if state.show_hint {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("\nHINT\n{}\n", lesson.hint)),
            ResetColor
        );
    }
    if state.completed.contains(lesson.id) {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("\n✓ Rust exercise tests passed\n"),
            ResetColor
        );
    }
    let _ = queue!(
        stdout,
        Print(
            "\nN next · P previous · R run check · H hint · C concept check · S solution · G Guidebook · L lesson list\n"
        )
    );
}
