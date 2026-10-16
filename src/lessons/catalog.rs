//! The ordered curriculum; each topic owns its three exercise records.

use super::model::Lesson;
use super::{
    archive_api, capstone, elo_scoring, opening_answer, orientation, pgn_board, rust_basics,
    server_rendering,
};

/// Eight codebase topics with an easy, medium, and hard exercise per topic.
/// Question 0 is the prefilled first run; all other ids map to Cargo test targets.
pub const LESSONS: [Lesson; 24] = [
    orientation::LESSONS[0],
    orientation::LESSONS[1],
    orientation::LESSONS[2],
    rust_basics::LESSONS[0],
    rust_basics::LESSONS[1],
    rust_basics::LESSONS[2],
    archive_api::LESSONS[0],
    archive_api::LESSONS[1],
    archive_api::LESSONS[2],
    pgn_board::LESSONS[0],
    pgn_board::LESSONS[1],
    pgn_board::LESSONS[2],
    elo_scoring::LESSONS[0],
    elo_scoring::LESSONS[1],
    elo_scoring::LESSONS[2],
    opening_answer::LESSONS[0],
    opening_answer::LESSONS[1],
    opening_answer::LESSONS[2],
    server_rendering::LESSONS[0],
    server_rendering::LESSONS[1],
    server_rendering::LESSONS[2],
    capstone::LESSONS[0],
    capstone::LESSONS[1],
    capstone::LESSONS[2],
];
