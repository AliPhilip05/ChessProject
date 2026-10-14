// NOTE: PGN contains metadata tokens as well as moves; only move tokens belong on the board.
// TODO: Exclude blank text, move numbers, results, and numeric annotation glyphs.
pub fn is_san_move(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|character| character.is_ascii_alphabetic())
}
