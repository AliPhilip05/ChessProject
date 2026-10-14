pub fn move_url(session_id: &str, ply: usize) -> String {
    format!("/move?sid={session_id}&ply={ply}")
}
