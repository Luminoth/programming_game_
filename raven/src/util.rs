use bevy::prelude::*;

// https://bevy-cheatbook.github.io/cookbook/cursor2world.html#2d-games
pub fn get_mouse_position(camera: (&Camera, &Transform), window: &Window) -> Option<Vec2> {
    if let Some(screen_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());
        let ndc = (screen_position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera.1.compute_matrix() * camera.0.projection_matrix.inverse();
        let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
        Some(world_position.truncate())
    } else {
        None
    }
}
