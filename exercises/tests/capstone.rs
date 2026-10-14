#[path = "../src/capstone.rs"]
mod exercise;

fn demo() -> exercise::GameRecord {
    exercise::GameRecord {
        moves: vec!["e4".into(), "e5".into(), "Nf3".into()],
        white_rating: 1180,
        black_rating: 1432,
        opening: "Italian Game".into(),
    }
}

#[test]
fn capstone_bounds_navigation_and_scores_selected_players() {
    let game = demo();
    assert_eq!(exercise::clamp_ply(0, game.moves.len()), 0);
    assert_eq!(exercise::clamp_ply(99, game.moves.len()), game.moves.len());
    assert!(exercise::selected_rating_is_close(
        Some(1200),
        None,
        &game,
        "white",
        "easy"
    ));
    assert!(!exercise::selected_rating_is_close(
        Some(1200),
        None,
        &game,
        "black",
        "easy"
    ));
    assert!(exercise::selected_rating_is_close(
        Some(1180),
        Some(1432),
        &game,
        "both",
        "hard"
    ));
}

#[test]
fn capstone_accepts_a_case_insensitive_opening_guess() {
    let game = demo();
    assert!(exercise::opening_matches("  ITALIAN GAME ", &game));
}
