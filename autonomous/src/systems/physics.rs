use bevy::prelude::*;

use crate::components::physics::Physical;

pub fn update(query: Query<(&mut Physical, &mut Transform)>) {}
