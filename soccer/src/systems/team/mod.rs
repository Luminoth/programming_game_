pub mod field_player;
pub mod goalie;

use bevy::prelude::*;

use crate::components::team::SoccerTeam;
use crate::game::team::SoccerTeamState;

pub fn global_state_execute(query: Query<&mut SoccerTeam>) {
    for soccer_team in query.iter() {
        SoccerTeamState::execute_global(soccer_team);
    }
}
