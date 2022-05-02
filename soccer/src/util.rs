use bevy::ecs::query::{Fetch, FilterFetch, WorldQuery};
use bevy::ecs::system::QuerySingleError;
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
