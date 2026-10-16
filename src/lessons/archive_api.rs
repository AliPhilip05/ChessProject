//! Easy, medium, and hard exercises for archive api.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "archive-api-easy",
        "02 · Chess.com data · Easy",
        "Select the newest archives",
        "The archive index is ordered oldest first, so recent entries must be visited in reverse.",
        "Complete the guided loop by adding each visited archive URL to the output vector.",
        "The iterator already visits no more than `limit` URLs in newest-first order.",
        "If the list is ordered January, February, March, what is the first selected month?",
        ["March", "January", "It is random"],
        0,
        "solutions/archive_api_easy.rs",
        "chesscom-archive-data",
    ),
    lesson(
        "archive-api",
        "02 · Chess.com data · Medium",
        "Load a public archive safely",
        "The public API lists archive URLs before it returns monthly games.",
        "Describe the request flow for random selection: obtain a player's archive list, load one archive, and choose one eligible chess game. Keep API failures visible to the user.",
        "Use the archive index first, then make a serial request to a month URL from that response.",
        "What is the first API request needed to find a player's archived games?",
        [
            "Fetch `/pub/player/{username}/games/archives`",
            "Send a move to the live game",
            "Guess a monthly URL without checking",
        ],
        0,
        "solutions/archive_api.rs",
        "chesscom-archive-data",
    ),
    lesson(
        "archive-api-hard",
        "02 · Chess.com data · Hard",
        "Find the newest usable month",
        "Recent archives may be empty, and a complete search needs a safe no-results outcome.",
        "Write the whole function that scans monthly game lists from newest to oldest and returns the first nonempty month.",
        "Reverse the iterator, find the first vector with at least one game, and return it as a slice.",
        "What should the search return if every archive month is empty?",
        ["None", "An empty URL", "A network request"],
        0,
        "solutions/archive_api_hard.rs",
        "chesscom-archive-data",
    ),
];
