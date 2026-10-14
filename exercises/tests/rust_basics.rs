#[path = "../src/rust_basics.rs"]
mod exercise;

#[test]
fn each_game_mode_has_a_readable_label() {
    assert_eq!(exercise::GameMode::GuessElo, exercise::GameMode::GuessElo);
    assert_ne!(
        exercise::GameMode::GuessElo,
        exercise::GameMode::GuessOpening
    );
    assert_eq!(
        exercise::label(exercise::GameMode::GuessElo),
        "Guess the Elo"
    );
    assert_eq!(
        exercise::label(exercise::GameMode::GuessOpening),
        "Guess the Opening"
    );
}
