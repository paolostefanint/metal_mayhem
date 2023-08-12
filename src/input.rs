use super::game::Game;
use super::player::Player;
use super::world::GameWorld;
use futures_util::SinkExt;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_websockets::{Error, Message, ServerBuilder};

pub async fn start_listening_websocket(game_arc: Arc<Mutex<Game>>) -> Result<(), Error> {
    println!("Start input thread begin");
    let listener = TcpListener::bind("0.0.0.0:40020").await?;

    let game = game_arc.clone();
    tokio::spawn(async move {
        println!("Start input listener...");
        while let Ok((stream, _)) = listener.accept().await {
            println!(
                "Input listener accepted a connection from {:?}",
                stream.peer_addr().unwrap()
            );
            let mut ws_stream = ServerBuilder::new().accept(stream).await?;

            let game = game.clone();
            tokio::spawn(async move {
                while let Some(msg) = ws_stream.next().await {
                    match msg {
                        Ok(msg) => {
                            // println!("Received a message from a client: {:?}", msg);
                            // println!("Received a message from a client: {}", msg);
                            // ws_stream
                            //     .send(Message::text(String::from("Hello from server")))
                            //     .await;
                            // return;

                            if msg.is_text() {
                                // id|movement|attack
                                // id|1.0:1.0|0
                                // $id|x:y|attack$id|x:y|attack$

                                let mut message = msg.as_text().unwrap();

                                match message {
                                    "ping" => {
                                        let pong_message = Message::text(String::from("pong"));
                                        match ws_stream.send(pong_message).await {
                                            Ok(_) => {}
                                            Err(e) => {
                                                println!("Error sending message: {}", e);
                                            }
                                        }
                                        continue;
                                    }
                                    _ => {
                                        // println!("Received a message from a client: {}", message);
                                        message = message.trim();
                                        let message = message.split("|").collect::<Vec<&str>>();
                                        // let id = message[0];
                                        // println!("id: {}", id);
                                        let movement = message[1]
                                            .split(":")
                                            .map(|x| x.parse::<f32>().unwrap())
                                            .collect::<Vec<f32>>();
                                        // println!("movement: {:?}", movement);
                                        let attack = message[2];
                                        // println!("attack: {}", attack);

                                        // println!("id: {}", id);
                                        // println!("movement: {:?}", movement);
                                        // println!("attack: {}", attack);

                                        let mut game = game.lock().unwrap();

                                        let world = game.get_world_mut();
                                        let mut players = world.get_players_mut();
                                        let first_player: &mut Player = players.get_mut(0).unwrap();

                                        first_player.input.mov = (movement[0], movement[1]);
                                        first_player.input.attack = attack == "1";
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error receiving message: {}", e);
                        }
                    }
                }
            });
        }
        Ok::<_, Error>(())
    });
    Ok::<_, Error>(())
}
