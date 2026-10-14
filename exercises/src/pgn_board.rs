// NOTE: Keep SAN moves while skipping headers, comments, move numbers, and results.
// TODO: Parse the PGN text into the move sequence here.
pub fn parse_san_moves(pgn: &str) -> Vec<String> {
    todo!("ignore PGN headers, move numbers, comments, and game results")
}

// NOTE: The board has a starting position plus one snapshot after each ply.
// TODO: Return the number of snapshots required for this move count.
pub fn position_count(move_count: usize) -> usize {
    todo!("include the starting position and one position after each ply")
}
