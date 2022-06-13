use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// Real-Time Collision Detection, Ericson

#[derive(Debug, Clone, Component, Inspectable)]
pub enum Bounds {
    Circle(Vec2, f32),
    Box(Vec2, Vec2),
}

impl Bounds {
    pub fn closest_point(&self, transform: &Transform, point: Vec2) -> Vec2 {
        let position = transform.translation.truncate();

        match self {
            Self::Circle(center, radius) => {
                let center = position + *center;

                let direction = (point - center).normalize_or_zero();
                direction * *radius
            }
            Self::Box(center, extents) => {
                let center = position + *center;

                let half_extents = *extents / 2.0;
                let min = center - half_extents;
                let max = center + half_extents;

                let x = point.x.max(min.x).min(max.x);
                let y = point.y.max(min.y).min(max.y);
                Vec2::new(x, y)
            }
        }
    }

    pub fn contains(&self, transform: &Transform, point: Vec2) -> bool {
        let position = transform.translation.truncate();

        match self {
            Self::Circle(center, radius) => {
                let center = position + *center;

                let d = center.distance_squared(point);
                d <= radius * radius
            }
            Self::Box(center, extents) => {
                let center = position + *center;

                (point.x - center.x).abs() * 2.0 <= extents.x
                    && (point.y - center.y).abs() * 2.0 <= extents.y
            }
        }
    }

    fn box_circle_intersects(
        circle_center: Vec2,
        circle_radius: f32,
        box_center: Vec2,
        box_extents: Vec2,
    ) -> bool {
        let box_half_extents = box_extents / 2.0;
        let box_min = box_center - box_half_extents;
        let box_max = box_center + box_half_extents;

        // point on the box nearest the circle
        let x = circle_center.x.max(box_min.x).min(box_max.x);
        let y = circle_center.y.max(box_min.y).min(box_max.y);
        let closest_point = Vec2::new(x, y);

        let d = circle_center.distance_squared(closest_point);
        d <= circle_radius * circle_radius
    }

    pub fn bounds_intersects(
        &self,
        transform: &Transform,
        other: &Bounds,
        other_transform: &Transform,
    ) -> bool {
        let position = transform.translation.truncate();
        let other_position = other_transform.translation.truncate();

        match self {
            Self::Circle(center, radius) => {
                let center = position + *center;
                match other {
                    Self::Circle(other_center, other_radius) => {
                        let other_center = other_position + *other_center;

                        let d = center.distance_squared(other_center);
                        let r = radius + other_radius;
                        d <= r * r
                    }
                    Self::Box(other_center, other_extents) => {
                        let other_center = other_position + *other_center;

                        Self::box_circle_intersects(center, *radius, other_center, *other_extents)
                    }
                }
            }
            Self::Box(center, extents) => {
                let center = position + *center;
                match other {
                    Self::Circle(other_center, other_radius) => {
                        let other_center = other_position + *other_center;

                        Self::box_circle_intersects(other_center, *other_radius, center, *extents)
                    }
                    Self::Box(other_center, other_extents) => {
                        let other_center = other_position + *other_center;

                        (center.x - other_center.x).abs() * 2.0 <= (extents.x + other_extents.x)
                            && (center.y - other_center.y).abs() * 2.0
                                <= (extents.y + other_extents.y)
                    }
                }
            }
        }
    }

