use bevy::prelude::*;

pub fn point_to_world_space(point: Vec2, heading: Vec2, side: Vec2, position: Vec2) -> Vec2 {
    // rotate
    let mut transform = Mat3::from_cols(heading.extend(0.0), side.extend(0.0), Vec3::Z);

    // translate
    transform *= Mat3::from_translation(position);

    transform.transform_point2(point)
}

pub fn vector_to_world_space(vector: Vec2, heading: Vec2, side: Vec2) -> Vec2 {
    // rotate
    let transform = Mat3::from_cols(heading.extend(0.0), side.extend(0.0), Vec3::Z);

    transform.transform_vector2(vector)
}

pub fn point_to_local_space(point: Vec2, heading: Vec2, side: Vec2, position: Vec2) -> Vec2 {
    let tx = -position.dot(heading);
    let ty = -position.dot(side);

    let transform = Mat3::from_cols(
        Vec3::new(heading.x, side.x, 0.0),
        Vec3::new(heading.y, side.y, 0.0),
        Vec3::new(tx, ty, 0.0),
    );

    transform.transform_point2(point)
}

/*pub fn vector_to_local_space(vector: Vec2, heading: Vec2, side: Vec2) -> Vec2 {
    let transform = Mat3::from_cols(
        Vec3::new(heading.x, side.x, 0.0),
        Vec3::new(heading.y, side.y, 0.0),
        Vec3::Z,
    );

    transform.transform_vector2(vector)
}*/

pub fn circles_overlap(apos: Vec2, aradius: f32, bpos: Vec2, bradius: f32) -> bool {
    let dist_between_centers =
        ((apos.x - bpos.x) * (apos.x - bpos.x) + (apos.y - bpos.y) * (apos.y - bpos.y)).sqrt();

    dist_between_centers < (aradius + bradius) || dist_between_centers < (aradius - bradius).abs()
}

pub fn overlapped(
    position: Vec2,
    radius: f32,
    others: impl AsRef<[(Vec2, f32)]>,
    min_dist_between: f32,
) -> bool {
    for (other_position, other_radius) in others.as_ref() {
        if circles_overlap(
            position,
            radius + min_dist_between,
            *other_position,
            *other_radius,
        ) {
            return true;
        }
    }
    false
}
