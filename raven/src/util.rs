use bevy::ecs::query::{Fetch, FilterFetch, WorldQuery};
use bevy::ecs::system::QuerySingleError;
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

pub trait OptionalSingle<'s, Q>
where
    Q: WorldQuery,
{
    fn optional_single(&self) -> Option<<Q::ReadOnlyFetch as Fetch<'_, 's>>::Item>;
    fn optional_single_mut(&mut self) -> Option<<Q::Fetch as Fetch<'_, '_>>::Item>;
}

impl<'w, 's, Q, F> OptionalSingle<'s, Q> for Query<'w, 's, Q, F>
where
    Q: WorldQuery,
    F: WorldQuery,
    F::Fetch: FilterFetch,
{
    fn optional_single(&self) -> Option<<Q::ReadOnlyFetch as Fetch<'_, 's>>::Item> {
        match self.get_single() {
            Ok(item) => Some(item),
            Err(QuerySingleError::NoEntities(_)) => None,
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("multiple items from optional single query")
            }
        }
    }

    fn optional_single_mut(&mut self) -> Option<<Q::Fetch as Fetch<'_, '_>>::Item> {
        match self.get_single_mut() {
            Ok(item) => Some(item),
            Err(QuerySingleError::NoEntities(_)) => None,
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("multiple items from optional single query")
            }
        }
    }
}

pub trait Vec2Utils {
    fn sign(&self, v2: Vec2) -> f32;

    fn rotate(&self, angle: f32) -> Vec2;
}

impl Vec2Utils for Vec2 {
    fn sign(&self, v2: Vec2) -> f32 {
        if self.y * v2.x > self.x * v2.y {
            -1.0
        } else {
            1.0
        }
    }

    fn rotate(&self, angle: f32) -> Vec2 {
        let v = self.extend(0.0);
        (Quat::from_rotation_z(angle) * v).truncate()
    }
}

pub trait TransformUtils {
    fn set_world_translation(&mut self, global_transform: &GlobalTransform, world_position: Vec3);
}

impl TransformUtils for Transform {
    fn set_world_translation(&mut self, global_transform: &GlobalTransform, world_position: Vec3) {
        /*println!(
            "before global: {}, local: {} to world_position: {}",
            global_transform.translation, self.translation, world_position
        );*/

        let parent_position = global_transform.translation - self.translation;
        //println!("parent: {}", parent_position);

        let local_position = world_position - parent_position;
        if self.translation.distance_squared(local_position) > f32::EPSILON * f32::EPSILON {
            self.translation = local_position;
        }
        //println!("after local: {}", self.translation);
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct TransformQueryMut<'w> {
    pub global_transform: &'w GlobalTransform,
    pub transform: &'w mut Transform,
}
