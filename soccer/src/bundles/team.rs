use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::bundles::actor::*;
use crate::components::actor::*;
use crate::components::team::*;
use crate::game::{Team, PLAYER_RADIUS};

#[derive(Debug, Default, Bundle)]
struct FieldPlayerBundle {
    player: FieldPlayer,
}

impl FieldPlayerBundle {
    fn spawn(commands: &mut Commands, position: Vec2, team: Team) -> Entity {
        info!("spawning field player for team {:?} at {}", team, position);

        let mut bundle = commands.spawn_bundle(FieldPlayerBundle {
            player: FieldPlayer { team },
        });

        bundle.insert(Name::new(format!("{:?} Field Player", team)));

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: PLAYER_RADIUS,
            },
            transform: Transform::from_translation(position.extend(0.0)),
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
    goalie: Goalie,
}

impl GoalieBundle {
    fn spawn(commands: &mut Commands, position: Vec2, team: Team) -> Entity {
        info!("spawning goalie for team {:?} at {}", team, position);

        let mut bundle = commands.spawn_bundle(GoalieBundle {
            goalie: Goalie { team },
        });

        bundle.insert(Name::new(format!("{:?} Goalie", team)));

        bundle.insert_bundle(ActorBundle {
            actor: Actor {
                bounding_radius: PLAYER_RADIUS,
            },
            transform: Transform::from_translation(position.extend(0.0)),
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
                        color: Color::DARK_GREEN,
                        options: FillOptions::default(),
                    }),
                    Transform::default(),
                ))
                .insert(Name::new("Model"));
        });

        bundle.id()
    }
}

pub fn spawn_team(
    commands: &mut Commands,
    field_position: Vec2,
    goalie_position: Vec2,
    team: Team,
) {
    let sign = match team {
        Team::Red => -1.0,
        Team::Blue => 1.0,
    };

    // players
    FieldPlayerBundle::spawn(commands, field_position + Vec2::new(0.0, 50.0), team);
    FieldPlayerBundle::spawn(
        commands,
        field_position + Vec2::new(100.0 * sign, 50.0),
        team,
    );
    FieldPlayerBundle::spawn(commands, field_position + Vec2::new(0.0, -50.0), team);
    FieldPlayerBundle::spawn(
        commands,
        field_position + Vec2::new(100.0 * sign, -50.0),
        team,
    );

    // goalie
    GoalieBundle::spawn(commands, goalie_position + Vec2::ZERO, team);
}
