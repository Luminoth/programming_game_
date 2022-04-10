use bevy::prelude::*;

use crate::components::goal::*;
use crate::game::Team;

#[derive(Debug, Default, Bundle)]
pub struct GoalBundle {
    pub goal: Goal,
}

impl GoalBundle {
    pub fn spawn(commands: &mut Commands, position: Vec2, team: Team) -> Entity {
        info!("spawning goal for team {:?} at {}", team, position);

        let mut bundle = commands.spawn_bundle(GoalBundle {
            goal: Goal { team },
        });

        bundle.insert(Name::new(format!("{:?} Goal", team)));

        bundle.id()
    }
}
