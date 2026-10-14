// NOTE: Assemble a small game model that supports both guessing modes.
// TODO: Implement each behavior below, using this game record as the starting model.
pub struct GameRecord {
    pub moves: Vec<String>,
    pub white_rating: i32,
    pub black_rating: i32,
    pub opening: String,
}

// NOTE: Valid board positions range from the initial position through the final ply.
// TODO: Clamp the requested position to that inclusive range.
pub fn clamp_ply(requested: usize, move_count: usize) -> usize {
    todo!("a position can range from the starting position through the final ply")
}

// NOTE: Score only the requested side, or require both guesses to be within range.
// TODO: Apply the selected difficulty tolerance to the chosen player ratings.
pub fn selected_rating_is_close(
    white_guess: Option<i32>,
    black_guess: Option<i32>,
    game: &GameRecord,
    target: &str,
    difficulty: &str,
) -> bool {
    todo!("score the selected player or players with the correct tolerance")
}

// NOTE: Match the opening name without case or whitespace sensitivity.
// TODO: Normalize the guess and stored opening before comparing them.
pub fn opening_matches(guess: &str, game: &GameRecord) -> bool {
    todo!("compare the opening answer without case or whitespace sensitivity")
}
