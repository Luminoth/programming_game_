use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::game::REGIONS;

#[derive(Debug, Default, Inspectable)]
pub struct Region {
    // NOTE: region origin is top left
    pub position: Vec2,
    pub extents: Vec2,
}

impl Region {
    fn new(position: Vec2, extents: Vec2) -> Self {
        Self { position, extents }
    }
}

#[derive(Debug, Component, Inspectable)]
pub struct Pitch {
    pub extents: Vec2,
    pub regions: Vec<Region>,
}

impl Pitch {
    pub fn new(extents: Vec2) -> Self {
        let hw = extents.x * 0.5;
        let hh = extents.y * 0.5;

        let region_count = (REGIONS.x * REGIONS.y) as usize;
        let region_size = Vec2::new(extents.x / REGIONS.x as f32, extents.y / REGIONS.y as f32);

        info!(
            "preparing {} pitch regions of size {}",
            region_count, region_size
        );

        let mut regions = Vec::with_capacity(region_count);
        for y in 0..REGIONS.y {
            for x in 0..REGIONS.x {
                let position = Vec2::new(
                    -hw + (x as f32 * region_size.x),
                    -hh + (y as f32 * region_size.y),
                );
                regions.push(Region::new(
                    position,
                    Vec2::new(region_size.x, region_size.y),
                ));
            }
        }

        Self { extents, regions }
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct PitchBorder;
