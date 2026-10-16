//! Browser game state and the in-memory session store.
use crate::chess::ChessGame;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum GameMode {
    Elo,
    Opening,
}

pub(super) struct Session {
    pub(super) game: ChessGame,
    pub(super) mode: GameMode,
    pub(super) difficulty: String,
    pub(super) target: String,
    pub(super) ply: usize,
    pub(super) note: String,
    pub(super) feedback: String,
    pub(super) revealed: bool,
}

static SESSIONS: OnceLock<Mutex<HashMap<String, Session>>> = OnceLock::new();
pub(super) static NEXT_SESSION: AtomicU64 = AtomicU64::new(1);

pub(super) fn sessions() -> &'static Mutex<HashMap<String, Session>> {
    SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(super) fn max_ply(session: &Session) -> usize {
    let game_count = session.game.ply_count();
    if session.mode == GameMode::Opening {
        game_count.min(8)
    } else {
        game_count
    }
}
