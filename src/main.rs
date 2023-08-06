mod connections;
mod input;
mod physics;
mod player;
mod render;
mod world;

use connections::start_client_connections;
use input::start_listening_websocket;
use physics::setup_world;
use player::{create_player, Player, PlayerConfiguration};
use render::render;
use render::start_render;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use world::start_world_loop;

const WORLD_SIZE: (f32, f32) = (20.0, 20.0);

#[tokio::main]
async fn main() {
    let mut world = setup_world(WORLD_SIZE);

    // Add player
    let player1_conf = PlayerConfiguration {
        player_id: 1,
        initial_position: (5.0, 10.0),
        size: 1.0,
        speed: 7.0,
    };
    let player1: Player = create_player(&player1_conf, &mut world);

    // let player2_conf = PlayerConfiguration {
    //     player_id: 2,
    //     initial_position: (10.0, 5.0),
    //     size: 1.0,
    // };
    // let player2: Player = create_player(&player2_conf, &mut world);

    // let player3_conf = PlayerConfiguration {
    //     player_id: 3,
    //     initial_position: (15.0, 5.0),
    //     size: 1.0,
    // };
    // let player3: Player = create_player(&player3_conf, &mut world);

    // let player4_conf = PlayerConfiguration {
    //     player_id: 4,
    //     initial_position: (15.0, 5.0),
    //     size: 1.0,
    // };
    // let player4: Player = create_player(&player4_conf, &mut world);

    // let player5_conf = PlayerConfiguration {
    //     player_id: 5,
    //     initial_position: (20.0, 10.0),
    //     size: 1.0,
    // };
    // let player5: Player = create_player(&player5_conf, &mut world);

    // let player6_conf = PlayerConfiguration {
    //     player_id: 6,
    //     initial_position: (15.0, 5.0),
    //     size: 1.0,
    // };
    // let player6: Player = create_player(&player6_conf, &mut world);

    // let player7_conf = PlayerConfiguration {
    //     player_id: 7,
    //     initial_position: (15.0, 5.0),
    //     size: 1.0,
    // };
    // let player7: Player = create_player(&player7_conf, &mut world);

    // let player8_conf = PlayerConfiguration {
    //     player_id: 8,
    //     initial_position: (17.0, 5.0),
    //     size: 1.0,
    // };
    // let player8: Player = create_player(&player8_conf, &mut world);

    world.players.push(player1);
    // world.players.push(player2);
    // world.players.push(player3);
    // world.players.push(player4);
    // world.players.push(player5);
    // world.players.push(player6);
    // world.players.push(player7);
    // world.players.push(player8);

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

            let last_time = world.current_time;
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(last_time).as_secs_f32();
            // println!("delta_time: {}", delta_time);
            world.current_time = current_time;

            for player in world.players.iter_mut() {
                player.tick(delta_time);
            }
        }
        // world.handle_inputs(&mut rigid_body_set);
        {
            let world = world_arc.lock().unwrap();
            println!("world");
            render(&world);
        }

        // thread::sleep(Duration::from_millis(100));
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
