use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Component)]
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

#[derive(Debug, Default, Component)]
pub struct Wife;

impl Wife {
    pub fn spawn(commands: &mut Commands, name: impl Into<String>) {
        let name = name.into();
        info!("spawning wife {}", name);

        commands
            .spawn()
            .insert(Wife::default())
            .insert(WifeState::default())
            .insert(Name::new(name));
    }
}
