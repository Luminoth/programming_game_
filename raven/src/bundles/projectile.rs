use bevy::prelude::*;

use crate::bundles::actor::*;
use crate::components::physics::*;
use crate::components::projectile::*;
use crate::components::*;
use crate::game::PROJECTILE_SORT;

#[derive(Debug, Bundle)]
pub struct ProjectileBundle {
    #[bundle]
    pub actor: ActorBundle,

    pub physical: Physical,

    pub projectile: Projectile,
}

impl ProjectileBundle {
    pub fn spawn(
        commands: &mut Commands,
        projectile: Projectile,
        position: Vec2,
        direction: Vec2,
    ) -> Entity {
        let velocity = direction * projectile.get_initial_speed();
        info!(
            "spawning projectile '{}' at {} with velocity {}",
            projectile.get_name(),
            position,
            velocity
        );

        let mut bundle = commands.spawn_bundle(ProjectileBundle {
            actor: ActorBundle {
                name: Name::new(projectile.get_name()),
                transform: TransformBundle::from_transform(Transform::from_translation(
                    position.extend(PROJECTILE_SORT),
                )),
                ..Default::default()
            },
            physical: Physical {
                velocity,
                mass: projectile.get_mass(),
                ..Default::default()
            },
            projectile: projectile.clone(),
        });

        bundle.with_children(|parent| {
            let mut model = parent.spawn();
            projectile.spawn_model(&mut model);
            model.insert(Model).insert(Name::new("Model"));
        });

        bundle.id()
    }
}
