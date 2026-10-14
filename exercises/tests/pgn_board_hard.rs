#[path = "../src/pgn_board_hard.rs"]
mod exercise;

#[test]
fn position_lookup_includes_the_start_and_clamps_to_the_end() {
    let positions = vec!["start".into(), "after e4".into(), "after e5".into()];
    assert_eq!(exercise::position_at(&positions, 0), Some("start"));
    assert_eq!(exercise::position_at(&positions, 1), Some("after e4"));
    assert_eq!(exercise::position_at(&positions, 99), Some("after e5"));
    assert_eq!(exercise::position_at(&[], 0), None);
}
