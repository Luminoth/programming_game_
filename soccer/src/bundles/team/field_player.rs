use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::actor::*;
use crate::components::agent::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::game::{team::Team, PLAYER_RADIUS};
use crate::resources::pitch::*;
use crate::PLAYER_SORT;

use super::super::actor::*;

#[derive(Debug, Default, Bundle)]
pub struct FieldPlayerBundle {
    pub player: FieldPlayer,
    pub agent: Agent,
    pub physical: Physical,
    pub steering: Steering,

    pub obstacle: Obstacle,
    pub obstacle_avoidance: ObstacleAvoidance,
}

impl FieldPlayerBundle {
    pub fn spawn(commands: &mut Commands, home_region: usize, team: Team, pitch: &Pitch) -> Entity {
        let position = pitch.regions.get(home_region).unwrap().position;

        info!(
            "spawning field player for team {:?} at {} (home region: {})",
            team, position, home_region
        );

        let mut bundle = commands.spawn_bundle(FieldPlayerBundle {
            player: FieldPlayer {
                team,
                home_region,
                default_region: home_region,
            },
            ..Default::default()
        });

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: PLAYER_RADIUS,
            },
            transform: Transform::from_translation(position.extend(PLAYER_SORT)),
            name: Name::new(format!("{:?} Field Player", team)),
            ..Default::default()
        });

        FieldPlayerStateMachine::insert(&mut bundle, FieldPlayerState::Wait, false);

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: PLAYER_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: team.color(),
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
