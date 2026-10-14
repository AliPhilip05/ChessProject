#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    GuessElo,
    GuessOpening,
}

pub fn label(mode: GameMode) -> &'static str {
    match mode {
        GameMode::GuessElo => "Guess the Elo",
        GameMode::GuessOpening => "Guess the Opening",
    }
}
