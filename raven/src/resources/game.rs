use crate::components::world::*;

#[derive(Debug)]
pub struct Map;

impl Map {
    pub fn get_random_spawnpoint<'a, S>(&self, spawnpoints: S) -> Option<SpawnPoint>
    where
        S: Iterator<Item = &'a SpawnPoint>,
    {
        todo!();
    }
}

#[derive(Debug)]
pub struct NavGraph;
