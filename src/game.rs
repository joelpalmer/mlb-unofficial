pub enum GameState {
    NoGame,
    Delayed,
    Postponed,
    OnLater,
    InProgress,
    Final,
}

pub struct Game {
    home_team: String,
    away_team: String,
    home_score: i16,
    away_score: i16,
}

impl Game {}
