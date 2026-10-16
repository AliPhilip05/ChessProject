//! Easy, medium, and hard exercises for capstone.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "capstone-easy",
        "07 · Capstone · Easy",
        "List every reviewable ply",
        "A game with N plies includes positions from zero through N, inclusive.",
        "Complete the guided range so it includes both the initial position and the final move position.",
        "An inclusive Rust range uses `..=`.",
        "How many reviewable positions are there for a three-ply game?",
        ["4", "3", "2"],
        0,
        "solutions/capstone_easy.rs",
        "capstone-and-end-to-end-flow",
    ),
    lesson(
        "capstone",
        "07 · Capstone · Medium",
        "Build a compact guessing game from scratch",
        "Connect data, move state, scoring, and visible feedback.",
        "Start from a blank file. Implement a small flow that loads or selects one game, steps through its moves, accepts the chosen guess, and reveals a result. You may use the sample data in `exercises/capstone.md` first, then connect the Chess.com archive loader.",
        "Build one small function at a time: parse a record, clamp a move index, score a guess, then render the next state.",
        "Which set of behaviors completes the capstone flow?",
        [
            "Load a game, navigate moves, accept guesses, and reveal feedback",
            "Only print the project title",
            "Fetch a player profile and stop",
        ],
        0,
        "solutions/capstone.rs",
        "capstone-and-end-to-end-flow",
    ),
    lesson(
        "capstone-hard",
        "07 · Capstone · Hard",
        "Finish a guessing round",
        "A round transition combines bounded navigation, normalized text answers, and reveal state.",
        "Build the round record and implement the full `finish_round` function. It clamps the ply, accepts a case- and whitespace-insensitive correct opening, and reveals only on success.",
        "Clamp to the number of plies. Normalize both names by splitting whitespace, joining with one space, and lowercasing.",
        "When should the round reveal the answer?",
        [
            "Only after a nonempty correct guess",
            "As soon as a move is selected",
            "After any text is submitted",
        ],
        0,
        "solutions/capstone_hard.rs",
        "capstone-and-end-to-end-flow",
    ),
];
