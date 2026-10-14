#[path = "../src/opening_answer_hard.rs"]
mod exercise;

#[test]
fn aliases_match_without_case_or_spacing_sensitivity() {
    let names = ["Italian Game", "Giuoco Piano"];
    assert!(exercise::matches_opening("  GIUOCO   PIANO ", &names));
    assert!(exercise::matches_opening("italian game", &names));
    assert!(!exercise::matches_opening("   ", &names));
    assert!(!exercise::matches_opening("Sicilian Defense", &names));
}
