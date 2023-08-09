use super::world::Direction;
use crate::world::GameEntity;

use super::collisions::{Body, BodyType, AABB};
use super::world::GameWorld;

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
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Player {
    pub id: u32,
    pub stats: PlayerStats,
    pub size: (f32, f32),
    pub position: (f32, f32),
    pub input: PlayerInputs,
    pub direction: Direction,
}

impl Player {
    pub fn take_damage(&mut self, damage: f32) {
        // go to max(0, health - damage)
        self.stats.health = (self.stats.health - damage).max(0.0);
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
    fn tick(&mut self, delta_time: f32) {}
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

pub struct PlayerConfiguration {
    pub player_id: u32,
    pub initial_position: (f32, f32),
    pub size: (f32, f32),
    pub speed: f32,
}

pub fn create_player(player_configuration: &PlayerConfiguration, world: &mut GameWorld) -> Player {
    // let aabb = AABB::new(
    //     (
    //         player_configuration.initial_position.0 - player_configuration.size.0 / 2.0,
    //         player_configuration.initial_position.1 - player_configuration.size.1 / 2.0,
    //     ),
    //     (
    //         player_configuration.initial_position.0 + player_configuration.size.0 / 2.0,
    //         player_configuration.initial_position.1 + player_configuration.size.1 / 2.0,
    //     ),
    // );

    let player = Player {
        id: player_configuration.player_id,
        stats: PlayerStats {
            attack: 0.0,
            defense: 0.0,
            max_speed: player_configuration.speed,
            health: 0.0,
        },
        size: player_configuration.size,
        position: player_configuration.initial_position,
        direction: Direction::R,
        input: PlayerInputs::new(),
    };
    return player;
}
