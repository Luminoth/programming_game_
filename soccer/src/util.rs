use bevy::ecs::query::{Fetch, FilterFetch, WorldQuery};
use bevy::ecs::system::QuerySingleError;
use bevy::math::Mat2;
use bevy::prelude::*;

pub fn point_to_world_space(point: Vec2, heading: Vec2, side: Vec2, position: Vec2) -> Vec2 {
    // rotate
    let mut transform = Mat3::from_cols(heading.extend(0.0), side.extend(0.0), Vec3::Z);

    // translate
    transform *= Mat3::from_translation(position);

    transform.transform_point2(point)
}

pub fn get_tangent_points(c: Vec2, r: f32, p: Vec2) -> Option<(Vec2, Vec2)> {
    let pmc = p - c;
    let sqrlen = pmc.length_squared();
    let rsqr = r * r;

    if sqrlen <= rsqr {
        return None;
    }

    let invsqrlen = 1.0 / sqrlen;
    let root = (sqrlen - rsqr).abs().sqrt();

    let t1 = Vec2::new(
        c.x + r * (r * pmc.x - pmc.y * root) * invsqrlen,
        c.y + r * (r * pmc.y + pmc.x * root) * invsqrlen,
    );
    let t2 = Vec2::new(
        c.x + r * (r * pmc.x + pmc.y * root) * invsqrlen,
        c.y + r * (r * pmc.y - pmc.x * root) * invsqrlen,
    );

    Some((t1, t2))
}

pub fn rotate_around_origin(v: Vec2, angle: f32) -> Vec2 {
    let transform = Mat2::from_angle(angle);
    transform.mul_vec2(v)
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
}

impl Vec2Utils for Vec2 {
    fn sign(&self, v2: Vec2) -> f32 {
        if self.y * v2.x > self.x * v2.y {
            -1.0
        } else {
            1.0
        }
    }
}

pub trait TransformUtils {
    fn set_world_translation(&mut self, global_transform: &GlobalTransform, world_position: Vec3);
}

impl TransformUtils for Transform {
    fn set_world_translation(&mut self, global_transform: &GlobalTransform, world_position: Vec3) {
        // TODO: this is exploding on floating point overflow :(

        /*info!(
            "before global: {}, local: {} to world_position: {}",
            global_transform.translation, self.translation, world_position
        );*/

        let parent_position = global_transform.translation - self.translation;
        let local_position = world_position - parent_position;
        self.translation = local_position;

        //info!("after local: {}", self.translation);
    }
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct TransformQueryMut<'w> {
    pub global_transform: &'w GlobalTransform,
    pub transform: &'w mut Transform,
}
