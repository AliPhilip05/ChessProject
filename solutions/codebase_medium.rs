#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    GuessElo,
    GuessOpening,
}

pub fn owner(mode: GameMode) -> &'static str {
    match mode {
        GameMode::GuessElo => "src/web.rs",
        GameMode::GuessOpening => "src/web.rs",
    }
}
