use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;
use crate::resources::*;

pub fn update(mut steering_behaviors: Query<SteeringQueryMut>) {
    for mut steering in steering_behaviors.iter_mut() {
        steering.steering.update(&mut steering.physical);
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
            .force(&params, &steering.steering, &physical, &physicals);
        steering
            .steering
            .accumulate_force(physical.physical, force, params.pursuit_weight);
    }
}
