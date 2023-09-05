mod collisions;
mod config;
mod game;
mod input;
mod map;
mod output;
mod player;
mod render;
mod world;

use config::{init_config, CONFIG};
use game::Game;
use input::start_server_listening_websocket;
use output::start_client_connections;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    init_config();
    let config = CONFIG.get().unwrap();
    let game = Some(Game::new());

    let (tx, rx): (Sender<ServerCommandMessage>, Receiver<ServerCommandMessage>) = mpsc::channel();

    // listening for server commands
    let _ = start_server_listening_websocket(tx).await;

    let game_arc = Arc::new(Mutex::new(game.unwrap()));

    {
        // game setup
        let game_clone = game_arc.clone();
        let mut game = game_clone.lock().unwrap();
        game.setup(config.world_size);
    }

    let game_clone = game_arc.clone();
    let _ = start_client_connections(game_clone).await;

    loop {
        let received = match rx.recv() {
            Ok(received) => received,
            Err(_) => {
                continue;
            }
        };

        match received.command {
            ServerCommand::Start => start_game(game_arc.clone(), received.data),
            ServerCommand::Stop => stop_game(game_arc.clone()),
            ServerCommand::PlayerInput => handle_input(game_arc.clone(), &received.data),
        }
    }
}

fn handle_input(game: Arc<Mutex<Game>>, data: &String) {
    let splitted_data = data.split("|").collect::<Vec<&str>>();
    let player_id = splitted_data.get(0).unwrap().parse::<u32>().unwrap();
    let input = splitted_data.get(1).unwrap();

    // remove first and last character from input
    let input = &input[1..input.len() - 1];
    let input = input.split(",").collect::<Vec<&str>>();

    let movement = (
        input.get(0).unwrap().parse::<f32>().unwrap(),
        input.get(1).unwrap().parse::<f32>().unwrap(),
    );
    let attack_string = input.get(2).unwrap();
    let attack = attack_string.parse::<u32>().unwrap();
    let attack = match attack {
        0 => false,
        1 => true,
        _ => false,
    };

    let mut game_ref = game.lock().unwrap();
    if game_ref.is_running() {
        // println!("PlayerId: {:?}, input: {:?}", player_id, input);
        game_ref.handle_input(player_id, movement, attack);
    }

    // let mut game_ref = game.lock().unwrap();
    // game_ref.handle_input(data);
}

fn start_game(game: Arc<Mutex<Game>>, data: String) {
    println!("Incoming start data: {:?}", data);
    let players_configs = serde_json::from_str::<Vec<player::PlayerConfiguration>>(&data).unwrap();

    println!("Starting game with {:?} players", players_configs.len());
    println!(
        "Players: {:?}",
        players_configs
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
    );

    // WEBSOCKET CONNECTIONS
    //
    let game = game.clone();
    let mut game_ref = game.lock().unwrap();
    game_ref.start();

    // add players
    for player_conf in players_configs {
        let player = player::Player::new(&player_conf);
        game_ref.add_player(player);
    }

    let game = game.clone();

    tokio::spawn(async move {
        loop {
            {
                let mut game = game.lock().unwrap();
                if !game.is_running() {
                    println!("Game stopped");
                    return;
                }
                game.update();
            }
            tokio::time::sleep(Duration::from_millis(1000 / 20)).await;
        }
    });
}

fn stop_game(game: Arc<Mutex<Game>>) {
    println!("Stopping game");
    let mut game_ref = game.lock().unwrap();
    game_ref.phase = game::GamePhase::WaitingForPlayers;
}
