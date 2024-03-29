use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::obstacle::*;
use crate::components::physics::*;
use crate::components::steering::*;
use crate::components::team::*;
use crate::game::PLAYER_RADIUS;
use crate::resources::pitch::*;
use crate::resources::ui::*;
use crate::resources::*;
use crate::{PLAYER_SORT, TEXT_SORT};

use super::super::actor::*;
use super::super::agent::*;

#[derive(Debug, Default, Bundle)]
pub struct GoalKeeperBundle<T>
where
    T: TeamColorMarker,
{
    pub player: SoccerPlayer,
    pub goal_keeper: GoalKeeper,
    pub team: T,

    pub physical: Physical,
    pub bounds: BoundingCircle,

    #[bundle]
    pub actor: ActorBundle,

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
        fonts: &Fonts,
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
            player: SoccerPlayer {
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
            bounds: BoundingCircle::from_radius(PLAYER_RADIUS),
            actor: ActorBundle {
                name: Name::new(format!("#{} {:?} Goal Keeper", number, team_color)),
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    position.extend(PLAYER_SORT),
                )),
                ..Default::default()
            },
            ..Default::default()
        });

        GoalKeeperStateMachine::insert(&mut bundle, GoalKeeperState::TendGoal, false);

        AgentBundle::insert(params, &mut bundle);

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

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::from_section(
                        format!("#{}", number),
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 18.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_alignment(TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    }),
                    transform: Transform::from_translation(Vec2::ZERO.extend(TEXT_SORT)),
                    ..Default::default()
                })
                .insert(Name::new("Number"));
        });

        bundle.id()
    }
}
