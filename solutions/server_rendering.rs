pub fn move_url(session_id: &str, ply: usize) -> String {
    format!("/move?sid={session_id}&ply={ply}")
}

pub fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
