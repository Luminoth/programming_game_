use bevy::prelude::*;

use crate::resources::SimulationParams;

#[derive(Debug, Default, Clone, Copy)]
pub struct PitchRegion {
    pub position: Vec2,
    pub extents: Vec2,
}

impl PitchRegion {
    pub fn is_inside(&self, position: Vec2) -> bool {
        position.x > self.left()
            && position.x < self.right()
            && position.y < self.top()
            && position.y > self.bottom()
    }

    pub fn is_inside_half(&self, position: Vec2) -> bool {
        let margin = self.extents * 0.25;

        position.x > self.left() + margin.x
            && position.x < self.right() - margin.x
            && position.y < self.top() + margin.y
            && position.y > self.bottom() - margin.y
    }

    pub fn left(&self) -> f32 {
        self.position.x - self.extents.x
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.extents.x
    }

    pub fn top(&self) -> f32 {
        self.position.y + self.extents.y
    }

    pub fn bottom(&self) -> f32 {
        self.position.y - self.extents.y
    }
}

#[derive(Debug)]
pub struct Pitch {
    pub extents: Vec2,
    pub regions: Vec<PitchRegion>,
}

impl Pitch {
    pub fn new(params: &SimulationParams) -> Self {
        let region_count = params.num_regions_horizontal * params.num_regions_vertical;
        let region_size = Vec2::new(
            params.pitch_extents.x / params.num_regions_horizontal as f32,
            params.pitch_extents.y / params.num_regions_vertical as f32,
        );

        info!(
            "preparing {} pitch regions of size {}",
            region_count, region_size
        );

        let hs = params.pitch_extents * 0.5;
        let hrs = region_size * 0.5;

        let mut regions = Vec::with_capacity(region_count);
        for x in 0..params.num_regions_horizontal {
            for y in 0..params.num_regions_vertical {
                let position = Vec2::new(
                    -hs.x + (x as f32 * region_size.x) + hrs.x,
                    -hs.y + (y as f32 * region_size.y) + hrs.y,
                );
                regions.push(PitchRegion {
                    position,
                    extents: region_size,
                });
            }
        }
        regions.reverse();

        Self {
            extents: params.pitch_extents,
            regions,
        }
    }
}
