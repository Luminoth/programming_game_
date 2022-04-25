use bevy::prelude::*;
use bevy_inspector_egui::*;

use crate::resources::SimulationParams;

#[derive(Debug, Default, Clone, Copy, Inspectable)]
pub struct PitchRegion {
    pub position: Vec2,
    pub extents: Vec2,
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct PitchRegionDebug;

#[derive(Debug, Component, Inspectable)]
pub struct Pitch {
    pub extents: Vec2,
    pub regions: Vec<PitchRegion>,
}

impl Pitch {
    pub fn new(params: &SimulationParams) -> Self {
        let hw = params.pitch_extents.x * 0.5;
        let hh = params.pitch_extents.y * 0.5;

        let region_count = params.num_regions_horizontal * params.num_regions_vertical;
        let region_size = Vec2::new(
            params.pitch_extents.x / params.num_regions_horizontal as f32,
            params.pitch_extents.y / params.num_regions_vertical as f32,
        );

        info!(
            "preparing {} pitch regions of size {}",
            region_count, region_size
        );

        let hs = region_size * 0.5;

        let mut regions = Vec::with_capacity(region_count);
        for y in 0..params.num_regions_vertical {
            for x in 0..params.num_regions_horizontal {
                let position = Vec2::new(
                    -hw + (x as f32 * region_size.x) + hs.x,
                    -hh + (y as f32 * region_size.y) + hs.y,
                );
                regions.push(PitchRegion {
                    position,
                    extents: Vec2::new(region_size.x, region_size.y),
                });
            }
        }

        Self {
            extents: params.pitch_extents,
            regions,
        }
    }
}

#[derive(Debug, Default, Component, Inspectable)]
pub struct PitchBorder;
