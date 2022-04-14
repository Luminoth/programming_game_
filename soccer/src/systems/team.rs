pub mod soccer_team {
    use bevy::prelude::*;

    use crate::components::team::SoccerTeam;
    use crate::game::team::SoccerTeamState;

    pub fn global_state_execute(mut query: Query<&SoccerTeam>) {
        for soccer_team in query.iter_mut() {
            SoccerTeamState::execute_global(soccer_team);
        }
    }
}

pub mod field_player {
    use bevy::prelude::*;

    use crate::components::team::FieldPlayer;
    use crate::game::team::FieldPlayerState;

    pub fn global_state_execute(mut query: Query<&Name, With<FieldPlayer>>) {
        for name in query.iter_mut() {
            FieldPlayerState::execute_global(name.as_str());
        }
    }
}

pub mod goalie {
    use bevy::prelude::*;

    use crate::components::team::Goalie;
    use crate::game::team::GoalieState;

    pub fn global_state_execute(mut query: Query<&Name, With<Goalie>>) {
        for name in query.iter_mut() {
            GoalieState::execute_global(name.as_str());
        }
    }
}
