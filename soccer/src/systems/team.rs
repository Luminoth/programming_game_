use bevy::prelude::*;

use crate::components::team::*;
use crate::game::team::*;

pub fn soccer_team_global_state_execute(mut query: Query<&SoccerTeam>) {
    for soccer_team in query.iter_mut() {
        SoccerTeamState::execute_global(soccer_team);
    }
}

pub fn field_player_global_state_execute(mut query: Query<&Name, With<FieldPlayer>>) {
    for name in query.iter_mut() {
        FieldPlayerState::execute_global(name.as_str());
    }
}

pub fn goalie_global_state_execute(mut query: Query<&Name, With<Goalie>>) {
    for name in query.iter_mut() {
        GoalieState::execute_global(name.as_str());
    }
}
