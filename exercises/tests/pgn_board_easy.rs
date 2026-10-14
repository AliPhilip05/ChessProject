#[path = "../src/pgn_board_easy.rs"]
mod exercise;

#[test]
fn move_recognizer_filters_pgn_scaffolding() {
    for token in ["e4", "Nf3", "O-O", "Qxe5+"] {
        assert!(exercise::is_san_move(token), "expected move: {token}");
    }
    for token in ["1.", "1...", "1-0", "*", "$1", ""] {
        assert!(!exercise::is_san_move(token), "expected metadata: {token}");
    }
}
