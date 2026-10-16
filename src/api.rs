//! Chess.com public archive access.

mod archives;
mod client;
mod mapping;
mod urls;

pub use archives::{game_from_archive_link, random_public_game};
