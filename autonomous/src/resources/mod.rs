pub mod debug;
pub mod ui;

#[derive(Debug, Default)]
pub struct SimulationParams {
    pub window_border: f32,

    // obstacles
    pub num_obstacles: usize,
    pub min_obstacle_radius: f32,
    pub max_obstacle_radius: f32,
    pub min_gap_between_obstacles: f32,

    // obstacle avoidance
    pub min_detection_box_length: f32,
}
