extern crate sys;

use std::sync::{Arc, Mutex};
mod connections;
mod input;
mod physics;
mod player;
mod render;
mod world;

use connections::start_client_connections;
use input::start_input;
use physics::{setup_world, start_physics_engine};
use player::{create_player, Player, PlayerConfiguration};
use render::start_render;

const WORLD_SIZE: (f32, f32) = (20.0, 20.0);

#[tokio::main]
async fn main() {
    let (mut world, mut rigid_body_set, mut collider_set) = setup_world(WORLD_SIZE);

    // Add player
    let player1_conf = PlayerConfiguration {
        player_id: 1,
        initial_position: (5.0, 10.0),
        size: 1.0,
    };
    let player1: Player = create_player(
        &player1_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    let player2_conf = PlayerConfiguration {
        player_id: 2,
        initial_position: (10.0, 5.0),
        size: 1.0,
    };
    let player2: Player = create_player(
        &player2_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    let player3_conf = PlayerConfiguration {
        player_id: 3,
        initial_position: (15.0, 5.0),
        size: 1.0,
    };
    let player3: Player = create_player(
        &player3_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    let player4_conf = PlayerConfiguration {
        player_id: 4,
        initial_position: (15.0, 5.0),
        size: 1.0,
    };
    let player4: Player = create_player(
        &player4_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    let player5_conf = PlayerConfiguration {
        player_id: 5,
        initial_position: (15.0, 5.0),
        size: 1.0,
    };
    let player5: Player = create_player(
        &player5_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    let player6_conf = PlayerConfiguration {
        player_id: 6,
        initial_position: (15.0, 5.0),
        size: 1.0,
    };
    let player6: Player = create_player(
        &player6_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    let player7_conf = PlayerConfiguration {
        player_id: 7,
        initial_position: (15.0, 5.0),
        size: 1.0,
    };
    let player7: Player = create_player(
        &player7_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    let player8_conf = PlayerConfiguration {
        player_id: 8,
        initial_position: (17.0, 5.0),
        size: 1.0,
    };
    let player8: Player = create_player(
        &player8_conf,
        &mut world,
        &mut rigid_body_set,
        &mut collider_set,
    );

    world.players.push(player1);
    world.players.push(player2);
    world.players.push(player3);
    world.players.push(player4);
    world.players.push(player5);
    world.players.push(player6);
    world.players.push(player7);
    world.players.push(player8);

    let world_arc = Arc::new(Mutex::new(world));
    let rigid_body_set_arc = Arc::new(Mutex::new(rigid_body_set));
    let collider_set_arc = Arc::new(Mutex::new(collider_set));

    // PHYSICS
    //
    let world = world_arc.clone();
    let rigid_body_set = rigid_body_set_arc.clone();
    let collider_set = collider_set_arc.clone();

    let physics_thread = start_physics_engine(world, rigid_body_set, collider_set);

    // RENDER
    //
    let world = world_arc.clone();
    let rigid_body_set = rigid_body_set_arc.clone();

    // let render_thread = start_render(world, rigid_body_set);

    // INPUT
    //
    let world = world_arc.clone();
    let rigid_body_set = rigid_body_set_arc.clone();

    let input_thread = start_input(world, rigid_body_set);

    // WEBSOCKET CONNECTIONS
    //
    let world = world_arc.clone();
    let rigid_body_set = rigid_body_set_arc.clone();

    start_client_connections(world, rigid_body_set).await;

    // render_thread.join().unwrap();
    physics_thread.join().unwrap();
    input_thread.join().unwrap();
}
