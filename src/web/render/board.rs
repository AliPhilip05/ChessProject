//! Board squares and chess-piece presentation.
use super::html::html_escape;

pub(super) fn render_board(position: &[char; 64]) -> String {
    let mut output =
        String::from("<div class=\"board\" role=\"img\" aria-label=\"Chess position\">");
    for row in 0..8 {
        for column in 0..8 {
            let piece = position[row * 8 + column];
            let square = if (row + column) % 2 == 0 {
                "light-square"
            } else {
                "dark-square"
            };
            let symbol = chess_symbol(piece);
            let label = if piece == '.' {
                "empty".to_owned()
            } else {
                format!(
                    "{} on {}{}",
                    piece_name(piece),
                    (b'a' + column as u8) as char,
                    8 - row
                )
            };
            output.push_str(&format!(
                "<div class=\"square {square}\" aria-label=\"{}\">{symbol}</div>",
                html_escape(&label)
            ));
        }
    }
    output.push_str("</div>");
    output
}

pub(super) fn mini_board(position: &[char; 64]) -> String {
    let mut output = String::from("<div class=\"mini-board-grid\">");
    for row in 0..8 {
        for column in 0..8 {
            let piece = position[row * 8 + column];
            let square = if (row + column) % 2 == 0 {
                "light-square"
            } else {
                "dark-square"
            };
            output.push_str(&format!(
                "<div class=\"square {square}\">{}</div>",
                chess_symbol(piece)
            ));
        }
    }
    output.push_str("</div>");
    output
}

fn chess_symbol(piece: char) -> &'static str {
    match piece {
        'K' => "♔",
        'Q' => "♕",
        'R' => "♖",
        'B' => "♗",
        'N' => "♘",
        'P' => "♙",
        'k' => "♚",
        'q' => "♛",
        'r' => "♜",
        'b' => "♝",
        'n' => "♞",
        'p' => "♟",
        _ => "",
    }
}

fn piece_name(piece: char) -> &'static str {
    match piece.to_ascii_uppercase() {
        'K' => "king",
        'Q' => "queen",
        'R' => "rook",
        'B' => "bishop",
        'N' => "knight",
        _ => "pawn",
    }
}
