//! Choose public games and search a player's monthly archives.

use crate::chess::ChessGame;
use serde_json::Value;

use super::client::get_json;
use super::mapping::to_chess_game;
use super::urls::{same_game_id, validate_username};

pub fn random_public_game(username: &str) -> Result<ChessGame, String> {
    let username = validate_username(username)?;
    let archives = archive_urls(&username)?;
    let mut candidates = Vec::new();

    for archive in archives.iter().rev().take(4) {
        let response = get_json(archive)?;
        if let Some(games) = response.get("games").and_then(Value::as_array) {
            candidates.extend(games.iter().filter_map(to_chess_game));
        }
        if !candidates.is_empty() {
            break;
        }
    }

    if candidates.is_empty() {
        return Err(format!("No public chess games were found for {username}."));
    }
    let tick = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as usize)
        .unwrap_or(0);
    let index = tick % candidates.len();
    Ok(candidates.swap_remove(index))
}

pub fn game_from_archive_link(username: &str, game_url: &str) -> Result<ChessGame, String> {
    let username = validate_username(username)?;
    let requested_url = game_url.trim();
    if !requested_url.starts_with("https://www.chess.com/game/")
        && !requested_url.starts_with("https://chess.com/game/")
    {
        return Err("Paste a public Chess.com game URL.".to_owned());
    }

    let archives = archive_urls(&username)?;
    for archive in archives.iter().rev().take(12) {
        let response = get_json(archive)?;
        let Some(games) = response.get("games").and_then(Value::as_array) else {
            continue;
        };
        for game in games {
            let url = game.get("url").and_then(Value::as_str).unwrap_or_default();
            if url == requested_url || same_game_id(url, requested_url) {
                return to_chess_game(game)
                    .ok_or_else(|| "That archive entry does not include a usable PGN.".to_owned());
            }
        }
    }
    Err(format!(
        "That game was not found in {username}'s recent public archives. Try another username or load a random archive game."
    ))
}

fn archive_urls(username: &str) -> Result<Vec<String>, String> {
    let url = format!("https://api.chess.com/pub/player/{username}/games/archives");
    let response = get_json(&url)?;
    let Some(archives) = response.get("archives").and_then(Value::as_array) else {
        return Err("Chess.com returned no archive list for that username.".to_owned());
    };
    let urls = archives
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if urls.is_empty() {
        return Err("This Chess.com player has no public archived games.".to_owned());
    }
    Ok(urls)
}
