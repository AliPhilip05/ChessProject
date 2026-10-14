#[path = "../src/capstone_hard.rs"]
mod exercise;

fn round() -> exercise::Round {
    exercise::Round {
        moves: vec!["e4".into(), "e5".into(), "Nf3".into()],
        ply: 0,
        opening: "Italian Game".into(),
        revealed: false,
    }
}

#[test]
fn finish_round_clamps_navigation_and_reveals_only_a_correct_answer() {
    let mut correct = round();
    assert!(exercise::finish_round(&mut correct, 99, " ITALIAN   GAME "));
    assert_eq!(correct.ply, 3);
    assert!(correct.revealed);

    let mut incorrect = round();
    assert!(!exercise::finish_round(&mut incorrect, 1, "Sicilian Defense"));
    assert_eq!(incorrect.ply, 1);
    assert!(!incorrect.revealed);
}
