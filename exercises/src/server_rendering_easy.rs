// NOTE: The move link must identify both the active game session and the selected ply.
// TODO: Add `&ply={ply}` to the already-built session URL.
pub fn move_url(session_id: &str, ply: usize) -> String {
    let base = format!("/move?sid={session_id}");
    base
}
