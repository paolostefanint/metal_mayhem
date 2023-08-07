use super::world::GameWorld;
use std::time::Instant;

pub fn setup_world(world_size: (f32, f32)) -> GameWorld {
    let mut world = GameWorld {
        current_time: Instant::now(),
        size: world_size,
        players: Vec::new(),
    };
    return world;
}
