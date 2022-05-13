use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::team::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goal {
    pub facing: Vec2,

    // scoring offsets
    pub top: Vec2,
    pub bottom: Vec2,
    pub score_center: Vec2,

    pub goals_scored: usize,
}

impl Goal {
    pub fn get_top(&self, transform: &Transform) -> Vec2 {
        let position = transform.translation.truncate();
        position + self.top
    }

    pub fn get_bottom(&self, transform: &Transform) -> Vec2 {
        let position = transform.translation.truncate();
        position + self.bottom
    }

    pub fn get_score_center(&self, transform: &Transform) -> Vec2 {
        let position = transform.translation.truncate();
        position + self.score_center
    }

    pub fn check_for_score(&mut self, transform: &Transform, ball_transform: &Transform) -> bool {
        let top = self.get_top(transform);
        let bottom = self.get_bottom(transform);

        let ball_position = ball_transform.translation.truncate();
        // TODO:
        /*if ball_position.x > top.x && ball_position.y < top.y && ball_position.y > bottom.y {
            self.goals_scored += 1;
            return true;
        }*/

        false
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
