//! Opening answer matching and rating difficulty rules.

pub fn normalize_answer(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

pub fn opening_matches(guess: &str, expected: &str) -> bool {
    !normalize_answer(guess).is_empty() && normalize_answer(guess) == normalize_answer(expected)
}

pub fn rating_tolerance(difficulty: &str) -> i32 {
    match difficulty.to_ascii_lowercase().as_str() {
        "easy" => 200,
        "hard" => 50,
        _ => 100,
    }
}

pub fn rating_is_close(guess: i32, actual: i32, difficulty: &str) -> bool {
    (guess - actual).abs() <= rating_tolerance(difficulty)
}

#[cfg(test)]
mod tests {
    use super::{opening_matches, rating_is_close};

    #[test]
    fn opening_answers_ignore_case_and_repeated_whitespace() {
        assert!(opening_matches("  ITALIAN   Game ", "Italian Game"));
        assert!(!opening_matches("", ""));
        assert!(!opening_matches("Italian", "Italian Game"));
    }

    #[test]
    fn each_rating_boundary_is_inclusive() {
        for (difficulty, tolerance) in [("EASY", 200), ("medium", 100), ("hard", 50)] {
            assert!(rating_is_close(1_200 - tolerance, 1_200, difficulty));
            assert!(rating_is_close(1_200 + tolerance, 1_200, difficulty));
            assert!(!rating_is_close(1_201 + tolerance, 1_200, difficulty));
        }
    }
}
