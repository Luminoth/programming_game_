use bevy::prelude::*;

pub fn point_to_world_space(point: Vec2, heading: Vec2, side: Vec2, position: Vec2) -> Vec2 {
    // rotate
    let mut transform = Mat3::from_cols(heading.extend(0.0), side.extend(0.0), Vec3::Z);

    // translate
    transform *= Mat3::from_translation(position);

    transform.transform_point2(point)
}
