use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MatchState {
    state: String,
    players: Vec<Player>
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Player {
    username: String
}