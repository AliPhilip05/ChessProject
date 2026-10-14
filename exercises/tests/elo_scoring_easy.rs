#[path = "../src/elo_scoring_easy.rs"]
mod exercise;

#[test]
fn error_is_the_same_on_either_side_of_the_actual_rating() {
    assert_eq!(exercise::rating_error(1400, 1500), 100);
    assert_eq!(exercise::rating_error(1600, 1500), 100);
    assert_eq!(exercise::rating_error(1500, 1500), 0);
}
