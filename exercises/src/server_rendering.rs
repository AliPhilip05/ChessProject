// NOTE: The browser uses this URL to request a specific saved board position.
// TODO: Include both the session identifier and ply in the URL.
pub fn move_url(session_id: &str, ply: usize) -> String {
    todo!("build the move-navigation URL used by the server-rendered page")
}

// NOTE: User-provided text must be escaped before it is inserted into HTML.
// TODO: Escape ampersands, angle brackets, quotes, and apostrophes here.
pub fn escape_html(text: &str) -> String {
    todo!("escape &, <, >, double quote, and apostrophe")
}
