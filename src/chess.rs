use std::collections::HashMap;

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

pub fn normalize_answer(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

pub fn opening_matches(guess: &str, expected: &str) -> bool {
    !normalize_answer(guess).is_empty() && normalize_answer(guess) == normalize_answer(expected)
}

pub fn rating_tolerance(difficulty: &str) -> i32 {
    match difficulty.to_ascii_lowercase().as_str() {
        "easy" => 200,
        "hard" => 50,
        _ => 100,
    }
}

pub fn rating_is_close(guess: i32, actual: i32, difficulty: &str) -> bool {
    (guess - actual).abs() <= rating_tolerance(difficulty)
}

pub fn pgn_headers(pgn: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in pgn.lines() {
        let line = line.trim();
        if !line.starts_with('[') || !line.ends_with(']') {
            continue;
        }
        let Some(space) = line.find(char::is_whitespace) else {
            continue;
        };
        let key = &line[1..space];
        let value = line[space..].trim().trim_end_matches(']').trim();
        let value = value
            .strip_prefix('"')
            .and_then(|text| text.strip_suffix('"'))
            .unwrap_or(value)
            .replace("\\\"", "\"");
        result.insert(key.to_owned(), value);
    }
    result
}

pub fn pgn_move_tokens(pgn: &str) -> Vec<String> {
    let mut body = String::new();
    let mut in_braces = false;
    let mut variation_depth = 0usize;
    let mut in_line_comment = false;

    for line in pgn.lines() {
        if line.trim().starts_with('[') {
            body.push(' ');
            continue;
        }
        for ch in line.chars() {
            if in_line_comment {
                continue;
            }
            if in_braces {
                if ch == '}' {
                    in_braces = false;
                    body.push(' ');
                }
                continue;
            }
            match ch {
                ';' if variation_depth == 0 => in_line_comment = true,
                '{' if variation_depth == 0 => in_braces = true,
                '(' => variation_depth += 1,
                ')' if variation_depth > 0 => variation_depth -= 1,
                _ if variation_depth == 0 => body.push(ch),
                _ => {}
            }
        }
        in_line_comment = false;
        body.push(' ');
    }

    body.split_whitespace().map(str::to_owned).collect()
}

fn header<'a>(headers: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    headers.get(key).map(String::as_str)
}

fn parse_rating(text: &str) -> Option<i32> {
    text.parse::<i32>().ok().filter(|rating| *rating > 0)
}

fn is_result(token: &str) -> bool {
    matches!(token, "1-0" | "0-1" | "1/2-1/2" | "*")
}

fn strip_move_number(token: &str) -> String {
    let mut index = 0;
    for byte in token.bytes() {
        if byte.is_ascii_digit() || byte == b'.' {
            index += 1;
        } else {
            break;
        }
    }
    token[index..].to_owned()
}

fn starting_position() -> [char; 64] {
    let rows = [
        "rnbqkbnr", "pppppppp", "........", "........", "........", "........", "PPPPPPPP",
        "RNBQKBNR",
    ];
    let mut board = ['.'; 64];
    for (rank, row) in rows.iter().enumerate() {
        for (file, piece) in row.chars().enumerate() {
            board[rank * 8 + file] = piece;
        }
    }
    board
}

