#[path = "../src/capstone_easy.rs"]
mod exercise;

#[test]
fn review_range_includes_start_and_final_position() {
    assert_eq!(exercise::reviewable_plies(3), vec![0, 1, 2, 3]);
    assert_eq!(exercise::reviewable_plies(0), vec![0]);
}
