pub fn parse_san_moves(pgn: &str) -> Vec<String> {
    let mut moves = Vec::new();
    let mut in_header = false;
    let mut in_braces = false;
    for token in pgn.split_whitespace() {
        if token.starts_with('[') {
            in_header = true;
        }
        if in_header {
            if token.ends_with(']') {
                in_header = false;
            }
            continue;
        }
        if token.starts_with('{') || token.starts_with(';') {
            in_braces = token.starts_with('{') && !token.ends_with('}');
            continue;
        }
        if in_braces {
            if token.ends_with('}') {
                in_braces = false;
            }
            continue;
        }
        let san = token.trim_start_matches(|character: char| character.is_ascii_digit() || character == '.');
        if san.is_empty() || matches!(san, "1-0" | "0-1" | "1/2-1/2" | "*") {
            continue;
        }
        moves.push(san.trim_end_matches(|character| matches!(character, '!' | '?')).to_owned());
    }
    moves
}

pub fn position_count(move_count: usize) -> usize {
    move_count + 1
}
