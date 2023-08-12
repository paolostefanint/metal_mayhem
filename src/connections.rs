use super::game::Game;
use super::world::{GameState, GameWorld};
use futures_util::SinkExt;
use serde_json;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::{self, Duration};
use tokio_websockets::{Error, Message, ServerBuilder};

pub async fn start_client_connections(game: Arc<Mutex<Game>>) -> Result<(), Error> {
    println!("Start client connections thread begin");
    let listener = TcpListener::bind("127.0.0.1:42000").await?;

    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let game = game.clone();
            println!(
                "New sender connection accepted from {:?}",
                stream.peer_addr().unwrap()
            );

            let mut ws_stream = ServerBuilder::new().accept(stream).await?;

            tokio::spawn(async move {
                let interval_duration = Duration::from_millis(1000 / 16);
                let mut interval = time::interval(interval_duration);

                loop {
                    interval.tick().await;
                    let game = game.clone();

                    let message = Message::text(get_game_state(game));

                    match ws_stream.send(message).await {
                        // Ok(_) => println!("Message sent"),
                        Ok(_) => (),
                        Err(e) => {
                            println!("Error sending message: {}", e);
                            break;
                        }
                    };
                }
            });
        }

        Ok::<_, Error>(())
    });

    Ok::<_, Error>(())
}

fn get_game_state(game: Arc<Mutex<Game>>) -> String {
    {
        let game = game.lock().unwrap();
        let world = game.get_world();

        let game_state = GameState {
            current_time: 0.0,
            current_state: game.phase,
            players: world.get_players_state(),
        };
        let game_state_json = serde_json::to_string(&game_state).unwrap();

        return game_state_json;
    }
}
