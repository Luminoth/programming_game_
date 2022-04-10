use bevy::prelude::*;

use crate::components::team::*;
use crate::game::Team;

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

        bundle.id()
    }
}

pub fn spawn_team(
    commands: &mut Commands,
    field_position: Vec2,
    goalie_position: Vec2,
    team: Team,
) {
    // players
    FieldPlayerBundle::spawn(commands, field_position + Vec2::ZERO, team);
    FieldPlayerBundle::spawn(commands, field_position + Vec2::ZERO, team);
    FieldPlayerBundle::spawn(commands, field_position + Vec2::ZERO, team);
    FieldPlayerBundle::spawn(commands, field_position + Vec2::ZERO, team);

    // goalie
    GoalieBundle::spawn(commands, goalie_position + Vec2::ZERO, team);
}
