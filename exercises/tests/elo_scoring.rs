#[path = "../src/elo_scoring.rs"]
mod exercise;

#[test]
fn each_difficulty_uses_its_defined_rating_window() {
    assert_eq!(exercise::tolerance("easy"), 200);
    assert_eq!(exercise::tolerance("medium"), 100);
    assert_eq!(exercise::tolerance("hard"), 50);
}

#[test]
fn tolerance_is_inclusive_and_uses_absolute_error() {
    assert!(exercise::within_tolerance(1400, 1500, "medium"));
    assert!(exercise::within_tolerance(1600, 1500, "medium"));
    assert!(!exercise::within_tolerance(1601, 1500, "medium"));
}
