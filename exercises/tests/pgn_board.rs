#[path = "../src/pgn_board.rs"]
mod exercise;

#[test]
fn san_parser_ignores_pgn_scaffolding() {
    let pgn = "[Event \"Demo\"]\n1. e4 e5 2. Nf3 Nc6 {comment} 1-0";
    assert_eq!(
        exercise::parse_san_moves(pgn),
        vec!["e4", "e5", "Nf3", "Nc6"]
    );
}

#[test]
fn each_ply_has_a_saved_board_position() {
    assert_eq!(exercise::position_count(0), 1);
    assert_eq!(exercise::position_count(4), 5);
}
