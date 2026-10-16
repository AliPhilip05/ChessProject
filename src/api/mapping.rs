//! Map one Chess.com archive record into the app's game model.

use crate::chess::ChessGame;
use serde_json::Value;

use super::urls::opening_from_eco;

pub(super) fn to_chess_game(record: &Value) -> Option<ChessGame> {
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::to_chess_game;

    #[test]
    fn archive_metadata_overrides_pgn_headers() {
        let record = json!({
            "rules": "chess",
            "pgn": "[White \"PGN White\"]\n[WhiteElo \"900\"]\n[Opening \"PGN opening\"]\n1. e4 e5",
            "white": {"username": "ArchiveWhite", "rating": 1200},
            "black": {"username": "ArchiveBlack", "rating": 1300},
            "opening": "Italian Game",
            "url": "https://www.chess.com/game/live/123",
            "time_control": "600"
        });

        let game = to_chess_game(&record).unwrap();
        assert_eq!(game.white_name, "ArchiveWhite");
        assert_eq!(game.black_name, "ArchiveBlack");
        assert_eq!(game.white_rating, Some(1200));
        assert_eq!(game.black_rating, Some(1300));
        assert_eq!(game.opening, "Italian Game");
        assert_eq!(game.time_control, "600");
        assert_eq!(game.ply_count(), 2);
    }

    #[test]
    fn variants_and_records_without_pgn_are_excluded() {
        assert!(to_chess_game(&json!({"rules": "chess960", "pgn": "1. e4"})).is_none());
        assert!(to_chess_game(&json!({"rules": "chess"})).is_none());
    }
}
