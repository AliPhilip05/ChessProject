#[path = "../src/opening_answer.rs"]
mod exercise;

#[test]
fn opening_guesses_ignore_case_and_extra_spaces() {
    assert!(exercise::opening_matches(
        "  ITALIAN   GAME ",
        "Italian Game"
    ));
    assert!(!exercise::opening_matches(
        "Sicilian Defense",
        "Italian Game"
    ));
    assert!(!exercise::opening_matches("   ", "Italian Game"));
}
