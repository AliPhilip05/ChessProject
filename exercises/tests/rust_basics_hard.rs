#[path = "../src/rust_basics_hard.rs"]
mod exercise;

#[test]
fn parser_accepts_only_the_two_supported_mode_names() {
    assert_eq!(exercise::parse_mode("elo"), Some(exercise::GameMode::GuessElo));
    assert_eq!(exercise::parse_mode("opening"), Some(exercise::GameMode::GuessOpening));
    assert_eq!(exercise::parse_mode("Opening"), None);
    assert_eq!(exercise::parse_mode("other"), None);
}
