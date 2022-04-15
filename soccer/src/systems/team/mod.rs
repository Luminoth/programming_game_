pub mod field_player;
pub mod goalie;

use bevy::prelude::*;

use crate::components::team::SoccerTeam;
use crate::game::team::SoccerTeamState;

pub fn global_state_execute(mut query: Query<&SoccerTeam>) {
    for soccer_team in query.iter_mut() {
        SoccerTeamState::execute_global(soccer_team);
    }
}
