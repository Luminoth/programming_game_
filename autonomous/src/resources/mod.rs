pub mod debug;
pub mod ui;

#[derive(Debug, Default)]
pub struct SimulationParams {
    pub window_border: f32,

    // vehicles
    pub vehicle_mass: f32,
    pub vehicle_max_steering_force: f32,
    pub vehicle_max_speed: f32,
    pub vehicle_max_turn_rate: f32,

    // obstacles
    pub num_obstacles: usize,
    pub min_obstacle_radius: f32,
    pub max_obstacle_radius: f32,
    pub min_gap_between_obstacles: f32,

    // steering weights
    pub seek_weight: f32,
    pub flee_weight: f32,
    pub arrive_weight: f32,
    pub evade_weight: f32,
    pub pursuit_weight: f32,
    pub wander_weight: f32,
    pub obstacle_avoidance_weight: f32,
    pub wall_avoidance_weight: f32,

    // obstacle avoidance
    pub min_detection_box_length: f32,

    // wall avoidance
    pub wall_detection_feeler_length: f32,
}
