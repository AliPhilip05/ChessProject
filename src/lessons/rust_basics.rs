//! Easy, medium, and hard exercises for rust basics.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "rust-basics-easy",
        "01 · Rust foundations · Easy",
        "Label each game mode",
        "An enum match returns the user-facing label associated with each mode.",
        "The enum and match are provided. Replace the two placeholder strings with the correct labels.",
        "Use the visible mode names: `Guess the Elo` and `Guess the Opening`.",
        "What kind of value does the label function return?",
        [
            "A borrowed static string",
            "A network response",
            "A board position",
        ],
        0,
        "solutions/rust_basics_easy.rs",
        "rust-types-and-mode-selection",
    ),
    lesson(
        "rust-basics",
        "01 · Rust foundations · Medium",
        "Represent the two game modes",
        "Enums model the two modes; PartialEq lets tests compare their variants.",
        "Model Guess the Elo and Guess the Opening as explicit Rust modes. Derive the equality traits needed for assert_eq! to compare two mode values.",
        "Start with `enum GameMode`, add one variant per screen, and derive `PartialEq`.",
        "Which trait does assert_eq! use to compare values for equality?",
        ["PartialEq", "Display", "Iterator"],
        0,
        "solutions/rust_basics.rs",
        "rust-types-and-mode-selection",
    ),
    lesson(
        "rust-basics-hard",
        "01 · Rust foundations · Hard",
        "Parse the requested mode",
        "A parser turns external strings into one of a closed set of safe enum alternatives.",
        "Implement the complete `parse_mode` function for the exact mode names `elo` and `opening`; return `None` for anything else.",
        "Use a match with two recognized strings and a catch-all fallback.",
        "What should an unknown mode string produce?",
        ["None", "GuessElo", "A panic"],
        0,
        "solutions/rust_basics_hard.rs",
        "rust-types-and-mode-selection",
    ),
];
