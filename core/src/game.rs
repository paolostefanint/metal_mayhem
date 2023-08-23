use crate::player::Player;
use crate::world::GameWorld;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum GamePhase {
    WaitingForPlayers,
    RoundCountdown,
    Running,
    RoundEnd,
}

pub struct Game {
    pub phase: GamePhase,
    world: GameWorld,
}

impl Game {
    pub fn new(world_size: (f32, f32)) -> Game {
        Game {
            phase: GamePhase::WaitingForPlayers,
            world: GameWorld::new(world_size),
        }
    }

    pub fn start(&mut self) {
        self.phase = GamePhase::Running;
    }

    pub fn is_running(&self) -> bool {
        self.phase == GamePhase::Running
    }

    pub fn add_player(&mut self, player: Player) {
        self.world.add_entity(Box::new(player));
    }

    pub fn update(&mut self) {
        if self.phase == GamePhase::Running {
            self.world.update();
            println!("Game updated");
        } else {
            println!("Game is not running");
        }
    }

    pub fn get_world(&self) -> &GameWorld {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut GameWorld {
        &mut self.world
    }
}
