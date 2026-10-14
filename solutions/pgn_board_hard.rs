pub fn position_at(positions: &[String], requested_ply: usize) -> Option<&str> {
    let final_ply = positions.len().saturating_sub(1);
    positions.get(requested_ply.min(final_ply)).map(String::as_str)
}
