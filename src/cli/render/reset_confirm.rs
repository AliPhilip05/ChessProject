use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::Write;

use super::super::state::UiState;

pub(super) fn draw_reset_confirm<W: Write>(stdout: &mut W, state: &UiState) {
    let path = state.lesson().id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("Reset current exercise?\n\n"),
        ResetColor,
        Print(format!(
            "This restores exercises/src/{path}.rs from Git HEAD.\n"
        )),
        Print("A copy of the current file will be saved in .gte-backup/.\n\n"),
        SetForegroundColor(Color::White),
        Print("Reset (y/n)? "),
        ResetColor
    );
}
