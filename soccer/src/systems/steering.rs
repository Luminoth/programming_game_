use bevy::prelude::*;

use crate::components::physics::Physical;
use crate::components::steering::Steering;

pub fn update(mut query: Query<(&mut Steering, &mut Physical)>) {
    for (mut steering, mut physical) in query.iter_mut() {
        steering.update(&mut physical);
    }
}
