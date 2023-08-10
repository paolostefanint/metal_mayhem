mod collisions;
mod connections;
mod input;
mod player;
mod render;
mod world;

use connections::start_client_connections;
use input::start_listening_websocket;
use player::{Player, PlayerConfiguration};
use render::render;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use world::GameWorld;

const WORLD_SIZE: (f32, f32) = (20.0, 20.0);

#[tokio::main]
async fn main() {
    let mut world = GameWorld::new(WORLD_SIZE);

    // Add player
    let player1_conf = PlayerConfiguration {
        player_id: 1,
        initial_position: (5.0, 10.0),
        size: (1.0, 0.5),
        speed: 3.0,
    };
    let player1: Player = Player::new(&player1_conf);

    let player2_conf = PlayerConfiguration {
        player_id: 2,
        initial_position: (10.0, 5.0),
        size: (1.0, 0.5),
        speed: 3.0,
    };
    let player2: Player = Player::new(&player2_conf);

    world.add_entity(Box::new(player1));
    world.add_entity(Box::new(player2));

    let world_arc = Arc::new(Mutex::new(world));

    // INPUT
    //
    let world = world_arc.clone();
    let _ = start_listening_websocket(world).await;

    // WEBSOCKET CONNECTIONS
    //
    let world = world_arc.clone();
    let _c = start_client_connections(world).await;

    loop {
        {
            let mut world = world_arc.lock().unwrap();
            world.update();
        }
        // {
        //     let world = world_arc.lock().unwrap();
        //     println!("world");
        //     render(&world);
        // }

        // thread::sleep(Duration::from_millis(100));
        tokio::time::sleep(Duration::from_millis(1000 / 20)).await;
    }
}
