use crate::world::GameEntity;

use super::collisions::{Body, BodyType, AABB};
use super::world::GameWorld;

pub enum EntityType {
    Player,
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
}

impl Player {}

impl GameEntity for Player {
    fn as_player_mut(&mut self) -> &mut Player {
        return self;
    }
    fn as_player(&self) -> &Player {
        return self;
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

        // self.position = (
        //     x + mov_x * speed * delta_time,
        //     y + mov_y * speed * delta_time,
        // );
        // self.body.aabb.min = (
        //     self.position.0 - self.size.0 / 2.0,
        //     self.position.1 - self.size.1 / 2.0,
        // );
        // self.body.aabb.max = (
        //     self.position.0 + self.size.0 / 2.0,
        //     self.position.1 + self.size.1 / 2.0,
        // );

        // println!("player position tick: {:?}", self.position);
    }
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
        input: PlayerInputs::new(),
    };
    return player;
}
