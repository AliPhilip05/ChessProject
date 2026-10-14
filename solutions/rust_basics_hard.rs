#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    GuessElo,
    GuessOpening,
}

pub fn parse_mode(argument: &str) -> Option<GameMode> {
    match argument {
        "elo" => Some(GameMode::GuessElo),
        "opening" => Some(GameMode::GuessOpening),
        _ => None,
    }
}
