//! Board initialization and piece movement geometry.

pub(super) fn starting_position() -> [char; 64] {
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

pub(super) fn piece_can_move(
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
