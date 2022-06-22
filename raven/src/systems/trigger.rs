use bevy::prelude::*;

use crate::components::trigger::*;

pub fn update(time: Res<Time>, mut triggeres: Query<&mut Trigger>) {
    for mut trigger in triggeres.iter_mut() {
        trigger.update(time.delta_seconds());
    }
}
