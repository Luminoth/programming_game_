use crate::game::state::State;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WifeState {
    DoHouseWork,
    VisitBathroom,
    CookStew,
}

impl Default for WifeState {
    fn default() -> Self {
        Self::DoHouseWork
    }
}

impl State for WifeState {}
