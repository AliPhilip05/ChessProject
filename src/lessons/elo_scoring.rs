//! Easy, medium, and hard exercises for elo scoring.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "elo-scoring-easy",
        "04 · Guess the Elo · Easy",
        "Calculate rating error",
        "A rating miss is a non-negative absolute distance on either side of the actual rating.",
        "Complete the guided function so guesses above and below the actual rating produce the same error size.",
        "Use the integer absolute-value method after subtracting the ratings.",
        "If the guess is 100 points below the actual rating, what is its error?",
        ["100", "-100", "0"],
        0,
        "solutions/elo_scoring_easy.rs",
        "elo-scoring",
    ),
    lesson(
        "elo-scoring",
        "04 · Guess the Elo · Medium",
        "Score an Elo estimate",
        "Difficulty is an absolute-error threshold around the actual rating.",
        "For a submitted guess, calculate the absolute difference from the hidden rating. Easy accepts 200 points, Medium 100, and Hard 50. Apply scoring only to the selected player or players.",
        "The core expression is `(guess - actual).abs() <= tolerance`.",
        "At Medium difficulty, which guess is accepted for an actual rating of 1500?",
        ["1399", "1400", "1601"],
        1,
        "solutions/elo_scoring.rs",
        "elo-scoring",
    ),
    lesson(
        "elo-scoring-hard",
        "04 · Guess the Elo · Hard",
        "Evaluate a possibly missing guess",
        "Optional ratings preserve missing archive data, and a tolerance boundary is inclusive.",
        "Implement the whole scoring function. Return `None` if the guess, rating, or tolerance is invalid; otherwise return whether the error is within tolerance.",
        "Use `?` to unwrap both options and reject a negative tolerance before comparing absolute error.",
        "If the actual rating is absent, what result preserves that uncertainty?",
        ["None", "Some(false)", "Zero"],
        0,
        "solutions/elo_scoring_hard.rs",
        "elo-scoring",
    ),
];
