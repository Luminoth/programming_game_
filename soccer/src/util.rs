use bevy::ecs::query::{QueryItem, QuerySingleError, ROQueryItem, WorldQuery};
use bevy::math::Mat2;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

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
    fn optional_single(&self) -> Option<ROQueryItem<'_, Q>>;
    fn optional_single_mut(&mut self) -> Option<QueryItem<'_, Q>>;
}

impl<'w, 's, Q, F> OptionalSingle<'s, Q> for Query<'w, 's, Q, F>
where
    Q: WorldQuery,
    F: WorldQuery,
{
    fn optional_single(&self) -> Option<ROQueryItem<'_, Q>> {
        match self.get_single() {
            Ok(item) => Some(item),
            Err(QuerySingleError::NoEntities(_)) => None,
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("multiple items from optional single query")
            }
        }
    }

    fn optional_single_mut(&mut self) -> Option<QueryItem<'_, Q>> {
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

#[derive(Debug, Inspectable)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn contains(&self, transform: &Transform, point: Vec2) -> bool {
        let center = transform.translation.truncate();

        let left = center.x + self.left;
        let right = center.x + self.right;
        let bottom = center.y + self.bottom;
        let top = center.y + self.top;

        point.x > left && point.x < right && point.y > bottom && point.y < top
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

        let parent_position = global_transform.translation() - self.translation;
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
pub struct TransformQueryMut {
    pub global_transform: &'static GlobalTransform,
    pub transform: &'static mut Transform,
}
