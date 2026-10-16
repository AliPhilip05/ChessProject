//! Easy, medium, and hard exercises for orientation.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "intro-0",
        "00 · Project orientation · Easy",
        "Question 0 · Find your way around GTE",
        "The launcher chooses between the terminal UI and browser app; six modules divide the app's responsibilities.",
        "This first exercise is already complete. Run its check and read the passing test names to see the GTE logo, both entry points, and the codebase map. No edits are required.",
        "The test checks `product_mark`, `entrypoint`, and `component`. Compare those names with `src/main.rs` and the project map in the Guidebook.",
        "When GTE starts with `--web`, which function does `main` call?",
        ["web::serve", "cli::run", "ChessGame::demo"],
        0,
        "solutions/intro_0.rs",
        "question-zero-project-map",
    ),
    lesson(
        "codebase-medium",
        "00 · Project orientation · Medium",
        "Trace a game mode to its module",
        "Each mode is a Rust enum value, and a match can trace it to the module that implements its behavior.",
        "Complete `owner` so both explicit game modes map to the Rust module that owns their browser flow.",
        "Match each `GameMode` variant and return the stable module path as a string slice.",
        "Which module owns the app's two browser game modes?",
        ["src/web.rs", "src/chess.rs", "src/api.rs"],
        0,
        "solutions/codebase_medium.rs",
        "question-zero-project-map",
    ),
    lesson(
        "codebase-hard",
        "00 · Project orientation · Hard",
        "Route a request to its owner",
        "The browser server separates a request path from its query before dispatching the route.",
        "Implement the entire route lookup. Recognize game and Guidebook paths and return `None` for unknown paths.",
        "Split once at `?`, then match the path. All supported browser routes belong to the web module.",
        "Which module receives `/play?sid=4` after the path is parsed?",
        ["src/web.rs", "src/api.rs", "src/cli.rs"],
        0,
        "solutions/codebase_hard.rs",
        "web-server-and-rendered-pages",
    ),
];
