pub mod debug;
pub mod messaging;

use bevy::prelude::*;

use crate::game::team::*;

// this is mainly an event because calling find_support()
// pollutes systems with so many queries
// due to needing to potentially update
// the best supporting spot
pub struct FindSupportEvent(pub Entity);

pub struct GoalScoredEvent(pub TeamColor);
