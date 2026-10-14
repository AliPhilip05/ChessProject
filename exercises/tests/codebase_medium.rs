#[path = "../src/codebase_medium.rs"]
mod exercise;

#[test]
fn both_modes_are_distinct_and_owned_by_the_web_layer() {
    let elo = exercise::GameMode::GuessElo;
    let opening = exercise::GameMode::GuessOpening;
    assert_ne!(elo, opening);
    assert_eq!(exercise::owner(elo), "src/web.rs");
    assert_eq!(exercise::owner(opening), "src/web.rs");
}
