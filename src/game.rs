mod mlbapi;
pub enum GameState {
    NoGame,
    Delayed,
    Postponed,
    OnLater,
    InProgress,
    Final,
}

pub struct Game {
    pub home_team: String,
    pub away_team: String,
    pub home_score: i16,
    pub away_score: i16,
}

impl Game {
    fn get_game_data<'a>(team: &str, date: &str) -> Option<&'a str> {
        // try the team and date against the MLB api
        unimplemented!();
    }

    pub fn new(team: &str, date: &str) -> Game {
        // call get game data and return the game w/ the data
        // fake response...
        Game {
            home_team: "Dodgers".to_string(),
            away_team: "Giants".to_string(),
            home_score: 22,
            away_score: 0,
        }
    }
}
