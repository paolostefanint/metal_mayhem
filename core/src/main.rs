mod collisions;
mod connections;
mod game;
mod input;
mod map;
mod player;
mod render;
mod stage;
mod world;

use connections::start_client_connections;
use game::Game;
use input::{start_player_listening_websocket, start_server_listening_websocket};
use player::{Player, PlayerConfiguration};
use render::render;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const WORLD_SIZE: (f32, f32) = (20.0, 20.0);

#[derive(Debug)]
pub enum ServerCommand {
    Start,
    Stop,
    PlayerInput,
}

pub struct ServerCommandMessage {
    command: ServerCommand,
    data: String,
}

#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<ServerCommandMessage>, Receiver<ServerCommandMessage>) = mpsc::channel();

    // listening for server commands
    let _ = start_server_listening_websocket(tx).await;

    let game = Game::new(WORLD_SIZE);
    let game_arc = Arc::new(Mutex::new(game));

    let game_clone = game_arc.clone();
    let _ = start_client_connections(game_clone).await;

    loop {
        let received = rx.recv().unwrap();
        println!("Received: {:?}", received.command);

        match received.command {
            ServerCommand::Start => start_game(game_arc.clone()),
            ServerCommand::Stop => stop_game(game_arc.clone()),
            ServerCommand::PlayerInput => {
                println!("Player input");
            }
        }
    }
}

fn start_game(game: Arc<Mutex<Game>>) {
    println!("Starting game");
    // once the match should be starting, init the gamie

    // // Add player
    // let player1_conf = PlayerConfiguration {
    //     player_id: 1,
    //     initial_position: (5.0, 10.0),
    //     size: (1.0, 0.5),
    //     speed: 3.0,
    // };
    // let player1: Player = Player::new(&player1_conf);

    // let player2_conf = PlayerConfiguration {
    //     player_id: 2,
    //     initial_position: (10.0, 5.0),
    //     size: (1.0, 0.5),
    //     speed: 3.0,
    // };
    // let player2: Player = Player::new(&player2_conf);

    // let player3_conf = PlayerConfiguration {
    //     player_id: 3,
    //     initial_position: (20.0, 5.0),
    //     size: (1.0, 0.5),
    //     speed: 3.0,
    // };
    // let player3: Player = Player::new(&player3_conf);

    // game.add_player(player1);
    // game.add_player(player2);
    // game.add_player(player3);

    // INPUT
    //
    //
    // let game = game_arc.clone();
    // let _ = start_player_listening_websocket(game).await;

    // WEBSOCKET CONNECTIONS
    //
    let game = game.clone();
    let mut game_ref = game.lock().unwrap();
    game_ref.start();

    let game = game.clone();

    tokio::spawn(async move {
        loop {
            {
                let mut game = game.lock().unwrap();
                if !game.is_running() {
                    break;
                }
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
    });

    // let _c = start_client_connections(game).await;
}

fn stop_game(game: Arc<Mutex<Game>>) {
    println!("Stopping game");
    let mut game_ref = game.lock().unwrap();
    game_ref.phase = game::GamePhase::WaitingForPlayers;
}
