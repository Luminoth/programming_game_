use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Clone, Component, Inspectable)]
pub enum Bounds {
    Circle(Vec2, f32),
    Box(Vec2, Vec2),
}

impl Bounds {
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
                let half_extents = *extents / 2.0;

                let min = center - half_extents;
                let max = center + half_extents;

                point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
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
        let nearest = Vec2::new(
            box_min.x.max(circle_center.x.min(box_max.x)),
            box_min.y.max(circle_center.y.min(box_max.y)),
        );

        let d = circle_center.distance_squared(nearest);
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
                        let half_extents = *extents / 2.0;
                        let min = center - half_extents;
                        let max = center + half_extents;

                        let other_center = other_position + *other_center;
                        let other_half_extents = *other_extents / 2.0;
                        let other_min = other_center - other_half_extents;
                        let other_max = other_center + other_half_extents;

                        other_min.x <= max.x
                            && other_max.x >= min.x
                            && other_max.y >= min.y
                            && other_min.y <= max.y
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
    ) -> bool {
        let position = transform.translation.truncate();

        match self {
            Self::Circle(center, radius) => {
                let center = position + *center;

                todo!();
            }
            Self::Box(center, extents) => {
                // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection

                let center = position + *center;
                let half_extents = *extents / 2.0;

                let min = center - half_extents;
                let max = center + half_extents;

                let invdir = 1.0 / direction;

                let (tmin, tmax) = if invdir.x >= 0.0 {
                    ((min.x - origin.x) * invdir.x, (max.x - origin.x) * invdir.x)
                } else {
                    ((max.x - origin.x) * invdir.x, (min.x - origin.x) * invdir.x)
                };

                let (tymin, tymax) = if invdir.x >= 0.0 {
                    ((min.y - origin.y) * invdir.y, (max.y - origin.y) * invdir.y)
                } else {
                    ((max.y - origin.y) * invdir.y, (min.y - origin.y) * invdir.y)
                };

                if (tmin > tymax) || (tymin > tmax) {
                    return false;
                }

                let dx = tmax - tmin;
                let dy = tymax - tymin;
                let d = dx * dx + dy * dy;
                d <= max_distance * max_distance
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
