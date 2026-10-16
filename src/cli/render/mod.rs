mod check;
mod home;
mod lesson;
mod list;
mod reset_confirm;
mod solution;
mod test_result;

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use std::io::Write;

use super::state::{Screen, UiState};
use crate::lessons::LESSONS;

pub(super) fn draw<W: Write>(stdout: &mut W, state: &UiState) {
    let _ = queue!(stdout, MoveTo(0, 0), Clear(ClearType::All));
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(
            " █████    ███████   ██████\n██        ██        ██\n██ ███    █████     █████\n██  ██    ██        ██\n █████    ███████   ██████\n\n  GUESS THE ELO · RUST CODEBASE WALKTHROUGH\n\n"
        )
    );
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(" Rustlings-style project exercises\n\n"),
        ResetColor
    );

    match state.screen {
        Screen::Home => home::draw_home(stdout, state),
        Screen::List => list::draw_list(stdout, state),
        Screen::Lesson => lesson::draw_lesson(stdout, state),
        Screen::Check => check::draw_check(stdout, state),
        Screen::Solution => solution::draw_solution(stdout, state),
        Screen::TestResult => test_result::draw_test_result(stdout, state),
        Screen::ResetConfirm => reset_confirm::draw_reset_confirm(stdout, state),
    }

    if !state.message.is_empty() {
        let _ = queue!(stdout, Print(format!("\n{}\n", state.message)));
    }
    if state.screen != Screen::Home {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("\nQ quit · Esc back · ? shortcuts"),
            ResetColor
        );
    }
    let _ = stdout.flush();
}

pub(super) fn plain_fallback(reason: &str) {
    println!("ChessProject interactive lesson UI\n{reason}\n");
    for (index, lesson) in LESSONS.iter().enumerate() {
        println!("{}. {} — {}", index + 1, lesson.title, lesson.concept);
    }
    println!(
        "\nStart the browser Guidebook with `cargo run -- --web` at http://127.0.0.1:7878/guide."
    );
}
