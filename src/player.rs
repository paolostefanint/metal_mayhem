use rapier2d::prelude::*;

use super::world::GameWorld;

#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub attack: f32,
    pub defense: f32,
    pub max_speed: f32,
    pub health: f32,
    pub size: f32,
    pub input: PlayerInputs,
    pub body_handle: RigidBodyHandle,
}

#[derive(Debug)]
pub struct PlayerInputs {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerInputs {
    pub fn new() -> PlayerInputs {
        PlayerInputs {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

pub struct PlayerConfiguration {
    pub player_id: u8,
    pub initial_position: (f32, f32),
    pub size: f32,
}

fn create_player_physics(player_size: f32, player_position: (f32, f32)) -> (RigidBody, Collider) {
    let player_body = RigidBodyBuilder::dynamic()
        .translation(vector![player_position.0, player_position.1])
        .build();
    let player_collider = ColliderBuilder::ball(player_size).restitution(1.0).build();

    return (player_body, player_collider);
}

fn add_player_to_world(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    player_body: RigidBody,
    player_collider: Collider,
) -> RigidBodyHandle {
    let player_body_handle = rigid_body_set.insert(player_body);
    collider_set.insert_with_parent(player_collider, player_body_handle, rigid_body_set);
    return player_body_handle;
}

fn create_game_player(
    player_configuration: &PlayerConfiguration,
    player_body_handle: RigidBodyHandle,
) -> Player {
    let player = Player {
        id: player_configuration.player_id,
        attack: 0.0,
        defense: 0.0,
        max_speed: 0.0,
        health: 0.0,
        size: 0.0,
        input: PlayerInputs::new(),
        body_handle: player_body_handle,
    };
    return player;
}

pub fn create_player(
    player_configuration: &PlayerConfiguration,
    world: &mut GameWorld,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
) -> Player {
    let (player_body, player_collider) = create_player_physics(
        player_configuration.size,
        player_configuration.initial_position,
    );
    let player_body_handle =
        add_player_to_world(rigid_body_set, collider_set, player_body, player_collider);
    let player = create_game_player(player_configuration, player_body_handle);
    return player;
}
