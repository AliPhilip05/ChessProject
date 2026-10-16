//! Rust rendering logic for the game pages and reusable HTML components.
mod board;
mod game;
mod home;
mod html;
mod panels;

pub(super) use game::render_game;
pub(super) use home::render_home;
pub(super) use html::html_escape;
