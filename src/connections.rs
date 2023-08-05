use super::world::{GameState, GameWorld};
use futures_util::SinkExt;
use rapier2d::prelude::*;
use serde_json;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::{self, Duration};
use tokio_websockets::{Error, Message, ServerBuilder};

pub async fn start_client_connections(
    world: Arc<Mutex<GameWorld>>,
    rigid_body_set: Arc<Mutex<RigidBodySet>>,
) -> Result<(), Error> {
    println!("Start client connections thread begin");
    let listener = TcpListener::bind("127.0.0.1:42000").await?;

    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let world = world.clone();
            let rigid_body_set = rigid_body_set.clone();

            let mut ws_stream = ServerBuilder::new().accept(stream).await?;

            tokio::spawn(async move {
                let interval_duration = Duration::from_millis(1000 / 16);
                let mut interval = time::interval(interval_duration);

                loop {
                    interval.tick().await;
                    let world = world.clone();
                    let rigid_body_set = rigid_body_set.clone();

                    let message = Message::text(get_game_state(world, rigid_body_set));

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

fn get_game_state(
    world: Arc<Mutex<GameWorld>>,
    rigid_body_set: Arc<Mutex<RigidBodySet>>,
) -> String {
    {
        let world = match world.lock() {
            Ok(world) => world,
            Err(poisoned) => {
                println!("poisoned world mutex on send game thread");
                poisoned.into_inner()
            }
        };
        let rigid_body_set = match rigid_body_set.lock() {
            Ok(rigid_body_set) => rigid_body_set,
            Err(poisoned) => {
                println!("poisoned rigid_body_set mutex on send game state thread");
                poisoned.into_inner()
            }
        };

        let game_state = GameState {
            current_time: 0.0,
            current_state: String::from("TEST MESSAGE"),
            players: world.get_players_state(&rigid_body_set),
        };
        let game_state_json = serde_json::to_string(&game_state).unwrap();

        return game_state_json;
    }
}
