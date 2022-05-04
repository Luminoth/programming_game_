pub mod debug;
pub mod messaging;
pub mod physics;
pub mod steering;
pub mod team;

use bevy::prelude::*;

use crate::components::physics::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Physics,

    // steering
    Steering,
    SteeringUpdatePhysics,

    // state machines
    GlobalStateExecute,
    StateExecute,
    GlobalStateOnMessage,
    StateEnter,
    StateExit,

    TeamStates,
    FieldPlayerEvents,
    FieldPlayerStates,
    GoalKeeperStates,

    TeamUpdate,
    FieldPlayerUpdate,
    GoalKeeperUpdate,
}

pub fn facing(_time: Res<Time>, mut query: Query<PhysicalQueryUpdateMut>) {
    for mut physical in query.iter_mut() {
        if physical.physical.heading.length_squared() < std::f32::EPSILON {
            continue;
        }

        let angle = -physical.physical.heading.angle_between(Vec2::Y);
        physical.transform.rotation = Quat::from_rotation_z(angle);
    }
}
