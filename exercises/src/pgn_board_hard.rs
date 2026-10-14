// NOTE: Saved positions include the initial board and one snapshot for every accepted ply.
// TODO: Implement a safe lookup that clamps requests to the final available snapshot.
pub fn position_at(positions: &[String], requested_ply: usize) -> Option<&str> {
    todo!("select a saved position without indexing past the snapshot list")
}
