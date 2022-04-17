use super::team::Team;

#[derive(Debug, PartialEq, Eq)]
pub enum MessageEvent {
    GoHome(Team),
}
