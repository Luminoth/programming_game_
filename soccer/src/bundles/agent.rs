use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::agent::*;
use crate::components::steering::*;
use crate::resources::*;
use crate::{DEBUG_RADIUS, DEBUG_SORT};

#[derive(Debug, Default, Bundle)]
pub struct AgentBundle {
    pub agent: Agent,
    pub steering: Steering,
}

impl AgentBundle {
    pub fn insert(params: &SimulationParams, commands: &mut EntityCommands) {
        commands.insert_bundle(AgentBundle::default());

        Self::insert_debug(params, commands);
    }

    pub fn insert_with_separation(params: &SimulationParams, commands: &mut EntityCommands) {
        Self::insert(params, commands);

        Agent::separation_on(commands);
    }

    fn insert_debug(params: &SimulationParams, commands: &mut EntityCommands) {
        if params.debug_vis {
            commands.with_children(|parent| {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            radius: DEBUG_RADIUS,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: Color::BLACK,
                            options: FillOptions::default(),
                        }),
                        Transform::from_translation(Vec2::ZERO.extend(DEBUG_SORT)),
                    ))
                    .insert(SteeringTargetDebug);
            });
        }
    }
}
