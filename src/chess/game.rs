//! Game metadata and the sequence of board positions used by the app.

use super::board::starting_position;
use super::pgn::{
    header, is_result, parse_rating, pgn_headers, pgn_move_tokens, strip_move_number,
};
use super::san::apply_san;

#[derive(Clone)]
pub struct ChessGame {
    pub white_name: String,
    pub black_name: String,
    pub white_rating: Option<i32>,
    pub black_rating: Option<i32>,
    pub opening: String,
    pub eco: String,
    pub result: String,
    pub time_control: String,
    pub date: String,
    pub source_url: String,
    pub demo: bool,
    pub moves: Vec<String>,
    positions: Vec<[char; 64]>,
}

impl ChessGame {
    pub fn from_pgn(pgn: &str) -> Self {
        Self::with_metadata(pgn, None, None, None, None, None, false)
    }

    pub fn with_metadata(
        pgn: &str,
        white_rating: Option<i32>,
        black_rating: Option<i32>,
        opening_override: Option<String>,
        eco_override: Option<String>,
        url_override: Option<String>,
        demo: bool,
    ) -> Self {
        let headers = pgn_headers(pgn);
        let mut board = starting_position();
        let mut positions = vec![board];
        let mut moves = Vec::new();
        let mut white_to_move = true;

        for token in pgn_move_tokens(pgn) {
            let san = strip_move_number(&token);
            if san.is_empty() || is_result(&san) || san.starts_with('$') || san == "e.p." {
                continue;
            }
            if apply_san(&mut board, &san, white_to_move) {
                moves.push(san);
                positions.push(board);
                white_to_move = !white_to_move;
            }
        }

        let white_name = header(&headers, "White").unwrap_or("White").to_owned();
        let black_name = header(&headers, "Black").unwrap_or("Black").to_owned();
        let opening = opening_override
            .filter(|value| !value.trim().is_empty())
            .or_else(|| header(&headers, "Opening").map(str::to_owned))
            .unwrap_or_else(|| "Unclassified opening".to_owned());
        let eco = eco_override
            .filter(|value| !value.trim().is_empty())
            .or_else(|| header(&headers, "ECO").map(str::to_owned))
            .unwrap_or_default();
        let source_url = url_override
            .filter(|value| !value.trim().is_empty())
            .or_else(|| header(&headers, "Site").map(str::to_owned))
            .unwrap_or_default();

        Self {
            white_name,
            black_name,
            white_rating: white_rating
                .or_else(|| header(&headers, "WhiteElo").and_then(parse_rating)),
            black_rating: black_rating
                .or_else(|| header(&headers, "BlackElo").and_then(parse_rating)),
            opening,
            eco,
            result: header(&headers, "Result").unwrap_or("*").to_owned(),
            time_control: header(&headers, "TimeControl")
                .unwrap_or("Unknown")
                .to_owned(),
            date: header(&headers, "UTCDate")
                .or_else(|| header(&headers, "Date"))
                .unwrap_or("Date unavailable")
                .to_owned(),
            source_url,
            demo,
            moves,
            positions,
        }
    }

    pub fn demo() -> Self {
        let pgn = r#"
[Event "ChessProject demo game"]
[Site "https://www.chess.com/"]
[Date "2026.10.14"]
[White "QuietKnight"]
[Black "TacticalTea"]
[Result "1-0"]
[WhiteElo "1180"]
[BlackElo "1432"]
[TimeControl "600+5"]
[ECO "C50"]
[Opening "Italian Game"]

1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 4. c3 Nf6 5. d4 exd4 6. cxd4 Bb4+
7. Nc3 Nxe4 8. O-O Bxc3 9. d5 Na5 10. Bd3 f5 1-0
"#;
        let mut game = Self::from_pgn(pgn);
        game.demo = true;
        game
    }

    pub fn position_at(&self, ply: usize) -> &[char; 64] {
        let last = self.positions.len().saturating_sub(1);
        &self.positions[ply.min(last)]
    }

    pub fn ply_count(&self) -> usize {
        self.positions.len().saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::ChessGame;

    #[test]
    fn replay_keeps_initial_position_and_castles_both_pieces() {
        let game = ChessGame::from_pgn("1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 4. O-O");

        assert_eq!(game.ply_count(), 7);
        assert_eq!(game.position_at(0)[60], 'K');
        assert_eq!(game.position_at(7)[62], 'K');
        assert_eq!(game.position_at(7)[61], 'R');
        assert_eq!(game.position_at(7)[60], '.');
        assert_eq!(game.position_at(usize::MAX), game.position_at(7));
    }

    #[test]
    fn replay_removes_the_captured_pawn_for_en_passant() {
        let game = ChessGame::from_pgn("1. e4 a6 2. e5 d5 3. exd6");

        assert_eq!(game.ply_count(), 5);
        assert_eq!(game.position_at(5)[19], 'P');
        assert_eq!(game.position_at(5)[27], '.');
    }
}
