//! Renders one round, including players, board navigation, and the move list.
use super::board::render_board;
use super::html::{html_escape, page};
use super::panels::{render_elo_panel, render_opening_panel};
use crate::chess::ChessGame;
use crate::web::session::{GameMode, Session, max_ply};

pub(in crate::web) fn render_game(session: &Session, id: &str) -> String {
    let game = &session.game;
    let mode_label = if session.mode == GameMode::Elo {
        "GUESS THE ELO"
    } else {
        "GUESS THE OPENING"
    };
    let limit = max_ply(session);
    let move_list = render_move_list(game, id, session.ply, limit);
    let board = render_board(game.position_at(session.ply));
    let note = if session.note.is_empty() {
        String::new()
    } else {
        format!("<div class=\"toast\">{}</div>", html_escape(&session.note))
    };
    let game_link = if game.source_url.starts_with("https://") {
        format!(
            "<a class=\"source-link\" href=\"{}\" target=\"_blank\" rel=\"noreferrer\">Open on Chess.com ↗</a>",
            html_escape(&game.source_url)
        )
    } else {
        String::new()
    };
    let feedback = if session.feedback.is_empty() {
        String::new()
    } else {
        format!("<div class=\"result-box\">{}</div>", session.feedback)
    };
    let sidebar = if session.mode == GameMode::Elo {
        render_elo_panel(session, id, &feedback)
    } else {
        render_opening_panel(session, id, &feedback)
    };
    let title = if session.mode == GameMode::Elo {
        "Guess the Elo"
    } else {
        "Guess the Opening"
    };
    let body = format!(
        include_str!("../../../assets/templates/game.html"),
        if game.demo {
            "DEMO DATA"
        } else {
            "CHESS.COM ARCHIVE"
        },
        html_escape(title),
        render_player(
            "black",
            &game.black_name,
            game.black_rating,
            session.revealed && session.target != "white"
        ),
        board,
        render_player(
            "white",
            &game.white_name,
            game.white_rating,
            session.revealed && session.target != "black"
        ),
        session.ply.saturating_sub(1),
        session.ply,
        limit,
        session.ply.saturating_add(1).min(limit),
        limit,
        limit,
        move_list,
        sidebar,
        html_escape(&game.result),
        html_escape(&game.time_control),
        html_escape(&game.date),
        if game.eco.is_empty() {
            "—".to_owned()
        } else {
            html_escape(&game.eco)
        },
        game_link,
        mode_label = mode_label,
        id = id,
    );
    page(title, &note, &body)
}

fn render_player(color: &str, name: &str, rating: Option<i32>, revealed: bool) -> String {
    let (icon, class, label) = if color == "white" {
        ("♙", "white-dot", "White")
    } else {
        ("♟", "black-dot", "Black")
    };
    let rating_text = if revealed {
        rating
            .map(|value| format!("Rating · {value}"))
            .unwrap_or_else(|| "Rating unavailable".to_owned())
    } else {
        "Rating hidden".to_owned()
    };
    let visible_rating = if revealed {
        String::new()
    } else {
        "•••".to_owned()
    };
    format!(
        "<div class=\"player-line\"><span class=\"piece-dot {class}\">{icon}</span><div><strong>{}</strong><small>{label} · {}</small></div><span class=\"rating-lock\">{visible_rating}</span></div>",
        html_escape(name),
        html_escape(&rating_text)
    )
}

fn render_move_list(
    game: &ChessGame,
    id: &str,
    selected_ply: usize,
    visible_plies: usize,
) -> String {
    if game.moves.is_empty() {
        return "<p class=\"muted\">No readable moves in this PGN.</p>".to_owned();
    }
    let mut output = String::from("<div class=\"move-grid\">");
    for (index, san) in game.moves.iter().take(visible_plies).enumerate() {
        let ply = index + 1;
        let move_number = (index / 2) + 1;
        let class = if ply == selected_ply {
            "move-token selected"
        } else {
            "move-token"
        };
        if index % 2 == 0 {
            output.push_str(&format!(
                "<span class=\"move-number\">{move_number}.</span>"
            ));
        }
        output.push_str(&format!(
            "<a class=\"{class}\" href=\"/move?sid={id}&amp;ply={ply}\">{}</a>",
            html_escape(san)
        ));
    }
    output.push_str("</div>");
    output
}
