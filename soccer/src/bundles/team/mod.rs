mod field_player;
mod goal_keeper;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::SimulationParams;
use crate::{DEBUG_RADIUS, DEBUG_SORT};

use field_player::*;
use goal_keeper::*;

#[derive(Debug, Bundle)]
pub struct SoccerTeamBundle {
    pub team: SoccerTeam,
    pub support_spots: SupportSpotCalculator,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl SoccerTeamBundle {
    pub fn spawn(commands: &mut Commands, params: &SimulationParams, team: Team, pitch: &Pitch) {
        info!("spawning team {:?}", team);

        let support_spots = SupportSpotCalculator::new(team, params);
        let debug_support_spots = if params.debug_vis {
            Some(support_spots.spots.clone())
        } else {
            None
        };

        let mut bundle = commands.spawn_bundle(SoccerTeamBundle {
            team: SoccerTeam::new(team),
            support_spots,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        });

        SoccerTeamStateMachine::insert(&mut bundle, SoccerTeamState::PrepareForKickOff, false);

        bundle.insert(Name::new(format!("{:?} Team", team)));

        if params.debug_vis {
            bundle.with_children(|parent| {
                for spot in debug_support_spots.unwrap() {
                    parent
                        .spawn_bundle(GeometryBuilder::build_as(
                            &shapes::Circle {
                                radius: DEBUG_RADIUS + (spot.score * DEBUG_RADIUS),
                                ..Default::default()
                            },
                            DrawMode::Fill(FillMode {
                                color: team.color(),
                                options: FillOptions::default(),
                            }),
                            Transform::from_translation(spot.position.extend(DEBUG_SORT)),
                        ))
                        .insert(SupportSpotDebug);
                }
            });
        }

        let home_regions = match team {
            Team::Red => RED_TEAM_DEFENDING_HOME_REGIONS,
            Team::Blue => BLUE_TEAM_DEFENDING_HOME_REGIONS,
        };

        // goal keeper
        GoalKeeperBundle::spawn(commands, home_regions[0], team, pitch);

        // players
        for home_region in home_regions.iter().take(TEAM_SIZE).skip(1) {
            FieldPlayerBundle::spawn(commands, *home_region, team, pitch);
        }
    }
}
