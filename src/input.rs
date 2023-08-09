use super::player::Player;
use super::world::GameWorld;
use futures_util::SinkExt;
use std::mem::take;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_websockets::{Error, Message, ServerBuilder};

pub async fn start_listening_websocket(world_arc: Arc<Mutex<GameWorld>>) -> Result<(), Error> {
    println!("Start input thread begin");
    let listener = TcpListener::bind("127.0.0.1:40020").await?;

    let world = world_arc.clone();
    tokio::spawn(async move {
        println!("Start input thread");
        while let Ok((stream, _)) = listener.accept().await {
            let mut ws_stream = ServerBuilder::new().accept(stream).await?;

            let world = world.clone();
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

                                let mut message = msg.as_text().unwrap();
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

                                let mut world = world.lock().unwrap();

                                let mut players = world.get_players_mut();
                                let first_player: &mut Player = players.get_mut(0).unwrap();

                                first_player.input.mov = (movement[0], movement[1]);
                                first_player.input.attack = attack == "1";

                                // let mut p = take(*first_player);

                                // println!("movement: {:?}", movement);

                                // p.input.mov = (movement[0], movement[1]);
                                // p.input.attack = attack == "1";
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
