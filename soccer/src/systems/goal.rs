use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;
use crate::components::team::*;
use crate::events::*;

pub fn update<T>(
    mut goal_scored_events: EventWriter<GoalScoredEvent>,
    goal: Query<GoalQuery<T>>,
    ball_transform: Query<&Transform, With<Ball>>,
) where
    T: TeamColorMarker,
{
    let ball_transform = ball_transform.single();

    let goal = goal.single();

    if goal.goal.check_for_score(goal.transform, ball_transform) {
        goal_scored_events.send(GoalScoredEvent(goal.team.team_color()));
    }
}
