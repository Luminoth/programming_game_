use bevy::prelude::*;

use crate::components::physics::*;
use crate::components::steering::*;

pub fn update_steering<T>(query: Query<(&T, &mut Physical)>)
where
    T: Steering + Component,
{
}
