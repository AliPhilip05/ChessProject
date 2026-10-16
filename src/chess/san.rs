//! Apply Standard Algebraic Notation moves to a board snapshot.

use super::board::piece_can_move;

pub(super) fn apply_san(board: &mut [char; 64], raw_san: &str, white_to_move: bool) -> bool {
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
