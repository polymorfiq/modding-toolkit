use serde::{Serialize, Deserialize};
use game_base::GameMode;
use game_base::VirtualGameName;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MatchState {
    pub state: String,
    pub players: Vec<Player>
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Player {
    pub username: String,
    pub is_inactive: bool,
    pub is_spectator: bool,
    pub is_bot: bool
}

pub fn get_match_state() -> Option<MatchState> {
    let mut match_state = MatchState::default();
    let game_mode = GameMode::mode()?;
    let players = game_mode.players();
    match_state.state = game_mode.state()?.match_phase.to_string();

    for player in players {
        match_state.players.push(Player{
            username: player.player_name.to_string(),
            is_inactive: player.flags.b_is_inactive(),
            is_spectator: player.flags.b_is_spectator(),
            is_bot: player.flags.b_is_a_bot()
        });
    }

    Some(match_state)
}