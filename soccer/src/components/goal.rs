use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::components::physics::*;
use crate::components::team::*;

#[derive(Debug, Default, Component, Inspectable)]
pub struct Goal {
    pub facing: Vec2,

    // scoring offsets
    pub top: Vec2,
    pub bottom: Vec2,
    pub score_center: Vec2,
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

    pub fn check_for_score(
        &self,
        transform: &Transform,
        bounds: &BoundingRect,
        ball_transform: &Transform,
    ) -> bool {
        let ball_position = ball_transform.translation.truncate();
        bounds.contains(transform, ball_position)
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct GoalDebug;

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct TeamGoalQuery<T>
where
    T: TeamColorMarker,
{
    pub goal: &'static Goal,
    pub team: &'static T,

    pub transform: &'static Transform,
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct GoalQuery {
    pub goal: &'static Goal,
    pub transform: &'static Transform,
}
