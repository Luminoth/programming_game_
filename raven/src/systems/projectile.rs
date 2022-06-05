use bevy::prelude::*;

use crate::components::projectile::*;
use crate::ORTHO_SIZE;

pub fn check_bounds(
    mut commands: Commands,
    windows: Res<Windows>,
    projectiles: Query<(Entity, &Transform, &Name), With<Projectile>>,
) {
    let window = windows.get_primary().unwrap();
    let aspect_ratio = window.width() / window.height();

    for (entity, transform, name) in projectiles.iter() {
        let max_x = ORTHO_SIZE;
        let max_y = ORTHO_SIZE / aspect_ratio;

        let position = transform.translation.truncate();
        if position.x < -max_x || position.x > max_x || position.y < -max_y || position.y > max_y {
            info!("despawning projectile '{}'", name);
            commands.entity(entity).despawn_recursive();
        }
    }
}
