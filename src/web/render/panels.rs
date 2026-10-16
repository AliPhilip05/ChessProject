//! Guess forms and their mode-specific feedback panels.
use super::html::html_escape;
use crate::chess::ChessGame;
use crate::web::session::Session;

pub(super) fn render_elo_panel(session: &Session, id: &str, feedback: &str) -> String {
    let white_input = if session.target == "black" {
        String::new()
    } else {
        format!(
            "<label>White · {}<input type=number min=0 max=4000 name=white placeholder=\"Your guess\" required></label>",
            html_escape(&session.game.white_name)
        )
    };
    let black_input = if session.target == "white" {
        String::new()
    } else {
        format!(
            "<label>Black · {}<input type=number min=0 max=4000 name=black placeholder=\"Your guess\" required></label>",
            html_escape(&session.game.black_name)
        )
    };
    let answer_area = if session.revealed {
        feedback.to_owned()
    } else {
        String::new()
    };
    format!(
        include_str!("../../../assets/templates/elo-panel.html"),
        crate::chess::rating_tolerance(&session.difficulty),
        session.difficulty.to_ascii_uppercase(),
        crate::chess::rating_tolerance(&session.difficulty),
        id,
        white_input,
        black_input,
        answer_area,
        session.target.to_ascii_uppercase(),
        session.difficulty.to_ascii_uppercase()
    )
}

pub(super) fn render_opening_panel(session: &Session, id: &str, feedback: &str) -> String {
    let feedback = if session.revealed {
        feedback.to_owned()
    } else {
        String::new()
    };
    format!(
        include_str!("../../../assets/templates/opening-panel.html"),
        session.ply,
        html_escape(&opening_sequence(&session.game)),
        id,
        feedback
    )
}

fn opening_sequence(game: &ChessGame) -> String {
    let mut sequence = String::new();
    for (index, san) in game.moves.iter().take(8).enumerate() {
        if index % 2 == 0 {
            sequence.push_str(&format!("{}.", index / 2 + 1));
        }
        sequence.push(' ');
        sequence.push_str(san);
    }
    sequence.trim().to_owned()
}
