use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::actor::*;
use crate::components::agent::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::game::PLAYER_RADIUS;
use crate::resources::pitch::*;
use crate::resources::*;
use crate::PLAYER_SORT;

use super::super::actor::*;

#[derive(Debug, Default, Bundle)]
pub struct GoalKeeperBundle<T>
where
    T: TeamColorMarker,
{
    pub goal_keeper: GoalKeeper,
    pub team: T,

    pub agent: Agent,
    pub steering: Steering,
    pub physical: Physical,

    pub obstacle: Obstacle,
    pub obstacle_avoidance: ObstacleAvoidance,
}

impl<T> GoalKeeperBundle<T>
where
    T: TeamColorMarker,
{
    pub fn spawn(
        commands: &mut Commands,
        params: &SimulationParams,
        pitch: &Pitch,
        number: usize,
        home_region: usize,
    ) -> Entity {
        let position = pitch.regions.get(home_region).unwrap().position;

        let team = T::default();
        let team_color = team.team_color();

        info!(
            "spawning goal keeper #{} for team {:?} at {} (home region: {})",
            number, team_color, position, home_region
        );

        let mut bundle = commands.spawn_bundle(GoalKeeperBundle {
            goal_keeper: GoalKeeper {
                number,
                home_region,
                default_region: home_region,
            },
            team,
            physical: Physical {
                mass: params.player_mass,
                max_speed: params.player_max_speed_without_ball,
                max_force: params.player_max_force,
                max_turn_rate: params.player_max_turn_rate,
                ..Default::default()
            },
            ..Default::default()
        });

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: PLAYER_RADIUS,
            },
            transform: Transform::from_translation(position.extend(PLAYER_SORT)),
            name: Name::new(format!("#{} {:?} Goal Keeper", number, team_color)),
            ..Default::default()
        });

        GoalKeeperStateMachine::insert(&mut bundle, GoalKeeperState::TendGoal, false);

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: PLAYER_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: team_color.goal_keeper_color(),
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
