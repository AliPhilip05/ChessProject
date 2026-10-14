pub struct Round {
    pub moves: Vec<String>,
    pub ply: usize,
    pub opening: String,
    pub revealed: bool,
}

pub fn finish_round(round: &mut Round, requested_ply: usize, guess: &str) -> bool {
    round.ply = requested_ply.min(round.moves.len());
    let normalize = |value: &str| {
        value.split_whitespace().collect::<Vec<_>>().join(" ").to_ascii_lowercase()
    };
    let guess = normalize(guess);
    let correct = !guess.is_empty() && guess == normalize(&round.opening);
    if correct {
        round.revealed = true;
    }
    correct
}
