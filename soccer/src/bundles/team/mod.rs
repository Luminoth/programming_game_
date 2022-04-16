mod field_player;
mod goalie;

use bevy::prelude::*;
//use bevy_prototype_lyon::prelude::*;

use crate::components::team::*;
use crate::game::{team::Team, GOALIE_PAD, PLAYER_RADIUS, PLAYER_SPREAD, TEAM_SPREAD};
use crate::resources::SimulationParams;

use field_player::*;
use goalie::*;

#[derive(Debug, Bundle)]
pub struct SoccerTeamBundle {
    pub team: SoccerTeam,
    pub state: SoccerTeamStateMachine,
    pub support_spots: SupportSpotCalculator,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
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

        let support_spots = SupportSpotCalculator::new(team, params);

        let mut bundle = commands.spawn_bundle(SoccerTeamBundle {
            team: SoccerTeam {
                team,
                ..Default::default()
            },
            state: SoccerTeamStateMachine::default(),
            support_spots: support_spots.clone(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        });

        bundle.insert(Name::new(format!("{:?} Team", team)));

        /*bundle.with_children(|parent| {
            for spot in &support_spots.spots {
                parent
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            radius: 5.0 + spot.score,
                            ..Default::default()
                        },
                        DrawMode::Fill(FillMode {
                            color: team.color(),
                            options: FillOptions::default(),
                        }),
                        Transform::from_translation(spot.position.extend(100.0)),
                    ))
                    .insert(SupportSpotDebug);
            }
        });*/

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
