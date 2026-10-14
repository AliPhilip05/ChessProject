#[path = "../src/rust_basics_easy.rs"]
mod exercise;

#[test]
fn labels_match_the_mode_names() {
    assert_eq!(exercise::label(exercise::GameMode::GuessElo), "Guess the Elo");
    assert_eq!(exercise::label(exercise::GameMode::GuessOpening), "Guess the Opening");
}
