//! Chess game data, PGN replay, and guessing rules.
//!
//! Consumers use this facade; parsing and board mechanics remain internal.

mod answers;
mod board;
mod game;
mod pgn;
mod san;

// Keep the original helper paths available to codebase readers and consumers.
#[allow(unused_imports)]
pub use answers::normalize_answer;
pub use answers::{opening_matches, rating_is_close, rating_tolerance};
pub use game::ChessGame;
#[allow(unused_imports)]
pub use pgn::{pgn_headers, pgn_move_tokens};
