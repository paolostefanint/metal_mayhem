use super::player::Player;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize)]
pub enum Direction {
    L,
    R,
}

pub struct GameWorld {
    pub current_time: Instant,
    pub size: (f32, f32),
    pub players: Vec<Player>,
}

impl GameWorld {
    pub fn get_players_state(&self) -> Vec<PlayerState> {
        let players = self.players.iter().map(|player| PlayerState {
            id: player.id.clone(),
            p: player.position.clone(),
            dir: Direction::L,
        });
        return players.collect();
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub current_time: f32,
    pub current_state: String,
    pub players: Vec<PlayerState>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerState {
    id: u8,
    pub p: (f32, f32),
    pub dir: Direction,
}

impl GameWorld {}

pub fn start_world_loop(world_arc: Arc<Mutex<GameWorld>>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            {
                let mut world = world_arc.lock().unwrap();

                let last_time = world.current_time;
                let current_time = Instant::now();
                let delta_time = current_time.duration_since(last_time).as_secs_f32();
                // println!("delta_time: {}", delta_time);
                world.current_time = current_time;

                for player in world.players.iter_mut() {
                    player.tick(delta_time);
                }
            }
        }
    })
}
