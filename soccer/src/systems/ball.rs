use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::resources::*;

pub fn update(mut ball: Query<&Ball>, walls: Query<WallQuery>) {
    let ball = ball.single_mut();

    ball.test_collision_with_walls(walls.iter());
}

pub fn update_physics(
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut ball: Query<PhysicalQueryMut, With<Ball>>,
) {
    let params = params_assets.get(&params_asset.handle).unwrap();

    let mut ball = ball.single_mut();

    // simulate friction
    if ball.physical.velocity.length_squared() > params.friction * params.friction {
        let direction = ball.physical.velocity.normalize();
        ball.physical.velocity += direction * -params.friction;
    }
}
