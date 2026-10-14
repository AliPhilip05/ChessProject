// NOTE: This final exercise combines move state, normalized answers, and reveal state.
// TODO: Define the round record and implement the complete state transition below.
pub struct Round {
    pub moves: Vec<String>,
    pub ply: usize,
    pub opening: String,
    pub revealed: bool,
}

// TODO: Clamp the ply, compare normalized answers, reveal only on a correct nonempty guess.
pub fn finish_round(round: &mut Round, requested_ply: usize, guess: &str) -> bool {
    todo!("finish a GTE round from its game data and learner answer")
}