fn apply_san(board: &mut [char; 64], raw_san: &str, white_to_move: bool) -> bool {
    let mut san = raw_san.to_owned();
    while san
        .chars()
        .last()
        .is_some_and(|character| matches!(character, '+' | '#' | '!' | '?'))
    {
        san.pop();
    }
    if san.is_empty() || san == "e.p." {
        return false;
    }

    if matches!(san.as_str(), "O-O" | "0-0" | "O-O-O" | "0-0-0") {
        let (king_from, king_to, rook_from, rook_to) = if white_to_move {
            if san.ends_with("O-O-O") || san.ends_with("0-0-0") {
                (60, 58, 56, 59)
            } else {
                (60, 62, 63, 61)
            }
        } else if san.ends_with("O-O-O") || san.ends_with("0-0-0") {
            (4, 2, 0, 3)
        } else {
            (4, 6, 7, 5)
        };
        board[king_to] = board[king_from];
        board[king_from] = '.';
        board[rook_to] = board[rook_from];
        board[rook_from] = '.';
        return true;
    }

    let promotion = san
        .split_once('=')
        .and_then(|(_, value)| value.chars().next());
    if let Some((before, _)) = san.split_once('=') {
        san = before.to_owned();
    }
    let capture = san.contains('x');
    let clean = san.replace('x', "");
    let bytes = clean.as_bytes();
    if bytes.len() < 2 {
        return false;
    }
    let dest_file = bytes[bytes.len() - 2];
    let dest_rank = bytes[bytes.len() - 1];
    if !(b'a'..=b'h').contains(&dest_file) || !(b'1'..=b'8').contains(&dest_rank) {
        return false;
    }
    let to_x = (dest_file - b'a') as i32;
    let to_y = 8 - (dest_rank - b'0') as i32;
    let prefix = &clean[..clean.len() - 2];
    let (piece, disambiguation) = match prefix.chars().next() {
        Some(piece @ ('K' | 'Q' | 'R' | 'B' | 'N')) => (piece, &prefix[1..]),
        _ => ('P', prefix),
    };
    let target = board[(to_y * 8 + to_x) as usize];
    if target != '.' && target.is_ascii_uppercase() == white_to_move {
        return false;
    }
    if (capture && target == '.' && piece != 'P') || (!capture && target != '.') {
        return false;
    }

    let mut origin = None;
    for from_y in 0..8i32 {
        for from_x in 0..8i32 {
            let index = (from_y * 8 + from_x) as usize;
            let candidate = board[index];
            if candidate == '.'
                || candidate.to_ascii_uppercase() != piece
                || candidate.is_ascii_uppercase() != white_to_move
                || !disambiguation_matches(disambiguation, from_x, from_y)
                || !piece_can_move(
                    board,
                    from_x,
                    from_y,
                    to_x,
                    to_y,
                    piece,
                    white_to_move,
                    capture,
                )
            {
                continue;
            }
            origin = Some((from_x, from_y));
            break;
        }
        if origin.is_some() {
            break;
        }
    }
    let Some((from_x, from_y)) = origin else {
        return false;
    };

    let from_index = (from_y * 8 + from_x) as usize;
    let to_index = (to_y * 8 + to_x) as usize;
    let mut moved_piece = board[from_index];
    if piece == 'P' && from_x != to_x && board[to_index] == '.' {
        let captured_y = if white_to_move { to_y + 1 } else { to_y - 1 };
        if (0..8).contains(&captured_y) {
            board[(captured_y * 8 + to_x) as usize] = '.';
        }
    }
    if piece == 'P' && (to_y == 0 || to_y == 7) {
        let promoted = promotion.unwrap_or('Q').to_ascii_uppercase();
        moved_piece = if white_to_move {
            promoted
        } else {
            promoted.to_ascii_lowercase()
        };
    }
    board[from_index] = '.';
    board[to_index] = moved_piece;
    true
}

fn disambiguation_matches(disambiguation: &str, from_x: i32, from_y: i32) -> bool {
    disambiguation.chars().all(|ch| match ch {
        'a'..='h' => (ch as u8 - b'a') as i32 == from_x,
        '1'..='8' => 8 - (ch as u8 - b'0') as i32 == from_y,
        _ => true,
    })
}

fn piece_can_move(
    board: &[char; 64],
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
    piece: char,
    white: bool,
    capture: bool,
) -> bool {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    if dx == 0 && dy == 0 {
        return false;
    }
    match piece {
        'N' => (dx.abs() == 1 && dy.abs() == 2) || (dx.abs() == 2 && dy.abs() == 1),
        'K' => dx.abs() <= 1 && dy.abs() <= 1,
        'R' if dx == 0 || dy == 0 => path_is_clear(board, from_x, from_y, to_x, to_y),
        'B' if dx.abs() == dy.abs() => path_is_clear(board, from_x, from_y, to_x, to_y),
        'Q' if dx == 0 || dy == 0 || dx.abs() == dy.abs() => {
            path_is_clear(board, from_x, from_y, to_x, to_y)
        }
        'P' => {
            let direction = if white { -1 } else { 1 };
            if capture {
                dx.abs() == 1 && dy == direction
            } else if dx != 0 {
                false
            } else if dy == direction {
                board[(to_y * 8 + to_x) as usize] == '.'
            } else {
                let start_rank = if white { 6 } else { 1 };
                dy == direction * 2
                    && from_y == start_rank
                    && board[((from_y + direction) * 8 + from_x) as usize] == '.'
                    && board[(to_y * 8 + to_x) as usize] == '.'
            }
        }
        _ => false,
    }
}

fn path_is_clear(board: &[char; 64], from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> bool {
    let step_x = (to_x - from_x).signum();
    let step_y = (to_y - from_y).signum();
    let mut x = from_x + step_x;
    let mut y = from_y + step_y;
    while x != to_x || y != to_y {
        if board[(y * 8 + x) as usize] != '.' {
            return false;
        }
        x += step_x;
        y += step_y;
    }
    true
}
