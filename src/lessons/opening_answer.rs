//! Easy, medium, and hard exercises for opening answer.

use super::model::{Lesson, lesson};

pub(super) const LESSONS: [Lesson; 3] = [
    lesson(
        "opening-answer-easy",
        "05 · Guess the Opening · Easy",
        "Normalize an opening name",
        "Whitespace splitting plus a lowercase conversion makes answer text consistent.",
        "The word collection and joining steps are provided. Finish the normalization by lowercasing the joined answer.",
        "Call `to_ascii_lowercase` on the joined string before returning it.",
        "How should `ITALIAN   GAME` normalize?",
        ["italian game", "ITALIAN   GAME", "Italian"],
        0,
        "solutions/opening_answer_easy.rs",
        "opening-answers",
    ),
    lesson(
        "opening-answer",
        "05 · Guess the Opening · Medium",
        "Normalize opening guesses",
        "Text answers can ignore capitalization and extra surrounding whitespace.",
        "Compare the learner's opening guess with the expected name without treating capitalization as meaningful.",
        "Trim both values, then compare their lowercase forms.",
        "What result should `  ITALIAN GAME ` produce when the answer is `Italian Game`?",
        [
            "Correct",
            "Incorrect because of capitalization",
            "An API error",
        ],
        0,
        "solutions/opening_answer.rs",
        "opening-answers",
    ),
    lesson(
        "opening-answer-hard",
        "05 · Guess the Opening · Hard",
        "Match an accepted opening alias",
        "A canonical opening may have more than one accepted human-readable name.",
        "Write the full case-insensitive, whitespace-insensitive matcher for a guess and a list of accepted opening names.",
        "Normalize the guess once, reject it when blank, then compare it against every normalized accepted name.",
        "Should an empty guess match an empty alias list?",
        ["No", "Yes", "Only in Easy mode"],
        0,
        "solutions/opening_answer_hard.rs",
        "opening-answers",
    ),
];
