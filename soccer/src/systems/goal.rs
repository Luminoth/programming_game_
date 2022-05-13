use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::goal::*;

pub fn update(
    mut goals: Query<(&mut Goal, &Transform)>,
    ball_transform: Query<&Transform, With<Ball>>,
) {
    let ball_transform = ball_transform.single();

    for (mut goal, transform) in goals.iter_mut() {
        if goal.check_for_score(transform, ball_transform) {
            info!("GOOOOOOAAALLLL!!!!");
        }
    }
}
