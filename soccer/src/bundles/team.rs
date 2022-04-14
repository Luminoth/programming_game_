use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::actor::*;
use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::game::{team::Team, GOALIE_PAD, PLAYER_RADIUS, PLAYER_SPREAD, TEAM_SPREAD};
use crate::resources::SimulationParams;
use crate::PLAYER_SORT;

use super::actor::*;

#[derive(Debug, Default, Bundle)]
pub struct SoccerTeamBundle {
    pub team: SoccerTeam,
    pub state: SoccerTeamStateMachine,
}

impl SoccerTeamBundle {
    pub fn spawn(commands: &mut Commands, params: &SimulationParams, team: Team) {
        info!("spawning team {:?}", team);

        let hw = params.pitch_extents.x * 0.5;
        let (field_center, goalie_position, sign) = match team {
            Team::Red => (
                Vec2::new(-TEAM_SPREAD, 0.0),
                Vec2::new(
                    -hw + params.goal_extents.x + PLAYER_RADIUS + GOALIE_PAD,
                    0.0,
                ),
                -1.0,
            ),
            Team::Blue => (
                Vec2::new(TEAM_SPREAD, 0.0),
                Vec2::new(hw - params.goal_extents.x - PLAYER_RADIUS - GOALIE_PAD, 0.0),
                1.0,
            ),
        };

        let mut bundle = commands.spawn_bundle(SoccerTeamBundle {
            team: SoccerTeam { team },
            ..Default::default()
        });

        bundle.insert(Name::new(format!("{:?} Team", team)));

        // TODO: we should treat the field position as the center
        // rather than the inner-most point

        // players
        FieldPlayerBundle::spawn(
            commands,
            field_center + Vec2::new(0.0, PLAYER_SPREAD.y),
            team,
        );
        FieldPlayerBundle::spawn(
            commands,
            field_center + Vec2::new(PLAYER_SPREAD.x * sign, PLAYER_SPREAD.y),
            team,
        );
        FieldPlayerBundle::spawn(
            commands,
            field_center + Vec2::new(0.0, -PLAYER_SPREAD.y),
            team,
        );
        FieldPlayerBundle::spawn(
            commands,
            field_center + Vec2::new(PLAYER_SPREAD.x * sign, -PLAYER_SPREAD.y),
            team,
        );

        // goalie
        GoalieBundle::spawn(commands, goalie_position + Vec2::ZERO, team);
    }
}

#[derive(Debug, Default, Bundle)]
struct FieldPlayerBundle {
    pub player: FieldPlayer,
    pub physical: Physical,
    pub state: FieldPlayerStateMachine,

    pub obstacle: Obstacle,
    pub obstacle_avoidance: ObstacleAvoidance,
}

impl FieldPlayerBundle {
    fn spawn(commands: &mut Commands, position: Vec2, team: Team) -> Entity {
        info!("spawning field player for team {:?} at {}", team, position);

        let mut bundle = commands.spawn_bundle(FieldPlayerBundle {
            player: FieldPlayer { team },
            ..Default::default()
        });

        bundle.insert(Name::new(format!("{:?} Field Player", team)));

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: PLAYER_RADIUS,
            },
            transform: Transform::from_translation(position.extend(PLAYER_SORT)),
            ..Default::default()
        });

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

#[derive(Debug, Default, Bundle)]
struct GoalieBundle {
    pub goalie: Goalie,
    pub physical: Physical,
    pub state: GoalieStateMachine,

    pub obstacle: Obstacle,
    pub obstacle_avoidance: ObstacleAvoidance,
}

impl GoalieBundle {
    fn spawn(commands: &mut Commands, position: Vec2, team: Team) -> Entity {
        info!("spawning goalie for team {:?} at {}", team, position);

        let mut bundle = commands.spawn_bundle(GoalieBundle {
            goalie: Goalie { team },
            ..Default::default()
        });

        bundle.insert(Name::new(format!("{:?} Goalie", team)));

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: PLAYER_RADIUS,
            },
            transform: Transform::from_translation(position.extend(PLAYER_SORT)),
            ..Default::default()
        });

        bundle.with_children(|parent| {
            parent
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: PLAYER_RADIUS,
                        ..Default::default()
                    },
                    DrawMode::Fill(FillMode {
                        color: Color::YELLOW,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}
