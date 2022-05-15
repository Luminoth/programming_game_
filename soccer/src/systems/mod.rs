pub mod ball;
pub mod debug;
pub mod goal;
pub mod messaging;
pub mod physics;
pub mod steering;
pub mod team;

use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::physics::*;
use crate::components::team::*;
use crate::events::*;
use crate::game::team::*;
use crate::resources::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum Systems {
    Physics,

    // steering
    Steering,
    SteeringUpdatePhysics,

    // state machines
    GlobalStateExecute,
    StateExecute,
    GlobalStateOnMessage,
    StateEnter,
    StateExit,

    TeamStates,
    FieldPlayerEvents,
    FieldPlayerStates,
    GoalKeeperStates,

    TeamUpdate,
    FieldPlayerUpdate,
    GoalKeeperUpdate,

    GoalUpdate,
}

pub fn goal_scored_event_handler(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut events: EventReader<GoalScoredEvent>,
    mut teams: Query<(Entity, &mut SoccerTeamStateMachine)>,
    mut ball: Query<(&mut Physical, &mut Transform), With<Ball>>,
) {
    for event in events.iter() {
        info!("GOOOOOOAAALLLL!!!!");

        // re-center the ball
        let (mut physical, mut transform) = ball.single_mut();
        physical.teleport(&mut transform, Vec2::ZERO);

        // update the score
        match event.0 {
            TeamColor::Red => game_state.red_team_score += 1,
            TeamColor::Blue => game_state.blue_team_score += 1,
        }

        // prepare for kick off
        for (team, mut state_machine) in teams.iter_mut() {
            state_machine.change_state(&mut commands, team, SoccerTeamState::PrepareForKickOff);
        }
    }
}
