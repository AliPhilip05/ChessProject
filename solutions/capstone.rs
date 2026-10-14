pub struct GameRecord {
    pub moves: Vec<String>,
    pub white_rating: i32,
    pub black_rating: i32,
    pub opening: String,
}

pub fn clamp_ply(requested: usize, move_count: usize) -> usize {
    requested.min(move_count)
}

pub fn selected_rating_is_close(
    white_guess: Option<i32>,
    black_guess: Option<i32>,
    game: &GameRecord,
    target: &str,
    difficulty: &str,
) -> bool {
    let tolerance = match difficulty.to_ascii_lowercase().as_str() {
        "easy" => 200,
        "hard" => 50,
        _ => 100,
    };
    let white_ok = white_guess
        .map(|guess| (guess - game.white_rating).abs() <= tolerance)
        .unwrap_or(false);
    let black_ok = black_guess
        .map(|guess| (guess - game.black_rating).abs() <= tolerance)
        .unwrap_or(false);
    match target.to_ascii_lowercase().as_str() {
        "white" => white_ok,
        "black" => black_ok,
        _ => white_ok && black_ok,
    }
}

pub fn opening_matches(guess: &str, game: &GameRecord) -> bool {
    let normalize = |text: &str| {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase()
    };
    !normalize(guess).is_empty() && normalize(guess) == normalize(&game.opening)
}
