//! Easy, medium, and hard exercises for server rendering.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "server-rendering-easy",
        "06 · Web app and Guidebook · Easy",
        "Build a move-navigation URL",
        "Server-rendered move links carry a session id and selected ply as query parameters.",
        "Finish the provided URL by appending the ply query parameter to the existing session URL.",
        "Use `format!` with both `session_id` and `ply` in the requested order.",
        "Which two values must a move-navigation link preserve?",
        [
            "Session id and ply",
            "Opening and rating",
            "Username and password",
        ],
        0,
        "solutions/server_rendering_easy.rs",
        "web-server-and-rendered-pages",
    ),
    lesson(
        "server-rendering",
        "06 · Web app and Guidebook · Medium",
        "Render a page from Rust",
        "The local server can render the board, move list, and forms in Rust.",
        "Render the current game position and controls from the Rust server. Keep a usable Guidebook route for learners.",
        "A link can carry the session id and requested ply back to a Rust route.",
        "In this prototype, which component owns page and board rendering?",
        [
            "A Rust web handler",
            "A spreadsheet macro",
            "A Chess.com account setting",
        ],
        0,
        "solutions/server_rendering.rs",
        "web-server-and-rendered-pages",
    ),
    lesson(
        "server-rendering-hard",
        "06 · Web app and Guidebook · Hard",
        "Render an escaped notice",
        "User-provided strings must be escaped before they are inserted into the HTML response.",
        "Implement the entire notice renderer, including a helper that escapes ampersands, angle brackets, quotes, and apostrophes.",
        "Escape the input characters one at a time, then insert the escaped values into the trusted wrapper.",
        "Which character must be escaped first to avoid re-escaping generated entities?",
        ["&", "<", ">"],
        0,
        "solutions/server_rendering_hard.rs",
        "web-server-and-rendered-pages",
    ),
];
