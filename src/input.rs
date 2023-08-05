use super::world::GameWorld;
use futures_util::SinkExt;
use rapier2d::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_websockets::{Error, Message, ServerBuilder};

pub async fn start_listening_websocket(
    world: Arc<Mutex<GameWorld>>,
    rigid_body_set: Arc<Mutex<RigidBodySet>>,
) -> Result<(), Error> {
    println!("Start input thread begin");
    let listener = TcpListener::bind("127.0.0.1:40020").await?;

    tokio::spawn(async move {
        println!("Start input thread");
        while let Ok((stream, _)) = listener.accept().await {
            let mut ws_stream = ServerBuilder::new().accept(stream).await?;

            tokio::spawn(async move {
                while let Some(msg) = ws_stream.next().await {
                    match msg {
                        Ok(msg) => {
                            println!("Received a message from a client: {:?}", msg);
                            // println!("Received a message from a client: {}", msg);
                            // ws_stream
                            //     .send(Message::text(String::from("Hello from server")))
                            //     .await;
                            // return;
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
