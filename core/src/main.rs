mod collisions;
mod connections;
mod game;
mod input;
mod map;
mod player;
mod render;
mod world;

use connections::start_client_connections;
use game::Game;
use input::start_listening_websocket;
use player::{Player, PlayerConfiguration};

use std::sync::{Arc, Mutex};
use std::time::Duration;


const WORLD_SIZE: (f32, f32) = (20.0, 20.0);

#[tokio::main]
async fn main() {
    let mut game = Game::new(WORLD_SIZE);

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

    let player3_conf = PlayerConfiguration {
        player_id: 3,
        initial_position: (20.0, 5.0),
        size: (1.0, 0.5),
        speed: 3.0,
    };
    let player3: Player = Player::new(&player3_conf);

    game.add_player(player1);
    game.add_player(player2);
    game.add_player(player3);

    let game_arc = Arc::new(Mutex::new(game));

    // INPUT
    //
    let game = game_arc.clone();
    let _ = start_listening_websocket(game).await;

    // WEBSOCKET CONNECTIONS
    //
    let game = game_arc.clone();
    let _c = start_client_connections(game).await;

    loop {
        {
            let mut game = game_arc.lock().unwrap();
            game.update();
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
