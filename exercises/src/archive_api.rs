// NOTE: Archive URLs arrive oldest first; the app needs the newest entries first.
// TODO: Implement this selection in the function directly below.
pub fn recent_archive_urls(urls: &[String], limit: usize) -> Vec<String> {
    todo!("select the most recent archive URLs, newest first, up to limit")
}

// NOTE: Chess.com usernames may contain ASCII letters, digits, and underscores.
// TODO: Validate the complete username here before a request is made.
pub fn valid_username(username: &str) -> bool {
    todo!("accept a non-empty username made of letters, numbers, or underscores")
}
