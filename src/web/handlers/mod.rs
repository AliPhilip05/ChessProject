//! Mutations and lookups for browser game requests.
mod guesses;
mod navigation;
mod setup;

pub(super) use guesses::{submit_elo_guess, submit_opening_guess};
pub(super) use navigation::{render_session, update_move};
pub(super) use setup::load_game;
