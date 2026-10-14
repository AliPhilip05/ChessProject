#[path = "../src/opening_answer_easy.rs"]
mod exercise;

#[test]
fn normalizer_collapses_spaces_and_lowercases_text() {
    assert_eq!(exercise::normalize_opening("  ITALIAN   GAME "), "italian game");
    assert_eq!(exercise::normalize_opening("   "), "");
}
