use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::resources::*;
use crate::util::*;
use crate::DEBUG_SORT;

pub fn update(mut steering_behaviors: Query<SteeringQueryMut>) {
    for mut steering in steering_behaviors.iter_mut() {
        steering.steering.update(&mut steering.physical);
    }
}

pub fn update_debug(
    agents: Query<(&Steering, &Children), Without<SteeringTargetDebug>>,
    mut steering_debug: Query<TransformQueryMut, With<SteeringTargetDebug>>,
) {
    for (steering, children) in agents.iter() {
        for &child in children.iter() {
            if let Ok(mut transform) = steering_debug.get_mut(child) {
                transform.transform.set_world_translation(
                    transform.global_transform,
                    steering.target.extend(DEBUG_SORT),
                );
            }
        }
    }
}

pub fn update_seek(
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut seeking: Query<(SeekQueryMut, PhysicalQuery)>,
) {
    let params = params_assets.get(&params_asset.handle).unwrap();

    for (mut steering, physical) in seeking.iter_mut() {
        let force = steering.seek.force(&steering.steering, &physical);
        steering
            .steering
            .accumulate_force(physical.physical, force, params.seek_weight);
    }
}

pub fn update_arrive(
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut arriving: Query<(ArriveQueryMut, PhysicalQuery)>,
) {
    let params = params_assets.get(&params_asset.handle).unwrap();

    for (mut steering, physical) in arriving.iter_mut() {
        let force = steering.arrive.force(&steering.steering, &physical);
        steering
            .steering
            .accumulate_force(physical.physical, force, params.arrive_weight);
    }
}

pub fn update_pursuit(
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut pursuing: Query<(PursuitQueryMut, PhysicalQuery)>,
    physicals: Query<PhysicalQuery>,
) {
    let params = params_assets.get(&params_asset.handle).unwrap();

    for (mut steering, physical) in pursuing.iter_mut() {
        let force = steering
            .pursuit
            .force(params, &steering.steering, &physical, &physicals);
        steering
            .steering
            .accumulate_force(physical.physical, force, params.pursuit_weight);
    }
}

pub fn update_interpose(
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut interposing: Query<(InterposeQueryMut, PhysicalQuery)>,
    physicals: Query<PhysicalQuery>,
) {
    let params = params_assets.get(&params_asset.handle).unwrap();

    for (mut steering, physical) in interposing.iter_mut() {
        let force = steering
            .interpose
            .force(&steering.steering, &physical, &physicals);
        steering
            .steering
            .accumulate_force(physical.physical, force, params.interpose_weight);
    }
}

pub fn update_separation(
    params_asset: Res<SimulationParamsAsset>,
    params_assets: ResMut<Assets<SimulationParams>>,
    mut separating: Query<(Entity, SoccerPlayerSeparationQueryMut, PhysicalQuery)>,
    players: Query<(Entity, &Transform), With<SoccerPlayer>>,
) {
    let params = params_assets.get(&params_asset.handle).unwrap();

    for (entity, mut steering, physical) in separating.iter_mut() {
        let force = steering
            .separation
            .force(params, entity, physical.transform, players.iter());
        steering
            .steering
            .accumulate_force(physical.physical, force, params.separation_weight);
    }
}
