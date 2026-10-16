//! Username validation and Chess.com URL metadata helpers.

pub(super) fn validate_username(username: &str) -> Result<String, String> {
    let username = username.trim().to_ascii_lowercase();
    if username.is_empty()
        || username.len() > 25
        || !username
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(
            "Enter a Chess.com username using letters, numbers, or underscores.".to_owned(),
        );
    }
    Ok(username)
}

pub(super) fn same_game_id(first: &str, second: &str) -> bool {
    let first_id = game_id(first);
    !first_id.is_empty() && first_id == game_id(second)
}

fn game_id(url: &str) -> &str {
    url.split('/')
        .filter(|part| !part.is_empty())
        .next_back()
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or_default()
}

pub(super) fn opening_from_eco(url: &str) -> String {
    url.rsplit('/').next().unwrap_or(url).replace('-', " ")
}

#[cfg(test)]
mod tests {
    use super::{same_game_id, validate_username};

    #[test]
    fn usernames_are_normalized_before_use_in_api_paths() {
        assert_eq!(validate_username("  Player_1 ").unwrap(), "player_1");
        assert!(validate_username("player/name").is_err());
        assert!(validate_username("").is_err());
    }

    #[test]
    fn game_links_match_across_host_and_query_variants() {
        assert!(same_game_id(
            "https://www.chess.com/game/live/12345",
            "https://chess.com/game/live/12345?move=4"
        ));
        assert!(!same_game_id("", ""));
        assert!(!same_game_id(
            "https://chess.com/game/live/1",
            "https://chess.com/game/live/2"
        ));
    }
}
