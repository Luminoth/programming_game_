use bevy::prelude::*;

use crate::bundles::actor::*;
use crate::components::physics::*;
use crate::components::projectile::*;
use crate::PROJECTILE_SORT;

#[derive(Debug, Default, Bundle)]
pub struct ProjectileBundle<T>
where
    T: Projectile,
{
    #[bundle]
    pub actor: ActorBundle,

    pub physical: Physical,

    pub projectile: T,
}

impl<T> ProjectileBundle<T>
where
    T: Projectile,
{
    pub fn spawn_at_position(commands: &mut Commands, position: Vec2, velocity: Vec2) -> Entity {
        info!(
            "spawning projectile '{}' at {} with velocity {}",
            T::name(),
            position,
            velocity
        );

        let mut bundle = commands.spawn_bundle(ProjectileBundle {
            actor: ActorBundle {
                name: Name::new(T::name()),
                transform: TransformBundle::from_transform(Transform::from_translation(
                    position.extend(PROJECTILE_SORT),
                )),
                ..Default::default()
            },
            physical: Physical {
                velocity,
                mass: T::mass(),
                ..Default::default()
            },
            projectile: T::default(),
        });

        bundle.with_children(|parent| {
            let mut model = parent.spawn();
            T::spawn_model(&mut model);
            model.insert(Name::new("Model"));
        });

        bundle.id()
    }
}
