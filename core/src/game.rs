use crate::player::{Player, PlayerInputs};
use crate::world::GameWorld;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum GamePhase {
    WaitingForPlayers,
    RoundCountdown,
    Running,
    RoundEnd,
}

pub struct Game {
    pub phase: GamePhase,
    pub started_at: Option<Instant>,
    world: GameWorld,
}

const ROUND_DURATION: u64 = 10;

impl Game {
    pub fn new(world_size: (f32, f32)) -> Game {
        Game {
            phase: GamePhase::WaitingForPlayers,
            started_at: None,
            world: GameWorld::new(world_size),
        }
    }

    pub fn start(&mut self) {
        self.phase = GamePhase::Running;
        self.started_at = Some(Instant::now());
    }

    pub fn end(&mut self) {
        self.phase = GamePhase::RoundEnd;
        self.get_world_mut().reset();
    }

    pub fn is_running(&self) -> bool {
        self.phase == GamePhase::Running
    }

    pub fn add_player(&mut self, player: Player) {
        self.world.add_entity(Box::new(player));
    }

    pub fn update(&mut self) {
        if self.round_is_over() {
            self.end();
            return;
        }
        if self.phase == GamePhase::Running {
            self.world.update();
        } else {
            println!("Game is not running");
        }
    }

    pub fn round_is_over(&mut self) -> bool {
        match self.started_at {
            Some(started_at) => {
                let elapsed = started_at.elapsed();
                elapsed.as_secs() >= ROUND_DURATION
            }
            None => true,
        }
    }
    pub fn get_world(&self) -> &GameWorld {
        &self.world
    }

    pub fn get_world_mut(&mut self) -> &mut GameWorld {
        &mut self.world
    }

    pub fn handle_input(&mut self, player_id: u32, movement: (f32, f32), attack: bool) {
        let world = self.get_world_mut();
        let players = world.get_players_mut();

        for player in players {
            if player.id == player_id {
                player.input = PlayerInputs {
                    mov: movement,
                    attack,
                }
            }
        }
    }
}
