use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MessageEvent {
    HiHoneyImHome(Entity),
    StewIsReady(Entity),
}