    pub fn ray_intersects(
        &self,
        transform: &Transform,
        origin: Vec2,
        direction: Vec2,
        max_distance: f32,
    ) -> Option<Vec2> {
        let position = transform.translation.truncate();

        match self {
            Self::Circle(center, radius) => {
                let center = position + *center;

                let m = origin - center;
                let b = m.dot(direction);
                let c = m.dot(m) - radius * radius;
                if c > 0.0 && b > 0.0 {
                    return None;
                }

                let discr = b * b - c;
                if discr < 0.0 {
                    return None;
                }

                let mut t = -b - discr.sqrt();
                if t < 0.0 {
                    t = 0.0;
                }

                if t > max_distance {
                    return None;
                }

                Some(origin + direction * t)
            }
            Self::Box(center, extents) => {
                let center = position + *center;

                let half_extents = *extents / 2.0;
                let min = center - half_extents;
                let max = center + half_extents;

                let mut tmin = 0.0_f32;
                let mut tmax = f32::MAX;

                if direction.x.abs() < f32::EPSILON {
                    if origin.x < min.x || origin.x > max.x {
                        return None;
                    }
                } else {
                    let ood = 1.0 / direction.x;
                    let mut t1 = (min.x - origin.x) * ood;
                    let mut t2 = (max.x - origin.x) * ood;
                    if t1 > t2 {
                        std::mem::swap(&mut t1, &mut t2);
                    }

                    tmin = tmin.max(t1);
                    tmax = tmax.min(t2);
                    if tmin > tmax {
                        return None;
                    }
                }

                if direction.y.abs() < f32::EPSILON {
                    if origin.y < min.y || origin.y > max.y {
                        return None;
                    }
                } else {
                    let ood = 1.0 / direction.y;
                    let mut t1 = (min.y - origin.y) * ood;
                    let mut t2 = (max.y - origin.y) * ood;
                    if t1 > t2 {
                        std::mem::swap(&mut t1, &mut t2);
                    }

                    tmin = tmin.max(t1);
                    tmax = tmax.min(t2);
                    if tmin > tmax {
                        return None;
                    }
                }

                if tmin > max_distance {
                    return None;
                }

                Some(origin + direction * tmin)
            }
        }
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct BoundsQuery<'w> {
    pub bounds: &'w Bounds,
    pub transform: &'w Transform,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_contains_point_center() {
        let radius = 10.0;
        let center = Vec2::new(5.0, 5.0);
        let position = Vec2::new(5.0, 5.0);

        let transform = Transform::from_translation(position.extend(0.0));
        let bounds = Bounds::Circle(center, radius);

        let point = position + center;

        assert_eq!(bounds.contains(&transform, point), true);
    }

    #[test]
    fn circle_contains_point_edge() {
        let radius = 10.0;
        let center = Vec2::new(5.0, 5.0);
        let position = Vec2::new(5.0, 5.0);

        let transform = Transform::from_translation(position.extend(0.0));
        let bounds = Bounds::Circle(center, radius);

        let angle: f32 = 0.5;
        let point = position + center + Vec2::new(angle.cos(), angle.sin()) * radius;

        assert_eq!(bounds.contains(&transform, point), true);
    }

    #[test]
    fn circle_not_contains_point_edge() {
        let radius = 10.0;
        let center = Vec2::new(5.0, 5.0);
        let position = Vec2::new(5.0, 5.0);

        let transform = Transform::from_translation(position.extend(0.0));
        let bounds = Bounds::Circle(center, radius);

        let angle: f32 = 0.5;
        let point = position + center + Vec2::new(angle.cos(), angle.sin()) * (radius + 0.1);

        assert_eq!(bounds.contains(&transform, point), false);
    }

    #[test]
    fn box_contains_point_center() {
        let extents = Vec2::new(10.0, 10.0);
        let center = Vec2::new(5.0, 5.0);
        let position = Vec2::new(5.0, 5.0);

        let transform = Transform::from_translation(position.extend(0.0));
        let bounds = Bounds::Box(center, extents);

        let point = position + center;

        assert_eq!(bounds.contains(&transform, point), true);
    }

    #[test]
    fn box_contains_point_edge() {
        let extents = Vec2::new(10.0, 10.0);
        let center = Vec2::new(5.0, 5.0);
        let position = Vec2::new(5.0, 5.0);

        let transform = Transform::from_translation(position.extend(0.0));
        let bounds = Bounds::Box(center, extents);

        let half_extents = extents / 2.0;
        let point = position + center + half_extents;

        assert_eq!(bounds.contains(&transform, point), true);
    }

    #[test]
    fn box_not_contains_point_edge() {
        let extents = Vec2::new(10.0, 10.0);
        let center = Vec2::new(5.0, 5.0);
        let position = Vec2::new(5.0, 5.0);

        let transform = Transform::from_translation(position.extend(0.0));
        let bounds = Bounds::Box(center, extents);

        let mut half_extents = extents / 2.0;
        half_extents.x += 0.1;
        let point = position + center + half_extents;

        assert_eq!(bounds.contains(&transform, point), false);
    }

    // TODO: test intersections
}
