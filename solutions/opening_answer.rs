pub fn opening_matches(guess: &str, expected: &str) -> bool {
    guess.trim().split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
        == expected.trim().split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}
