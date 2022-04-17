use crate::game::team::Team;

// TODO: this is not a bevy event and should probably live in the game module
// also the name isn't very good
#[derive(Debug, PartialEq, Eq)]
pub enum MessageEvent {
    GoHome(Team),
}
