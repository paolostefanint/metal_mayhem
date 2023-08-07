use super::collisions::{Body, BodyType, AABB};
use super::world::GameWorld;

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub attack: f32,
    pub defense: f32,
    pub max_speed: f32,
    pub health: f32,
    pub size: (f32, f32),
    pub position: (f32, f32),
    pub body: Body,
    pub input: PlayerInputs,
}

impl Player {
    pub fn tick(&mut self, delta_time: f32) {
        let (x, y) = self.position;
        let (mov_x, mov_y) = self.input.mov;
        let speed = self.max_speed;

        self.position = (
            x + mov_x * speed * delta_time,
            y + mov_y * speed * delta_time,
        );

        // println!("player position tick: {:?}", self.position);
    }
}

#[derive(Debug)]
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
    pub player_id: u8,
    pub initial_position: (f32, f32),
    pub size: (f32, f32),
    pub speed: f32,
}

pub fn create_player(player_configuration: &PlayerConfiguration, world: &mut GameWorld) -> Player {
    let aabb = AABB::new(
        (
            player_configuration.initial_position.0 - player_configuration.size.0 / 2.0,
            player_configuration.initial_position.1 - player_configuration.size.1 / 2.0,
        ),
        (
            player_configuration.initial_position.0 + player_configuration.size.0 / 2.0,
            player_configuration.initial_position.1 + player_configuration.size.1 / 2.0,
        ),
    );

    let player = Player {
        id: player_configuration.player_id,
        attack: 0.0,
        defense: 0.0,
        max_speed: player_configuration.speed,
        health: 0.0,
        size: player_configuration.size,
        position: player_configuration.initial_position,
        body: Body::new(aabb, BodyType::Static),
        input: PlayerInputs::new(),
    };
    return player;
}
