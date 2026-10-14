pub fn recent_archive_urls(urls: &[String], limit: usize) -> Vec<String> {
    urls.iter().rev().take(limit).cloned().collect()
}

pub fn valid_username(username: &str) -> bool {
    !username.is_empty()
        && username.len() <= 25
        && username
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
}

// The app's API module uses these checks before making serial requests to the
// official `/pub/player/{username}/games/archives` endpoint.
