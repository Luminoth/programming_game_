mod field_player;
mod goal_keeper;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::team::*;
use crate::game::team::*;
use crate::resources::pitch::*;
use crate::resources::*;
use crate::{DEBUG_RADIUS, DEBUG_SORT};

use field_player::*;
use goal_keeper::*;

#[derive(Debug, Bundle)]
pub struct SoccerTeamBundle<T>
where
    T: TeamColorMarker,
{
    pub team: SoccerTeam,
    pub color: T,

    pub support_spots: SupportSpotCalculator,

    #[bundle]
    pub transform: TransformBundle,
}

impl<T> SoccerTeamBundle<T>
where
    T: TeamColorMarker,
{
    pub fn spawn(commands: &mut Commands, params: &SimulationParams, fonts: &Fonts, pitch: &Pitch) {
        let color = T::default();
        let team_color = color.team_color();

        info!("spawning team {:?}", team_color);

        let support_spots = SupportSpotCalculator::new(team_color, params);
        let debug_support_spots = if params.debug_vis {
            Some(support_spots.spots.clone())
        } else {
            None
        };

        let mut bundle = commands.spawn_bundle(SoccerTeamBundle {
            team: SoccerTeam::default(),
            color,
            support_spots,
            transform: TransformBundle::default(),
        });

        SoccerTeamStateMachine::insert(&mut bundle, SoccerTeamState::PrepareForKickOff, false);

        bundle.insert(Name::new(format!("{:?} Team", team_color)));

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
                                color: team_color.color(),
                                options: FillOptions::default(),
                            }),
                            Transform::from_translation(spot.position.extend(DEBUG_SORT)),
                        ))
                        .insert(SupportSpotDebug);
                }
            });
        }

        let (numbers, home_regions) = match team_color {
            TeamColor::Red => (RED_TEAM_NUMBERS, RED_TEAM_DEFENDING_HOME_REGIONS),
            TeamColor::Blue => (BLUE_TEAM_NUMBERS, BLUE_TEAM_DEFENDING_HOME_REGIONS),
        };

        // goal keeper
        GoalKeeperBundle::<T>::spawn(commands, params, fonts, pitch, numbers[0], home_regions[0]);

        // players
        for i in 1..TEAM_SIZE {
            FieldPlayerBundle::<T>::spawn(
                commands,
                params,
                fonts,
                pitch,
                if i % 2 == 0 {
                    FieldPlayerRole::Attacker
                } else {
                    FieldPlayerRole::Defender
                },
                numbers[i],
                home_regions[i],
            );
        }
    }
}
