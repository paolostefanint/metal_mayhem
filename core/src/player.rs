use super::collisions::{Body, BodyType, AABB};
use super::world::Direction;
use crate::config::CONFIG;
use std::time::Instant;

use crate::world::GameEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SpriteState {
    Idle,
    Walk,
    Attack,
    Damage,
    Dead,
}
impl Default for SpriteState {
    fn default() -> Self {
        SpriteState::Idle
    }
}

pub enum EntityType {
    Player,
    PlayerHitBox,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerStats {
    pub attack: f32,
    pub defense: f32,
    pub max_speed: f32,
    pub health: f32,
    pub cooldown: f32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Player {
    pub id: u32,
    pub stats: PlayerStats,
    pub avatar: Avatar,
    pub size: (f32, f32),
    pub position: (f32, f32),
    pub input: PlayerInputs,
    pub direction: Direction,
    pub sprite_state: SpriteState,
    pub last_damage_time: Option<Instant>,
    pub last_attack_time: Option<Instant>,
}

#[derive(Debug, Copy, Clone)]
pub enum Avatar {
    Tizia,
    Gundam,
    Coso,
}

impl Default for Avatar {
    fn default() -> Self {
        Avatar::Tizia
    }
}

impl Player {
    pub fn new(player_configuration: &PlayerConfiguration) -> Player {
        let avatar = match player_configuration.avatar {
            1 => Avatar::Tizia,
            2 => Avatar::Gundam,
            3 => Avatar::Coso,
            _ => Avatar::Tizia,
        };

        let config = CONFIG.get().unwrap();

        let stats = match avatar {
            Avatar::Tizia => PlayerStats {
                attack: config.tizia.attack,
                defense: config.tizia.defense,
                max_speed: config.tizia.speed,
                health: config.tizia.health,
                cooldown: config.tizia.cooldown,
            },
            Avatar::Gundam => PlayerStats {
                attack: config.gundam.attack,
                defense: config.gundam.defense,
                max_speed: config.gundam.speed,
                health: config.gundam.health,
                cooldown: config.gundam.cooldown,
            },
            Avatar::Coso => PlayerStats {
                attack: config.coso.attack,
                defense: config.coso.defense,
                max_speed: config.coso.speed,
                health: config.coso.health,
                cooldown: config.coso.cooldown,
            },
        };

        Player {
            id: player_configuration.player_id,
            stats,
            avatar,
            size: config.player_size,
            position: player_configuration.initial_position,
            direction: Direction::R,
            input: PlayerInputs::new(),
            sprite_state: SpriteState::Idle,
            last_damage_time: None,
            last_attack_time: None,
        }
    }
    pub fn take_damage(&mut self, damage: f32) {
        // go to max(0, health - damage)
        self.stats.health = (self.stats.health - damage).max(0.0);
        self.sprite_state = SpriteState::Damage;
        self.last_damage_time = Some(Instant::now());
    }
    pub fn is_taking_damage(&self) -> bool {
        match self.last_damage_time {
            Some(_) => true,
            None => false,
        }
    }
}

impl GameEntity for Player {
    fn as_player_mut(&mut self) -> Option<&mut Player> {
        return Some(self);
    }
    fn as_player(&self) -> Option<&Player> {
        return Some(self);
    }
    fn get_id(&self) -> u32 {
        return self.id as u32;
    }
    fn get_entity_type(&self) -> EntityType {
        return EntityType::Player;
    }
    fn get_body(&self) -> Body {
        let aabb = AABB::new(
            (
                self.position.0 - self.size.0 / 2.0,
                self.position.1 - self.size.1 / 2.0,
            ),
            (
                self.position.0 + self.size.0 / 2.0,
                self.position.1 + self.size.1 / 2.0,
            ),
        );
        return Body {
            aabb: aabb,
            body_type: BodyType::Dynamic,
        };
    }
    fn tick(&mut self, delta_time: f32) {
        let (x, y) = self.position;
        let (mov_x, mov_y) = self.input.mov;
        let speed = self.stats.max_speed;

        self.direction = if mov_x > 0.0 {
            Direction::R
        } else if mov_x < 0.0 {
            Direction::L
        } else {
            self.direction
        };

        self.position = (
            x + mov_x * speed * delta_time,
            y + mov_y * speed * delta_time,
        );

        self.sprite_state = if mov_x != 0.0 || mov_y != 0.0 {
            SpriteState::Walk
        } else {
            SpriteState::Idle
        };

        let config = CONFIG.get().unwrap();

        match self.last_damage_time {
            Some(last_damage_time) => {
                let now = Instant::now();
                let elapsed = now.duration_since(last_damage_time);
                if elapsed.as_secs_f32() > config.damage_duration {
                    self.last_damage_time = None;
                }
            }
            None => {}
        }
    }
}

pub struct PlayerHitBox {
    pub id: u32,
    pub player_id: u32,
    pub size: (f32, f32),
    pub position: (f32, f32),
}

impl PlayerHitBox {
    pub fn new(player_id: u32, size: (f32, f32), position: (f32, f32)) -> PlayerHitBox {
        PlayerHitBox {
            id: 0,
            player_id,
            size,
            position,
        }
    }
    pub fn collides_with(&self, player: &Player) -> bool {
        let player_body = player.get_body();
        let hitbox_body = self.get_body();
        // println!("player_body: {:?}", player_body);
        // println!("hitbox_body: {:?}", hitbox_body);

        return hitbox_body.aabb.intersects(&player_body.aabb);
    }
}

impl GameEntity for PlayerHitBox {
    fn get_entity_type(&self) -> EntityType {
        EntityType::PlayerHitBox
    }
    fn get_id(&self) -> u32 {
        return self.id;
    }
    fn get_body(&self) -> Body {
        let aabb = AABB::new(
            (
                self.position.0 - self.size.0 / 2.0,
                self.position.1 - self.size.1 / 2.0,
            ),
            (
                self.position.0 + self.size.0 / 2.0,
                self.position.1 + self.size.1 / 2.0,
            ),
        );
        return Body {
            aabb,
            body_type: BodyType::Dynamic,
        };
    }
    fn tick(&mut self, _delta_time: f32) {}
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerInputs {
    pub mov: (f32, f32),
    pub attack: bool,
}

impl PlayerInputs {
    pub fn new() -> PlayerInputs {
        PlayerInputs {
            mov: (0.0, 0.0),
            attack: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConfiguration {
    pub player_id: u32,
    pub name: String,
    pub avatar: u8,
    pub pic: String,
    pub color: String,
    pub sub: String,
    pub initial_position: (f32, f32),
}
