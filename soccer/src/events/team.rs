use crate::game::team::*;

use super::state::*;

pub type SoccerTeamStateEnterEvent = StateEnterEvent<SoccerTeamState>;
pub type SoccerTeamStateExitEvent = StateExitEvent<SoccerTeamState>;

pub type FieldPlayerStateEnterEvent = StateEnterEvent<FieldPlayerState>;
pub type FieldPlayerStateExitEvent = StateExitEvent<FieldPlayerState>;

pub type GoalieStateEnterEvent = StateEnterEvent<GoalieState>;
pub type GoalieStateExitEvent = StateExitEvent<GoalieState>;
