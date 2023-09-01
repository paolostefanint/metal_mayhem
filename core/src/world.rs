use super::collisions::Body;
use super::game::GamePhase;
use super::player::{EntityType, Player};
use crate::config::CONFIG;
use crate::player::{PlayerHitBox, SpriteState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Direction {
    L,
    R,
}

impl Default for Direction {
    fn default() -> Self {
        return Direction::L;
    }
}

pub struct GameWorld {
    pub current_time: Instant,
    pub size: (f32, f32),
    pub entities: HashMap<u32, Box<dyn GameEntity + Send + 'static>>,
}

pub trait GameEntity {
    fn get_id(&self) -> u32;
    fn get_body(&self) -> Body;
    fn get_entity_type(&self) -> EntityType;
    fn tick(&mut self, delta_time: f32);
    // I obviously don't know what I'm doing here
    // Adding new entities means adding new methods to this trait
    // I don't want to do that
    // I want to be able to add new entities without changing this trait
    // I don't know how to do that
    // I feel dumb
    fn as_player_mut(&mut self) -> Option<&mut Player> {
        return None;
    }
    fn as_player(&self) -> Option<&Player> {
        return None;
    }
}

struct PlayerStats {
    id: u32,
    attack: f32,
    defense: f32,
}

impl GameWorld {
    pub fn new() -> GameWorld {
        let default_size = (20.0, 20.0);
        return GameWorld {
            current_time: Instant::now(),
            size: default_size,
            entities: HashMap::new(),
        };
    }

    pub fn setup(&mut self, world_size: (f32, f32)) {
        self.size = world_size;
    }

    pub fn get_players(&self) -> Vec<&Player> {
        let players = self
            .entities
            .values()
            .filter_map(|entity| match entity.get_entity_type() {
                EntityType::Player => Some(entity),
                _ => None,
            })
            .map(|entity| {
                let player = entity.as_player().unwrap();
                return player;
            })
            .collect::<Vec<&Player>>();

        return players;
    }
    pub fn get_players_mut(&mut self) -> Vec<&mut Player> {
        let players = self
            .entities
            .values_mut()
            .filter_map(|entity| match entity.get_entity_type() {
                EntityType::Player => Some(entity),
                _ => None,
            })
            .map(|entity| {
                let player = entity.as_player_mut().unwrap();
                return player;
            })
            .collect::<Vec<&mut Player>>();

        return players;
    }
    pub fn get_players_state(&self) -> Vec<PlayerState> {
        let players = self.get_players();
        let players = players.iter().map(|player| PlayerState {
            id: player.id.clone(),
            p: player.position.clone(),
            dir: player.direction.clone(),
            attack: player.input.attack,
            health: player.stats.health,
            sprite_state: player.sprite_state.clone(),
            damaged: player.is_taking_damage(),
        });
        return players.collect();
    }
    pub fn reset(&mut self) {
        self.entities.clear();
        self.current_time = Instant::now();
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub current_state: GamePhase,
    pub elapsed_time: f32,
    pub remaining_time: f32,
    pub players: Vec<PlayerState>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerState {
    id: u32,
    pub p: (f32, f32),
    pub dir: Direction,
    pub attack: bool,
    pub health: f32,
    pub sprite_state: SpriteState,
    pub damaged: bool,
}

impl GameWorld {
    pub fn add_entity(&mut self, entity: Box<dyn GameEntity + Send + 'static>) {
        self.entities.insert(entity.get_id(), entity);
    }
    pub fn update(&mut self) {
        let last_time = self.current_time;
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        // println!("delta_time: {}", delta_time
        self.current_time = current_time;

        for entity in self.entities.values_mut() {
            entity.tick(delta_time);
        }

        let mut hitboxes: Vec<PlayerHitBox> = vec![];

        let world_size = self.size.clone();
        let config = CONFIG.get().unwrap();

        for player in self.get_players_mut() {
            if player.input.attack {
                let hitbox_position = match player.direction {
                    Direction::L => (player.position.0 - config.hitbox.diff_x, player.position.1),
                    Direction::R => (player.position.0 + config.hitbox.diff_y, player.position.1),
                };
                let hitbox_size = (config.hitbox.size_x, config.hitbox.size_y);
                let hitbox = PlayerHitBox::new(player.get_id(), hitbox_size, hitbox_position);
                hitboxes.push(hitbox);
                player.sprite_state = SpriteState::Attack;
            }
            // check if player is out of bounds

            // check left
            if player.position.0 < 0.0 {
                player.position.0 = 0.0;
            }
            // check right
            if player.position.0 > world_size.0 {
                player.position.0 = world_size.0;
            }
            // check top
            if player.position.1 < 0.0 {
                player.position.1 = 0.0;
            }
            // check bottom
            if player.position.1 > world_size.1 {
                player.position.1 = world_size.1;
            }

            // if so, move them back in bounds
        }

        let mut players_stats: HashMap<u32, PlayerStats> = HashMap::new();
        for player in self.get_players() {
            let player_stats = PlayerStats {
                id: player.id,
                attack: player.stats.attack,
                defense: player.stats.defense,
            };
            players_stats.insert(player.id, player_stats);
        }

        for hitbox in hitboxes {
            for player in self.get_players_mut() {
                // check collisions on player
                if hitbox.player_id != player.id && hitbox.collides_with(&player) {
                    let damage = players_stats.get(&hitbox.player_id).unwrap().attack;
                    player.take_damage(damage);
                }
            }
        }
    }
}
