use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::team::*;
use crate::game::team::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goal {
    pub facing: Vec2,

    // scoring offsets
    pub top: Vec2,
    pub bottom: Vec2,
    pub score_center: Vec2,
}

impl Goal {
    pub fn get_opponent_goal_position<T>(team: &T, goals: &Query<AnyTeamGoalQuery>) -> Option<Vec2>
    where
        T: TeamColorMarker,
    {
        for goal in goals.iter() {
            if (goal.blue_team.is_some() && team.team_color() == TeamColor::Red)
                || (goal.red_team.is_some() && team.team_color() == TeamColor::Blue)
            {
                return Some(goal.transform.translation.truncate());
            }
        }

        None
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct GoalDebug;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct GoalQuery<'w, T>
where
    T: TeamColorMarker,
{
    pub goal: &'w Goal,
    pub team: &'w T,

    pub transform: &'w Transform,
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct AnyTeamGoalQuery<'w> {
    pub goal: &'w Goal,
    pub blue_team: Option<&'w BlueTeam>,
    pub red_team: Option<&'w RedTeam>,

    pub transform: &'w Transform,
}
