//! Easy, medium, and hard exercises for pgn board.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "pgn-board-easy",
        "03 · PGN and board state · Easy",
        "Recognize a move token",
        "PGN move text includes move numbers, results, and annotation tokens alongside SAN moves.",
        "Complete the token predicate so it distinguishes a chess move from common PGN scaffolding.",
        "Reject blank strings, move-number punctuation, result markers, and tokens beginning with `$`.",
        "Which token is a game result rather than a move?",
        ["1-0", "Nf3", "O-O"],
        0,
        "solutions/pgn_board_easy.rs",
        "pgn-parsing-and-board-positions",
    ),
    lesson(
        "pgn-board",
        "03 · PGN and board state · Medium",
        "Turn PGN into a move review",
        "A PGN is metadata plus an ordered sequence of moves.",
        "Separate PGN headers from SAN moves. Keep a board position after every ply so the previous and next controls can render the same game at different points.",
        "Store `positions[0]` as the starting board, then append one board after each applied move.",
        "How can move navigation return to an earlier position without replaying the whole game?",
        [
            "Save a position snapshot after each move",
            "Only save the last FEN",
            "Hide the move list",
        ],
        0,
        "solutions/pgn_board.rs",
        "pgn-parsing-and-board-positions",
    ),
    lesson(
        "pgn-board-hard",
        "03 · PGN and board state · Hard",
        "Navigate a saved position",
        "Saved positions include the initial position and one snapshot per accepted ply.",
        "Implement the complete lookup. Clamp an oversized requested ply to the final saved position and handle an empty snapshot list safely.",
        "The last valid index is `positions.len().saturating_sub(1)`. Then use `get` rather than indexing directly.",
        "What should position lookup return when there are no snapshots?",
        ["None", "The starting position", "An index panic"],
        0,
        "solutions/pgn_board_hard.rs",
        "pgn-parsing-and-board-positions",
    ),
];
