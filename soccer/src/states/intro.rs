use bevy::prelude::*;

use crate::components::camera::*;
use crate::resources::ui::*;

use super::*;

pub fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    // cameras
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(UiCamera)
        .insert(Name::new("UI Camera"));

    let root = spawn_ui_root(&mut commands);
    commands.entity(root).with_children(|parent| {
        spawn_button(parent, &fonts, "Run");
    });
}

pub fn button_handler(
    mut action_query: Query<(&Interaction, With<Button>), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok((interaction, _)) = action_query.get_single_mut() {
        if *interaction == Interaction::Clicked {
            state.set(GameState::Main).unwrap();
        }
    }
}

pub fn teardown(mut commands: Commands, entities: Query<Entity>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
