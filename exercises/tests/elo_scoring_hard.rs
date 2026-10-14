#[path = "../src/elo_scoring_hard.rs"]
mod exercise;

#[test]
fn scoring_preserves_missing_values_and_includes_the_limit() {
    assert_eq!(exercise::evaluate_guess(None, Some(1500), 100), None);
    assert_eq!(exercise::evaluate_guess(Some(1400), None, 100), None);
    assert_eq!(exercise::evaluate_guess(Some(1400), Some(1500), 100), Some(true));
    assert_eq!(exercise::evaluate_guess(Some(1399), Some(1500), 100), Some(false));
    assert_eq!(exercise::evaluate_guess(Some(1500), Some(1500), -1), None);
}
