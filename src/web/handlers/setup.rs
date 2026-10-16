//! Loads a game source and creates a round session.
use crate::api;
use crate::chess::ChessGame;
use crate::web::http::HttpResponse;
use crate::web::request::{normalize_difficulty, normalize_target};
use crate::web::session::{GameMode, NEXT_SESSION, Session, sessions};
use std::collections::HashMap;
use std::sync::atomic::Ordering;

pub(in crate::web) fn load_game(params: &HashMap<String, String>) -> HttpResponse {
    let mode = if params.get("mode").map(String::as_str) == Some("opening") {
        GameMode::Opening
    } else {
        GameMode::Elo
    };
    let difficulty = params
        .get("difficulty")
        .map(|value| normalize_difficulty(value))
        .unwrap_or_else(|| "medium".to_owned());
    let target = params
        .get("target")
        .map(|value| normalize_target(value))
        .unwrap_or_else(|| "both".to_owned());
    let username = params.get("username").map(String::as_str).unwrap_or("erik");
    let source = params.get("source").map(String::as_str).unwrap_or("demo");

    let (game, note) = match source {
        "random" => match api::random_public_game(username) {
            Ok(game) => (game, String::new()),
            Err(error) => (
                ChessGame::demo(),
                format!("API fallback: {error} The included sample game is shown instead."),
            ),
        },
        "link" => {
            let url = params.get("url").map(String::as_str).unwrap_or_default();
            match api::game_from_archive_link(username, url) {
                Ok(game) => (game, String::new()),
                Err(error) => (
                    ChessGame::demo(),
                    format!(
                        "Game link unavailable: {error} Demo data is shown so you can still explore the board."
                    ),
                ),
            }
        }
        _ => (
            ChessGame::demo(),
            "Demo data · this built-in game works without a network connection.".to_owned(),
        ),
    };
    if game.ply_count() == 0 {
        let demo = ChessGame::demo();
        let note = if note.is_empty() {
            "The loaded PGN had no readable moves, so the demo game is shown.".to_owned()
        } else {
            note
        };
        return create_session(demo, mode, difficulty, target, note);
    }
    create_session(game, mode, difficulty, target, note)
}

fn create_session(
    game: ChessGame,
    mode: GameMode,
    difficulty: String,
    target: String,
    note: String,
) -> HttpResponse {
    let id = NEXT_SESSION.fetch_add(1, Ordering::Relaxed).to_string();
    let ply = if mode == GameMode::Opening {
        8.min(game.ply_count())
    } else {
        0
    };
    let session = Session {
        game,
        mode,
        difficulty,
        target,
        ply,
        note,
        feedback: String::new(),
        revealed: false,
    };
    if let Ok(mut store) = sessions().lock() {
        store.insert(id.clone(), session);
    }
    HttpResponse::Redirect(format!("/play?sid={id}"))
}
