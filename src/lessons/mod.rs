//! Curriculum metadata, organized by responsibility and exercise topic.

mod archive_api;
mod capstone;
mod catalog;
mod elo_scoring;
mod model;
mod opening_answer;
mod orientation;
mod pgn_board;
mod rust_basics;
mod server_rendering;

pub use catalog::LESSONS;
pub use model::Lesson;
