use crate::chess::ChessGame;
use serde_json::Value;

const USER_AGENT: &str = "ChessProject prototype (Rust learning project)";

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

fn validate_username(username: &str) -> Result<String, String> {
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

fn get_json(url: &str) -> Result<Value, String> {
    let response = ureq::get(url)
        .set("User-Agent", USER_AGENT)
        .call()
        .map_err(|error| format!("Chess.com API request failed: {error}"))?;
    response
        .into_json::<Value>()
        .map_err(|error| format!("Chess.com returned unreadable JSON: {error}"))
}

fn to_chess_game(record: &Value) -> Option<ChessGame> {
    if record
        .get("rules")
        .and_then(Value::as_str)
        .is_some_and(|rules| rules != "chess")
    {
        return None;
    }
    let pgn = record.get("pgn")?.as_str()?;
    let white_rating = record
        .get("white")
        .and_then(|value| value.get("rating"))
        .and_then(Value::as_i64)
        .and_then(|rating| i32::try_from(rating).ok());
    let black_rating = record
        .get("black")
        .and_then(|value| value.get("rating"))
        .and_then(Value::as_i64)
        .and_then(|rating| i32::try_from(rating).ok());
    let white_name = record
        .get("white")
        .and_then(|value| value.get("username"))
        .and_then(Value::as_str);
    let black_name = record
        .get("black")
        .and_then(|value| value.get("username"))
        .and_then(Value::as_str);
    let opening = record
        .get("opening")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .or_else(|| {
            record
                .get("eco")
                .and_then(Value::as_str)
                .map(opening_from_eco)
        });
    let url = record.get("url").and_then(Value::as_str).map(str::to_owned);
    let mut game =
        ChessGame::with_metadata(pgn, white_rating, black_rating, opening, None, url, false);
    if let Some(name) = white_name {
        game.white_name = name.to_owned();
    }
    if let Some(name) = black_name {
        game.black_name = name.to_owned();
    }
    if let Some(time_control) = record.get("time_control").and_then(Value::as_str) {
        game.time_control = time_control.to_owned();
    }
    Some(game)
}

fn same_game_id(first: &str, second: &str) -> bool {
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

fn opening_from_eco(url: &str) -> String {
    url.rsplit('/').next().unwrap_or(url).replace('-', " ")
}
